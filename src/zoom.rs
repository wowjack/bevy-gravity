use bevy::{prelude::*, input::mouse::{MouseWheel, MouseScrollUnit}};
use bevy_math::DVec2;

use crate::{ui::SIDE_PANEL_WIDTH, CameraState, MassiveObject};


/// Event denoting that the view scale has changed.
/// Is this even required if objects are redrawn every frame?
#[derive(Event)]
pub struct ScaleChangeEvent(f32);


pub fn mouse_zoom(
    mut camera_query: Query<(&mut CameraState, &Camera, &GlobalTransform)>,
    mut scroll_events: EventReader<MouseWheel>,
    primary_window: Query<&Window>,
    mut event_writer: EventWriter<ScaleChangeEvent>,
) {
    let pixels_per_line = 100.; // Maybe make configurable?
    let scroll = scroll_events
        .read()
        .map(|ev| match ev.unit {
            MouseScrollUnit::Pixel => ev.y,
            MouseScrollUnit::Line => ev.y * pixels_per_line,
        })
        .sum::<f32>();

    if scroll == 0. {
        return;
    }

    let (mut state, camera, gtrans) = camera_query.single_mut();    
    let window = primary_window.single();
    
    let Some(cursor_pos) = window.cursor_position() else { return };
    let Some(unscaled_cursor_pos) = camera.viewport_to_world_2d(gtrans, cursor_pos) else { return };
    
    let old_scale = state.scale;
    state.scale = state.scale * (1. + scroll * 0.001);

    // Move the camera position to normalize the projection window
    let position_difference = (unscaled_cursor_pos / state.scale) - (unscaled_cursor_pos / old_scale);
    state.position -= position_difference.as_dvec2();
    
    event_writer.send(ScaleChangeEvent(state.scale));
}




/// TEMPORARY, JUST FOR DEV TESTING
pub fn process_scale_change(mut scale_change_events: EventReader<ScaleChangeEvent>, mut object_query: Query<(&mut Transform, &MassiveObject)>, camera_query: Query<&CameraState>) {
    if scale_change_events.is_empty() { return }

    let camera = camera_query.single();
    
    for e in scale_change_events.read() {
        for (mut trans, object) in object_query.iter_mut() {
            trans.translation = (object.position - camera.position).as_vec2().extend(0.) * e.0;
            trans.scale = Vec3::new(e.0, e.0, 1.);
        }
    }
}