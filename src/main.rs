use bevy::prelude::*;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::render::camera::Viewport;
use bevy::window::WindowResized;
use bevy_egui::EguiPlugin;
use bevy_math::DVec2;
use bevy_mod_picking::prelude::*;
use bevy_prototype_lyon::prelude::*;
use ui::SIDE_PANEL_WIDTH;
use zoom::{mouse_zoom, ScaleChangeEvent};

mod zoom;
mod ui;
mod massive_object;

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


/// Component representing the "state" of the camera
/// This is not the actual state of the camera since I want to allow for correct rendering of far away objects.
/// In reality the camera / projection does not move or scale, instead everything else does.
/// This way objects are always close to the origin when you can see them, so there isn't any float precision rendering nonsense
#[derive(Component, Clone)]
pub struct CameraState {
    // viewing far-away objects may still be a problem.
    // when a faraway object is translated to the origin, the object will render correctly but move in clearly discrete steps.
    // Same problem, different issue. It all stems from floating point precision
    position: DVec2, // maybe change to multiple precision in the future (if gravity calculation is optimized enough)
    scale: f32, // f32 should be fine for scale
}
impl Default for CameraState {
    fn default() -> Self {
        Self { position: Default::default(), scale: 1. }
    }
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














