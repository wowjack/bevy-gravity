use bevy::prelude::*;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::render::camera::Viewport;
use bevy::window::WindowResized;
use bevy_egui::EguiPlugin;
use bevy_math::DVec2;
use bevy_mod_picking::prelude::*;
use bevy_vector_shapes::Shape2dPlugin;
use fps_text::{spawn_debug_text, update_debug_text};
use itertools::Itertools;
use physics::{ChangeEvent, FutureFrame, MassiveObject, ObjectFuture};
use pseudo_camera::CameraState;
use ui::{ObjectSpawnOptions, SIDE_PANEL_WIDTH};
use visual_object::{CircleAssets, VisualObjectBundle};
use zoom::{mouse_zoom, ScaleChangeEvent};



mod zoom;
mod ui;
mod visual_object;
mod pseudo_camera;
mod physics;
mod fps_text;

//barnes-hut

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins/*.disable::<LogPlugin>()*/,
            EguiPlugin,
            DefaultPickingPlugins.build().disable::<DebugPickingPlugin>(),
            FrameTimeDiagnosticsPlugin,
            //LogDiagnosticsPlugin::default(),
            physics::PhysicsPlugin,
            visual_object::VisualObjectPlugin,
            Shape2dPlugin::default(),
        ))
        .add_event::<ScaleChangeEvent>()
        .insert_resource(ClearColor(Color::rgb(0.7, 0.7, 0.7)))
        .add_systems(Startup, (
            init,
            spawn_debug_text
        ))
        .add_systems(PostStartup, spawns)
        .add_systems(Update, (
            update_debug_text,
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

fn spawns(mut commands: Commands, mut ew: EventWriter<ChangeEvent>, circle_assets: Res<CircleAssets>) {
    let obj1 = MassiveObject { position: DVec2::ZERO, velocity: DVec2::Y*4., mass: 1. };
    let obj2 = MassiveObject { position: DVec2::X*-50., velocity: DVec2::ZERO, mass: 1000000000000. };
    let obj3 = MassiveObject { position: DVec2::X*50., velocity: DVec2::Y*2., mass: 1. };
    let obj4 = MassiveObject { position: DVec2::X*150., velocity: DVec2::Y*1., mass: 1. };
    let obj5 = MassiveObject { position: DVec2::X*250., velocity: DVec2::Y*2.1, mass: 1. };
    let e1 = commands.spawn(VisualObjectBundle::new(obj1.clone(), 2., circle_assets.as_ref())).id();
    let e2 = commands.spawn(VisualObjectBundle::new(obj2.clone(), 15., circle_assets.as_ref())).id();
    let e3 = commands.spawn(VisualObjectBundle::new(obj3.clone(), 3., circle_assets.as_ref())).id();
    let e4 = commands.spawn(VisualObjectBundle::new(obj4.clone(), 7., circle_assets.as_ref())).id();
    let e5 = commands.spawn(VisualObjectBundle::new(obj5.clone(), 8., circle_assets.as_ref())).id();
    ew.send_batch(vec![
        ChangeEvent { entity: e1, change: physics::Change::CreateObject(obj1) },
        ChangeEvent { entity: e2, change: physics::Change::CreateObject(obj2) },
        ChangeEvent { entity: e3, change: physics::Change::CreateObject(obj3) },
        ChangeEvent { entity: e4, change: physics::Change::CreateObject(obj4) },
        ChangeEvent { entity: e5, change: physics::Change::CreateObject(obj5) },
    ]);
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














