use bevy::{prelude::*, input::mouse::{MouseWheel, MouseScrollUnit}};

#[derive(Event)]
pub struct ProjectionScaleChange(f32);

pub fn mouse_zoom(
    mut query: Query<(&mut OrthographicProjection, &mut Transform)>,
    mut scroll_events: EventReader<MouseWheel>,
    primary_window: Query<&Window>,
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

    let window = primary_window.single();
    let window_size = Vec2::new(window.width(), window.height());
    let mouse_normalized_screen_pos = window
        .cursor_position()
        .map(|cursor_pos| (cursor_pos / window_size) * 2. - Vec2::ONE)
        .map(|p| Vec2::new(p.x, -p.y));

    for (mut proj, mut pos) in &mut query {
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
    }
}