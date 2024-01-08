use bevy::prelude::*;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::render::camera::Viewport;
use bevy::window::WindowResized;
use bevy_egui::EguiPlugin;
use bevy_mod_picking::prelude::*;
use bevy_prototype_lyon::prelude::ShapePlugin;
use object::MassiveObjectPlugin;
use object::select::SelectedObjects;
use ui::{SIDE_PANEL_WIDTH, BOTTOM_PANEL_HEIGHT};
use zoom::mouse_zoom;

mod zoom;
mod object;
mod ui;


fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins/*.disable::<LogPlugin>()*/,
            EguiPlugin,
            DefaultPickingPlugins.build().disable::<DebugPickingPlugin>(),
            ShapePlugin,
            FrameTimeDiagnosticsPlugin,
            //LogDiagnosticsPlugin::default(),
            MassiveObjectPlugin
        ))
        .insert_resource(ClearColor(Color::rgb(0.7, 0.7, 0.7)))
        .insert_resource(SelectedObjects::default())
        .add_systems(Startup, init)
        .add_systems(Update, (
            window_resize.before(mouse_zoom),
            mouse_zoom,
            ui::bottom_panel,
            ui::side_panel,
        ))
        .run()
}


#[derive(Component)]
pub struct MainCamera;

fn init(
    mut commands: Commands,
    window_query: Query<&Window>
) {
    let window = window_query.single();

    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                viewport: Some(Viewport {
                    physical_position: UVec2::ZERO,
                    physical_size: UVec2::new(window.physical_width()-SIDE_PANEL_WIDTH as u32, window.physical_height()-BOTTOM_PANEL_HEIGHT as u32),
                    depth: (0.0)..(1.0)
                }),
                ..default()
            },
            ..default()
        },
        MainCamera
    ));
}


//need to adjust the viewport whenever the window is resized.
fn window_resize(mut events: EventReader<WindowResized>, mut camera_query: Query<&mut Camera, With<MainCamera>>) {
    if events.is_empty() { return }

    let mut camera = camera_query.single_mut();

    for event in events.read() {
        camera.viewport = Some(Viewport{
            physical_size: UVec2::new((event.width - SIDE_PANEL_WIDTH) as u32, (event.height - BOTTOM_PANEL_HEIGHT) as u32),
            physical_position: UVec2::ZERO,
            depth: (0.0)..(1.0)
        });
    }
}














