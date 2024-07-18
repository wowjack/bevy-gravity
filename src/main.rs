use bevy::prelude::*;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::render::camera::Viewport;
use bevy::window::WindowResized;
use bevy_egui::EguiPlugin;
use bevy_mod_picking::prelude::*;
use bevy_vector_shapes::Shape2dPlugin;
use itertools::Itertools;
use physics::{Change, ChangeEvent, MassiveObject};
use pseudo_camera::camera::CameraState;
use pseudo_camera::pseudo_camera_plugin;
use system_tree::SystemTree;
use ui::SIDE_PANEL_WIDTH;
use util::generate_system;
use visual_object::{CircleMesh, VisualObjectBundle};


mod ui;
mod visual_object;
mod pseudo_camera;
mod physics;
mod util;
mod system_tree;
mod math;


fn main() {
    system_tree::run();
    
}


fn maina() {
    App::new()
        .add_plugins((
            DefaultPlugins/*.disable::<LogPlugin>()*/,
            EguiPlugin,
            DefaultPickingPlugins.build().disable::<DebugPickingPlugin>().disable::<DefaultHighlightingPlugin>(),
            FrameTimeDiagnosticsPlugin,
            //LogDiagnosticsPlugin::default(),
            physics::PhysicsPlugin,
            visual_object::VisualObjectPlugin,
            Shape2dPlugin::default(),
            pseudo_camera_plugin
        ))
        .insert_resource(ClearColor(Color::linear_rgb(0.7, 0.7, 0.7)))
        .add_systems(PostStartup, init)
        .add_systems(Update, (
            window_resize,
            ui::side_panel,
        ))
        .run();
}

fn init(
    mut commands: Commands,
    circle_mesh: Res<CircleMesh>,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
    mut change_event_writer: EventWriter<ChangeEvent>,
) {
    return;
    let bodies = generate_system();
    for bundle in bodies.into_iter().map(|d| VisualObjectBundle::new(d, circle_mesh.0.clone().into(), &mut color_materials)) {
        let object_data = bundle.object_data.clone();
        let entity = commands.spawn(bundle).id();
        let event = ChangeEvent { entity, change: Change::CreateObject(MassiveObject::from(object_data)) };
        change_event_writer.send(event);
    }
}




//need to adjust the viewport whenever the window is resized. (these events come ever frame for some reason)
fn window_resize(mut events: EventReader<WindowResized>, mut camera_query: Query<(&mut Camera, &mut CameraState)>, window_query: Query<&Window>) {
    let (mut camera, mut camera_state) = camera_query.single_mut();
    
    for event in events.read() {
        let Ok(window) = window_query.get(event.window) else { continue };

        camera_state.dimensions = Vec2::new(window.width(), window.height() - SIDE_PANEL_WIDTH);

        let width = ((window.width() - SIDE_PANEL_WIDTH) * window.scale_factor()) as u32;
        let height = window.physical_height();
    
        camera.viewport = Some(Viewport {
            physical_position: UVec2::ZERO,
            physical_size: UVec2::new(width, height),
            depth: (0.0)..(1.0)
        });
    }
}














