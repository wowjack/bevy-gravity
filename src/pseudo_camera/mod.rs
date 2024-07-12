/*
Create a pseudo-camera with zooming capabilities and a background grid
*/


pub mod fps_text;
pub mod zoom;
pub mod scale_grid;
pub mod camera;

use bevy::prelude::*;
use camera::CameraState;
use fps_text::{spawn_debug_text, update_debug_text};
use scale_grid::{draw_scale_grid, update_gizmo_config, AxesConfig, LargeGridConfig, SmallGridConfig};
use zoom::{mouse_scroll, zoom_camera, ScaleChangeEvent};

pub fn pseudo_camera_plugin(app: &mut App) {
    app.add_event::<ScaleChangeEvent>()
        .init_gizmo_group::<LargeGridConfig>()
        .init_gizmo_group::<SmallGridConfig>()
        .init_gizmo_group::<AxesConfig>()
        .add_systems(Startup, init)
        .add_systems(PreUpdate, (update_debug_text, update_gizmo_config, zoom_camera))
        .add_systems(Update, (mouse_scroll, draw_scale_grid));
}

fn init(mut commands: Commands, mut cs: ResMut<GizmoConfigStore>) {
    commands.spawn((
        Camera2dBundle::default(),
        CameraState::default()
    ));

    cs.config_mut::<AxesConfig>().0.line_width = 0.025;

    spawn_debug_text(commands);
}