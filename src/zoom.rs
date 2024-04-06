use bevy::{prelude::*, input::mouse::{MouseWheel, MouseScrollUnit}};

use crate::ui::{SIDE_PANEL_WIDTH, BOTTOM_PANEL_HEIGHT};

#[derive(Event)]
pub struct ProjectionScaleChange;

pub fn mouse_zoom(
    mut camera_query: Query<(&mut OrthographicProjection, &mut Transform)>,
    mut scroll_events: EventReader<MouseWheel>,
    primary_window: Query<&Window>,
    mut event_writer: EventWriter<ProjectionScaleChange>,
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

    let (mut proj, mut pos) = camera_query.single_mut();    
    let window = primary_window.single();
    let window_size = Vec2::new(window.width() - SIDE_PANEL_WIDTH, window.height() - BOTTOM_PANEL_HEIGHT);
    let mouse_normalized_screen_pos = window
        .cursor_position()
        .map(|cursor_pos| (cursor_pos / window_size) * 2. - Vec2::ONE)
        .map(|p| Vec2::new(p.x, -p.y));

    let old_scale = proj.scale;
    proj.scale = proj.scale * (1. + -scroll * 0.001);

    // Move the camera position to normalize the projection window
    if let Some(mouse_normalized_screen_pos) = mouse_normalized_screen_pos {
        let proj_size = proj.area.max / old_scale;
        let mouse_world_pos = pos.translation.truncate()
            + mouse_normalized_screen_pos * proj_size * old_scale;
        pos.translation = (mouse_world_pos
            - mouse_normalized_screen_pos * proj_size * proj.scale)
            .extend(pos.translation.z);
    }
    event_writer.send(ProjectionScaleChange);
}