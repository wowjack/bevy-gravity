use std::{collections::VecDeque, sync::{Arc, Mutex}, thread::{self, JoinHandle}};

use bevy::{ecs::system, gizmos::gizmos, math::DVec2, prelude::*};
use bevy_egui::egui::epaint::tessellator::path;
use itertools::Itertools;

use crate::{gravity_system_tree::system_manager::GravitySystemManager, pseudo_camera::camera::CameraState, visual_object::{FuturePathLineConfig, VisualObjectData}};



/// Spawn a background thread to calculate the future path of the selected object
#[derive(Component)]
pub struct PathCalculator {
    path: Arc<Mutex<VecDeque<DVec2>>>,
    _thread_handle: JoinHandle<()>,
}
impl PathCalculator {
    /// Takes in a system manager and an entity for which to clone while retaining the provided entity. 
    pub fn new(system_manager: &GravitySystemManager, entity: Entity) -> Self {
        let mut new_system = system_manager.retain_clone(entity).unwrap();
        let path_queue = Arc::new(Mutex::new(VecDeque::new()));
        let path_queue_copy = path_queue.clone();
        let handle = thread::spawn(move || {
            let mut back_is_removable = false;
            let body = unsafe { new_system.body_store.dynamic_bodies.get_unchecked(0) };
            path_queue.lock().unwrap().push_back(body.get_previous_absolute_position());
            loop {
                let mut queue = path_queue.lock().unwrap();
                if queue.len() < 500_000 {
                    new_system.step();
                    let body = unsafe { new_system.body_store.dynamic_bodies.get_unchecked(0) };
                    let new_position = body.get_previous_absolute_position();
                    if back_is_removable { queue.pop_back(); }
                    if let Some((last, second_last)) = queue.iter().rev().next_tuple() {
                        let p1: DVec2 = *last - *second_last;
                        let p2 = new_position - *last;
                        if p1.angle_between(p2).abs() < std::f64::consts::TAU/300. {
                            queue.push_back(new_position);
                            back_is_removable = true;
                            continue;
                        }
                    }
                    
                    back_is_removable = false;
                    queue.push_back(new_position);
                } else {
                    return
                }
            }
        });

        Self {
            path: path_queue_copy,
            _thread_handle: handle,
        }
    }
}


/// Draw the future path to the screen
pub fn draw_path(
    camera_query: Query<&CameraState>,
    object_query: Query<&PathCalculator, With<VisualObjectData>>,
    mut gizmos: Gizmos<FuturePathLineConfig>,
) {
    if object_query.is_empty() { return }

    let Ok(camera) = camera_query.get_single() else { return };

    for path_calc in object_query.iter() {
        let path_queue = path_calc.path.lock().unwrap();
        let iter = path_queue.iter().map(|pos| camera.physics_to_world_pos(pos));
        gizmos.linestrip_2d(iter, Color::WHITE);
    }
}