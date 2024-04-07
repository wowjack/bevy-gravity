use bevy::prelude::*;
use crate::ui::ToDraw;
use super::{physics_future::{ObjectFuture, PhysicsFuture}, select::SelectedObjects};



pub fn spawn_path_prediction(
    physics_future: Res<PhysicsFuture>,
    selected_objects: Res<SelectedObjects>,
    mut gizmos: Gizmos,
    to_draw: Res<ToDraw>
) {
    if to_draw.future_path == false { return }
    let Some(focused) = selected_objects.focused else { return };
    let point_list = physics_future.future.lock().unwrap().get(&focused).unwrap_or(&ObjectFuture::default()).get_future_linestrip(to_draw.prediction_buffer_len, to_draw.prediction_line_segment_size);
    gizmos.linestrip_2d(point_list.into_iter(), Color::BLACK);
}