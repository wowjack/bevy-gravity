use bevy::{prelude::*, input::mouse::{MouseWheel, MouseScrollUnit}};

use super::camera::CameraState;

pub const ZOOM_SPEED: f32 = 0.005;

/// Event denoting that the view scale has changed.
/// Is this even required if objects are redrawn every frame?
#[derive(Event)]
pub struct ScaleChangeEvent {
    pub old_scale: f32,
    pub new_scale: f32,
}


pub fn mouse_scroll(
    mut camera_query: Query<&mut CameraState>,
    mut scroll_events: EventReader<MouseWheel>,
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

    let mut state = camera_query.single_mut();
    let target_scale = state.get_target_scale();
    state.set_target_scale(target_scale * (1.+scroll*0.002));
}

pub fn zoom_camera(
    time: Res<Time>,
    mut camera_query: Query<(&mut CameraState, &Camera, &GlobalTransform)>,
    primary_window: Query<&Window>,
    mut event_writer: EventWriter<ScaleChangeEvent>,
) {
    let window = primary_window.single();
    let Some(cursor_pos) = window.cursor_position() else { return };
    let (mut state, camera, gtrans) = camera_query.single_mut();
    let old_scale = state.get_scale();

    let Some(unscaled_cursor_pos) = camera.viewport_to_world_2d(gtrans, cursor_pos) else { return };
    
    state.zoom_to_target_scale(time.delta().as_millis());

    // Move the camera position to normalize the projection window
    let position_difference = (unscaled_cursor_pos / state.get_scale()) - (unscaled_cursor_pos / old_scale);
    state.position -= position_difference.as_dvec2();

    event_writer.send(ScaleChangeEvent { old_scale, new_scale: state.get_scale() });
}



