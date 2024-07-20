use bevy::math::DVec2;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::window::WindowResized;
use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*, render::camera::Viewport};
use bevy_egui::EguiPlugin;
use bevy_mod_picking::prelude::*;
use bevy_vector_shapes::Shape2dPlugin;
use gravity_system_tree::builder::GravitySystemBuilder;
use gravity_system_tree::dynamic_body::DynamicBody;
use gravity_system_tree::static_body::StaticBody;
use itertools::Itertools;
//use physics::{Change, ChangeEvent, MassiveObject};
use pseudo_camera::camera::CameraState;
use pseudo_camera::pseudo_camera_plugin;
use gravity_system_tree::{static_body::StaticPosition, system_manager::GravitySystemManager};
use gravity_system_tree::SystemTree;
use ui::SIDE_PANEL_WIDTH;
//use util::generate_system;
use visual_object::{CircleMesh, VisualObjectBundle, VisualObjectData};
pub use bevy;
pub use itertools;



mod ui;
mod visual_object;
mod pseudo_camera;
//mod physics;
mod util;
mod math;
pub mod gravity_system_tree;


pub fn library_main() {
    App::new()
        .add_plugins((
            DefaultPlugins/*.disable::<LogPlugin>()*/,
            EguiPlugin,
            DefaultPickingPlugins.build().disable::<DebugPickingPlugin>().disable::<DefaultHighlightingPlugin>(),
            FrameTimeDiagnosticsPlugin,
            //LogDiagnosticsPlugin::default(),
            //physics::PhysicsPlugin,
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

pub const G: f64 = 6.6743015e-11;

fn init(
    mut commands: Commands,
    circle_mesh: Res<CircleMesh>,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
    //mut change_event_writer: EventWriter<ChangeEvent>,
) {
    let test_system = GravitySystemBuilder::new()
        .with_radius(1_000.)
        .with_position(StaticPosition::Circular { radius: 100_000., speed: 0.0005, start_angle: 0. })
        .with_time_step(1)
        .with_static_bodies(&[
            StaticBody::new(StaticPosition::Still, 100., 1., None),
            StaticBody::new(StaticPosition::Circular { radius: 600., speed: 0.001, start_angle: 0. }, 0.00000000001, 1., None),
            StaticBody::new(StaticPosition::Circular { radius: 800., speed: 0.0005, start_angle: 0. }, 0.00000000001, 1., None),
            StaticBody::new(StaticPosition::Circular { radius: 500., speed: 0.005, start_angle: 0. }, 0.00000000001, 1., None),
        ])
        .with_dynamic_bodies(&[
            DynamicBody::new(DVec2::new(10., 0.), DVec2::new(0., 3.), 1., None),
            DynamicBody::new(DVec2::new(20., 0.), DVec2::new(0., 2.5), 1., None),
            DynamicBody::new(DVec2::new(35., 0.), DVec2::new(0., 2.), 1., None),
            DynamicBody::new(DVec2::new(100., 0.), DVec2::new(0., 1.), 1., None),
            DynamicBody::new(DVec2::new(120., 0.), DVec2::new(0., 0.5), 1., None),

            DynamicBody::new(DVec2::new(-10., 0.), DVec2::new(0., 3.), 1., None),
            DynamicBody::new(DVec2::new(-20., 0.), DVec2::new(0., 2.5), 1., None),
            DynamicBody::new(DVec2::new(-35., 0.), DVec2::new(0., 2.), 1., None),
            DynamicBody::new(DVec2::new(-100., 0.), DVec2::new(0., 1.), 1., None),
            DynamicBody::new(DVec2::new(-120., 0.), DVec2::new(0., 0.5), 1., None),


            DynamicBody::new(DVec2::new(0., 10.), DVec2::new(3., 0.), 1., None),
            DynamicBody::new(DVec2::new(0., 20.), DVec2::new(2.5, 0.), 1., None),
            DynamicBody::new(DVec2::new(0., 35.), DVec2::new(2., 0.), 1., None),
            DynamicBody::new(DVec2::new(0., 100.), DVec2::new(1., 0.), 1., None),
            DynamicBody::new(DVec2::new(0., 120.), DVec2::new(0.5, 0.), 1., None),

            DynamicBody::new(DVec2::new(0., -10.), DVec2::new(3., 0.), 1., None),
            DynamicBody::new(DVec2::new(0., -20.), DVec2::new(2.5, 0.), 1., None),
            DynamicBody::new(DVec2::new(0., -35.), DVec2::new(2., 0.), 1., None),
            DynamicBody::new(DVec2::new(0., -100.), DVec2::new(1., 0.), 1., None),
            DynamicBody::new(DVec2::new(0., -120.), DVec2::new(0.5, 0.), 1., None),
        ]);
    let parent_system = GravitySystemBuilder::new()
        .with_radius(1_000_000_000.)
        .with_position(StaticPosition::Still)
        .with_time_step(10)
        .with_static_bodies(&[
            StaticBody::new(StaticPosition::Still, 1_000_000_000., 100., None)
        ])
        .with_dynamic_bodies(&[
            DynamicBody::new(DVec2::new(51_000., 0.), DVec2::new(0., 140.), 1., None),
            DynamicBody::new(DVec2::new(40_000., 0.), DVec2::new(0., 160.), 1., None),
            DynamicBody::new(DVec2::new(42_000., 0.), DVec2::new(0., 140.), 1., None),
            DynamicBody::new(DVec2::new(43_000., 0.), DVec2::new(0., 140.), 1., None),
            DynamicBody::new(DVec2::new(45_000., 0.), DVec2::new(0., 150.), 1., None),
            DynamicBody::new(DVec2::new(50_000., 0.), DVec2::new(0., 150.), 1., None),
            DynamicBody::new(DVec2::new(52_000., 0.), DVec2::new(0., 140.), 1., None),
            DynamicBody::new(DVec2::new(56_000., 0.), DVec2::new(0., 120.), 1., None),
            DynamicBody::new(DVec2::new(58_000., 0.), DVec2::new(0., 120.), 1., None),
            DynamicBody::new(DVec2::new(60_000., 0.), DVec2::new(0., 100.), 1., None),

            DynamicBody::new(DVec2::new(-51_000., 0.), DVec2::new(0., -140.), 1., None),
            DynamicBody::new(DVec2::new(-40_000., 0.), DVec2::new(0., -160.), 1., None),
            DynamicBody::new(DVec2::new(-42_000., 0.), DVec2::new(0., -140.), 1., None),
            DynamicBody::new(DVec2::new(-43_000., 0.), DVec2::new(0., -140.), 1., None),
            DynamicBody::new(DVec2::new(-45_000., 0.), DVec2::new(0., -150.), 1., None),
            DynamicBody::new(DVec2::new(-50_000., 0.), DVec2::new(0., -150.), 1., None),
            DynamicBody::new(DVec2::new(-52_000., 0.), DVec2::new(0., -140.), 1., None),
            DynamicBody::new(DVec2::new(-56_000., 0.), DVec2::new(0., -120.), 1., None),
            DynamicBody::new(DVec2::new(-58_000., 0.), DVec2::new(0., -120.), 1., None),
            DynamicBody::new(DVec2::new(-60_000., 0.), DVec2::new(0., -100.), 1., None),
        ])
        .with_children(&[
            test_system.clone().with_position(StaticPosition::Circular { radius: 105_000., speed: 0.000005, start_angle: 0.5 }),
            test_system.clone().with_position(StaticPosition::Circular { radius: 160_000., speed: 0.0003, start_angle: 3.5 }),
            test_system.clone().with_position(StaticPosition::Circular { radius: 117_000., speed: 0.00044, start_angle: 5. }),
            test_system.clone().with_position(StaticPosition::Circular { radius: 122_000., speed: 0.00053, start_angle: 1.5 }),
            test_system.clone().with_position(StaticPosition::Circular { radius: 136_000., speed: 0.00044, start_angle: 0.5 }),
            test_system.clone().with_position(StaticPosition::Circular { radius: 140_000., speed: 0.00035, start_angle: 2. }),
            test_system.clone().with_position(StaticPosition::Circular { radius: 100_000., speed: 0.0005, start_angle: 3. }),
            test_system.clone().with_position(StaticPosition::Circular { radius: 100_000., speed: 0.0005, start_angle: 5.5 }),
            test_system.clone().with_position(StaticPosition::Circular { radius: 110_000., speed: 0.0005, start_angle: 4. }),
            test_system,
        ]);
    let entities = (0..parent_system.total_bodies()).map(|_|
        commands.spawn(VisualObjectBundle::new(VisualObjectData::default(), circle_mesh.0.clone().into(), &mut color_materials)).id()
    ).collect_vec();
    let manager = GravitySystemManager::new(parent_system, &entities);
    commands.insert_resource(manager);
    
    /*
    let bodies = generate_system();
    for bundle in bodies.into_iter().map(|d| VisualObjectBundle::new(d, circle_mesh.0.clone().into(), &mut color_materials)) {
        let object_data = bundle.object_data.clone();
        let entity = commands.spawn(bundle).id();
        let event = ChangeEvent { entity, change: Change::CreateObject(MassiveObject::from(object_data)) };
        change_event_writer.send(event);
    }
    */
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













