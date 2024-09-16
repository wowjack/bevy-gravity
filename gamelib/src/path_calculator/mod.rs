use std::{collections::VecDeque, sync::{Arc, Mutex}, thread::{self, JoinHandle}};

use bevy::{ecs::system, gizmos::gizmos, math::DVec2, prelude::*};
use bevy_egui::egui::epaint::tessellator::path;
use itertools::Itertools;

use crate::{gravity_system_tree::{dynamic_body::DynamicBody, static_generator::StaticGenerator, system_manager::GravitySystemManager, system_tree::{DiscreteGravitySystemTime, GravitySystemTime}}, pseudo_camera::camera::CameraState, visual_object::{DrawOptions, FuturePathLineConfig, SimulationState, VisualObjectData}};



/// Spawn a background thread to calculate the future path of the selected object
#[derive(Component)]
pub struct PathCalculator {
    paths: Arc<Mutex<FuturePaths>>,
    _thread_handle: JoinHandle<()>,
}
impl PathCalculator {
    /// Takes in a system manager and an entity for which to clone while retaining the provided entity. 
    pub fn new(system_manager: &GravitySystemManager, entity: Entity) -> Self {
        let mut new_system = system_manager.retain_clone(entity).unwrap();
        let future_paths = Arc::new(Mutex::new(FuturePaths::default()));
        let future_paths_copy = future_paths.clone();

        let handle = thread::spawn(move || {
            loop {
                if Arc::strong_count(&future_paths) < 2 {
                    break;
                }
                let mut paths = future_paths.lock().unwrap();
                if !paths.should_stop() {
                    new_system.step();
                    let body = unsafe { new_system.body_store.dynamic_bodies.get_unchecked(0) };
                    paths.process_new_body_state(body, new_system.get_current_time());
                } else {
                    return
                }
            }
        });

        Self {
            paths: future_paths_copy,
            _thread_handle: handle,
        }
    }

    pub fn draw_path(&self, gizmos: &mut Gizmos<FuturePathLineConfig>, camera: &CameraState, time: GravitySystemTime, current_position: &DVec2) {
        let mut paths = self.paths.lock().unwrap();
        let dtime = unsafe { time.to_int_unchecked::<DiscreteGravitySystemTime>() + 1 };
        paths.drop_until_time(dtime);

        let first_position = paths.relative_path_segments
            .front()
            .map(|fp| 
                fp.path
                    .front()
                    .map(|(_, rp)| fp.generator.get_position(time)+*rp)
            ).flatten();
        if let Some(position) = first_position {
            gizmos.line_2d(camera.physics_to_world_pos(current_position), camera.physics_to_world_pos(&position), Color::WHITE);
        }

        paths.draw_relative_path(gizmos, camera, time);
    }
}



/// Struct to hold more detailed information about the future path of an object
#[derive(Default)]
pub struct FuturePaths {
    relative_path_segments: VecDeque<FuturePath>,
}
impl FuturePaths {
    pub fn process_new_body_state(&mut self, body: &DynamicBody, time: DiscreteGravitySystemTime) {
        let should_create_new_path = self.relative_path_segments
            .back()
            .map_or(true, |fp| 
                fp.generator.len() != body.get_parent_generator().len()
            );
        if should_create_new_path {
            self.relative_path_segments.push_back(FuturePath::from_body(body, time));
        } else {
            self.relative_path_segments.back_mut().unwrap().insert_new_position(body.get_previous_relative_position(), time);
        }
    }

    pub fn should_stop(&self) -> bool {
        self.relative_path_segments.iter().map(|rp| rp.len()).sum::<usize>() > 500_000
    }

    pub fn draw_relative_path(&self, gizmos: &mut Gizmos<FuturePathLineConfig>, camera: &CameraState, time: GravitySystemTime) {
        for path in &self.relative_path_segments {
            path.draw(time, gizmos, camera);
        }
    }

    pub fn drop_until_time(&mut self, time: DiscreteGravitySystemTime) {
        for fp in &mut self.relative_path_segments {
            fp.drop_until_time(time);
        }
        let index = self.relative_path_segments
            .iter()
            .find_position(|fp| fp.len() > 0)
            .map(|(i, _)| i)
            .unwrap_or(0);
        self.relative_path_segments.drain(0..index);
    }
}

#[derive(Default)]
pub struct FuturePath {
    path: VecDeque<(DiscreteGravitySystemTime, DVec2)>,
    generator: StaticGenerator,
    /// Whether or not the last position in the path queue should be retained or not. \
    /// This is useful for culling unnecessary points that fall on roughly the same line.
    last_is_removable: bool,
}
impl FuturePath {
    pub fn from_body(body: &DynamicBody, time: DiscreteGravitySystemTime) -> Self {
        let mut path = VecDeque::new();
        path.push_back((time, body.get_previous_relative_position()));
        Self {
            path,
            generator: body.get_parent_generator().clone(),
            last_is_removable: false
        }
    }

    pub fn len(&self) -> usize { self.path.len() }

    pub fn insert_new_position(&mut self, new_position: DVec2, time: DiscreteGravitySystemTime) {
        if let Some((last, second_last)) = self.path.iter().rev().next_tuple() {
            if (last.1 - second_last.1).angle_between(new_position - last.1).abs() < std::f64::consts::TAU/300. {
                if self.last_is_removable { self.path.pop_back(); }
                self.path.push_back((time, new_position));
                self.last_is_removable = true;
            } else {
                self.path.push_back((time, new_position));
                self.last_is_removable = false;
            }
        } else {
            self.path.push_back((time, new_position));
            self.last_is_removable = false;
        }
    }

    pub fn draw(&self, time: GravitySystemTime, gizmos: &mut Gizmos<FuturePathLineConfig>, camera: &CameraState) {
        let center_pos = self.generator.get_position(time);
        if center_pos == DVec2::ZERO {
            self.draw_without_center_position(gizmos, camera);
        } else {
            self.draw_with_center_position(center_pos, gizmos, camera);
        }
    }
    fn draw_with_center_position(&self, center_pos: DVec2, gizmos: &mut Gizmos<FuturePathLineConfig>, camera: &CameraState) {
        let iter = self.path.iter().map(|(_, p)| camera.physics_to_world_pos(&(center_pos+*p)));
        gizmos.linestrip_2d(iter, Color::WHITE);
    }
    fn draw_without_center_position(&self, gizmos: &mut Gizmos<FuturePathLineConfig>, camera: &CameraState) {
        let iter = self.path.iter().map(|(_, p)| camera.physics_to_world_pos(p));
        gizmos.linestrip_2d(iter, Color::WHITE);
    }

    fn drop_until_time(&mut self, time: DiscreteGravitySystemTime) {
        let index = self.path
            .iter()
            .find_position(|(t, _)| *t > time)
            .map(|(i, _)| i);
        if let Some(index) = index {
            self.path.drain(0..index);
        } else {
            self.path.clear();
        }
    }
}







/// Draw the future path to the screen
pub fn draw_path(
    camera_query: Query<&CameraState>,
    object_query: Query<(&PathCalculator, &VisualObjectData)>,
    mut gizmos: Gizmos<FuturePathLineConfig>,
    draw_options: Res<DrawOptions>,
    sim_state: Res<SimulationState>
) {
    if draw_options.draw_future_path == false { return }
    if object_query.is_empty() { return }

    let Ok(camera) = camera_query.get_single() else { return };

    for (path_calc, VisualObjectData { position, .. }) in object_query.iter() {

        path_calc.draw_path(&mut gizmos, camera, sim_state.current_time, position);
    }
}