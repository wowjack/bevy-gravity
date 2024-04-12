use std::{collections::VecDeque, sync::{Arc, Mutex}, thread::{self, JoinHandle}};
use bevy::prelude::*;
use bevy_math::DVec2;

use crate::{physics::{self, FutureFrame, MassiveObject, PhysicsFuture}, pseudo_camera::CameraState};

use super::SimulationState;


/// Current just a marker type used for deciding which objects to draw a future path for.
/// 
/// Ideally this future path should only be created when it needs to.
/// Only get the full buffer if the physics future has 
#[derive(Component, Clone, Copy)]
pub struct FuturePath;


pub fn draw_future_paths(
    object_query: Query<(Entity, &MassiveObject), With<FuturePath>>,
    camera_query: Query<&CameraState>,
    physics_future: Res<PhysicsFuture>,
    mut gizmos: Gizmos,
    sim_state: Res<SimulationState>,
) {
    let camera_state = camera_query.single();
    let future_map = physics_future.get_map();
    let map = future_map.map.read().unwrap();
    for (entity, object) in &object_query {
        let Some(object_future) = map.get(&entity) else { continue };
        let path = object_future.as_point_vec();
        gizmos.linestrip_2d(
            path.into_iter().map(|pos| camera_state.physics_to_world_pos(pos)),
            Color::GRAY
        );
    }
}