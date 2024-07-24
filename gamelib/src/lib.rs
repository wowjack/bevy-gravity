use bevy::color::palettes::css::CORNFLOWER_BLUE;
use bevy::math::DVec2;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::window::WindowResized;
use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*, render::camera::Viewport};
use bevy_egui::EguiPlugin;
use bevy_mod_picking::prelude::*;
use bevy_vector_shapes::Shape2dPlugin;
use gravity_system_tree::builder::GravitySystemBuilder;
use gravity_system_tree::dynamic_body::DynamicBody;
use gravity_system_tree::position_generator::PositionGenerator;
use gravity_system_tree::static_body::StaticBody;
use itertools::Itertools;
use math::get_orbital_speed;
//use physics::{Change, ChangeEvent, MassiveObject};
use pseudo_camera::camera::CameraState;
use pseudo_camera::pseudo_camera_plugin;
use gravity_system_tree::{static_body::StaticPosition, system_manager::GravitySystemManager};
use gravity_system_tree::SystemTree;
use ui::SIDE_PANEL_WIDTH;
//use util::generate_system;
use visual_object::{CircleMesh, SimulationState, VisualObjectBundle, VisualObjectData};
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
    //println!("half hour speed: {}", std::f64::consts::TAU/(60.0f64.powi(2)*30.));
    //let center_mass = 3e20;
    //let orbit_radius = 1e10;
    //let orbital_speed = get_orbital_speed(center_mass, orbit_radius);
    //return
    
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
            draw_system_circles,
        ))
        .run();
}

pub const G: f64 = 6.6743015e-11;



fn init(
    mut commands: Commands,
    circle_mesh: Res<CircleMesh>,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
) {
    let second_speed = std::f64::consts::TAU/60.;
    println!("5 minute speed:  {}", second_speed/60./5.);

    let child = GravitySystemBuilder::new()
        .with_radius(100.)
        .with_position(StaticPosition::Circular { radius: 1e4, speed: second_speed/60., start_angle: 0. })
        .with_time_step(1)
        .with_static_bodies(&[StaticBody::new(StaticPosition::Still, 100_000., 1., None)])
        .with_dynamic_bodies(&[
            //DynamicBody::new(DVec2::new(1_000., 0.), DVec2::new(0., -10.), 1e-10, None)
        ]);
        
    let test_system = GravitySystemBuilder::new()
        .with_radius(2e12)
        .with_position(StaticPosition::Still)
        .with_time_step(10)
        .with_static_bodies(&[StaticBody::new(StaticPosition::Still, 1e15, 100., None)])
        .with_children(&[child]);
    let manager = GravitySystemManager::new(test_system, &mut commands);
    let systems_details = manager.system.get_system_position_gens_and_radii();
    commands.insert_resource(SystemCircleResource { draw: true, gens: systems_details });
    commands.insert_resource(manager);
}

#[derive(Resource)]
struct SystemCircleResource {
    pub draw: bool,
    pub gens: Vec<(PositionGenerator, f64)>
}

fn draw_system_circles(
    mut gizmos: Gizmos,
    systems_resource: Res<SystemCircleResource>,
    state: Res<SimulationState>,
    camera_query: Query<&CameraState>
) {
    if systems_resource.draw == false { return }
    let camera = camera_query.single();
    for (gen, radius) in &systems_resource.gens {
        let position = gen.get(state.current_time);
        gizmos.circle_2d(camera.physics_to_world_pos(position), *radius as f32*camera.get_scale(), CORNFLOWER_BLUE);
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














