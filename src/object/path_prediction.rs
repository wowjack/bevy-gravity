use std::collections::VecDeque;

use bevy::prelude::*;

use crate::{MainCamera, ui::ToDraw};

use super::{physics_future::PhysicsFuture, object::MassiveObject, select::SelectedObjects};



// use the angle between previous line segments to improve rendering speed and distance displayed
// Some advanced method of processing the future to only receive 
pub fn spawn_path_prediction(
    physics_future: Res<PhysicsFuture>,
    selected_objects: Res<SelectedObjects>,
    mut gizmos: Gizmos,
    projection_query: Query<&OrthographicProjection, With<MainCamera>>,
    to_draw: Res<ToDraw>
) {
    if to_draw.future_path == false { return }

    let Some(focused) = selected_objects.focused else { return };

    let Ok(projection) = projection_query.get_single() else { return };
    let mut previously_recorded_point = Vec2::INFINITY;
    let mut point_list: Vec<Vec2> = vec![];
    for state in physics_future.future.lock().unwrap().get(&focused).unwrap_or(&VecDeque::new()).iter() {
        if state.position.distance_squared(previously_recorded_point) < to_draw.prediction_line_segment_size.powi(2) { continue }
        point_list.push(state.position);
        previously_recorded_point = state.position;

        if point_list.len() > to_draw.prediction_buffer_len { break }
    }

    gizmos.linestrip_2d(point_list.into_iter(), Color::BLACK);
}