use std::{collections::VecDeque, sync::{Arc, Mutex}, thread::{self, JoinHandle}};
use bevy::prelude::*;
use bevy_math::DVec2;

use crate::{physics::{self, FutureFrame, MassiveObject, PhysicsFuture}, pseudo_camera::CameraState};

use super::{DrawOptions, SelectedObjects, SimulationState};


/// Current just a marker type used for deciding which objects to draw a future path for.
/// 
/// Ideally this future path should only be created when it needs to.
/// First read The full buffer from the future.
/// Afterwards only read starting at whatever time the previous read ended at.
/// And reread the entire buffer is a change happens.
#[derive(Component, Clone, Copy)]
pub struct FuturePath;


pub fn draw_future_paths(
    object_query: Query<Entity, With<MassiveObject>>,
    camera_query: Query<&CameraState>,
    physics_future: Res<PhysicsFuture>,
    mut gizmos: Gizmos,
    draw_options: Res<DrawOptions>,
    selected_objects: Res<SelectedObjects>,
) {
    if draw_options.draw_future_path == false { return }
    let Some(focused) = selected_objects.focused else { return };
    let Ok(entity) = object_query.get(focused) else { return };
    let camera_state = camera_query.single();
    let future_map = physics_future.get_map();
    let map = future_map.map.read().unwrap();
    let Some(object_future) = map.get(&entity) else { return };
    let path = object_future.as_point_vec();
    gizmos.linestrip_2d(
        path.into_iter().map(|pos| camera_state.physics_to_world_pos(pos)),
        Color::GRAY
    );
}