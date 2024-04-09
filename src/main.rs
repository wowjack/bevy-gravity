use bevy::prelude::*;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::render::camera::Viewport;
use bevy::window::WindowResized;
use bevy_egui::EguiPlugin;
use bevy_math::DVec2;
use bevy_mod_picking::prelude::*;
use bevy_prototype_lyon::prelude::*;
use pseudo_camera::CameraState;
use ui::SIDE_PANEL_WIDTH;
use zoom::{mouse_zoom, ScaleChangeEvent};

mod zoom;
mod ui;
mod massive_object;
mod pseudo_camera;

//barnes-hut

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins/*.disable::<LogPlugin>()*/,
            EguiPlugin,
            DefaultPickingPlugins.build().disable::<DebugPickingPlugin>(),
            ShapePlugin,
            FrameTimeDiagnosticsPlugin,
            //LogDiagnosticsPlugin::default(),
        ))
        .add_event::<ScaleChangeEvent>()
        .insert_resource(ClearColor(Color::rgb(0.7, 0.7, 0.7)))
        .add_systems(Startup, init)
        .add_systems(Update, (
            window_resize.before(mouse_zoom),
            mouse_zoom,
            ui::side_panel,
        ))
        .run()
}



fn init(
    mut commands: Commands,
) {
    commands.spawn((
        Camera2dBundle::default(),
        CameraState::default(),
    ));
}





//need to adjust the viewport whenever the window is resized. (these events come ever frame for some reason)
fn window_resize(mut events: EventReader<WindowResized>, mut camera_query: Query<&mut Camera, With<CameraState>>, window_query: Query<&Window>) {
    let mut camera = camera_query.single_mut();
    
    for event in events.read() {

        let Ok(window) = window_query.get(event.window) else { continue };
        let width = ((window.width() - SIDE_PANEL_WIDTH) * window.scale_factor()) as u32;
        let height = window.physical_height();
    
        camera.viewport = Some(Viewport {
            physical_position: UVec2::ZERO,
            physical_size: UVec2::new(width, height),
            depth: (0.0)..(1.0)
        });
    }
}














