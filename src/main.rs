use background::{BackgroundBundle, DraggingBackground, SelectInRectEvent, rect_select, scale_background};
use bevy::prelude::*;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::render::camera::Viewport;
use bevy::window::WindowResized;
use bevy_egui::EguiPlugin;
use bevy_mod_picking::prelude::*;
use bevy_prototype_lyon::prelude::ShapePlugin;
use object::MassiveObjectPlugin;
use ui::{SIDE_PANEL_WIDTH, BOTTOM_PANEL_HEIGHT, ToDraw};
use zoom::{mouse_zoom, ProjectionScaleChange};

mod zoom;
mod object;
mod ui;
mod background;


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
        .insert_resource(DraggingBackground::default())
        .insert_resource(ToDraw::default())
        .add_event::<SelectInRectEvent>()
        .add_event::<ProjectionScaleChange>()
        .add_systems(Startup, init)
        .add_systems(Update, (
            window_resize.before(mouse_zoom),
            mouse_zoom.before(scale_background),
            scale_background,
            ui::bottom_panel,
            ui::side_panel,
            rect_select,
        ))
        .run()
}


#[derive(Component)]
pub struct MainCamera;

fn init(
    mut commands: Commands,
    window_query: Query<&Window>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>
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
        MainCamera,
    )).with_children(|builder| {
        builder.spawn(BackgroundBundle::new(&mut materials, &mut meshes));
    });
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














