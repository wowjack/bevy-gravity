use bevy::color::palettes::css::{CORNFLOWER_BLUE, GREEN, ORANGE, RED, WHITE, YELLOW};
use bevy::math::DVec2;
use bevy::window::WindowResized;
use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*, render::camera::Viewport};
use bevy_egui::EguiPlugin;
use bevy_mod_picking::prelude::*;
use bevy_vector_shapes::Shape2dPlugin;
use gravity_system_tree::builder::GravitySystemBuilder;
use gravity_system_tree::dynamic_body::DynamicBody;
use gravity_system_tree::position_generator::PositionGenerator;
use gravity_system_tree::static_body::StaticBody;
use math::get_orbital_speed;
use pseudo_camera::camera::CameraState;
use pseudo_camera::pseudo_camera_plugin;
use gravity_system_tree::{static_body::StaticPosition, system_manager::GravitySystemManager};
use ui::SIDE_PANEL_WIDTH;
//use util::generate_system;
use visual_object::SimulationState;
pub use bevy;
pub use itertools;



mod ui;
mod visual_object;
mod pseudo_camera;
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
    world: &mut World
) {

    //let child = GravitySystemBuilder::new()
    //    .with_radius(5000.)
    //    .with_position(StaticPosition::Circular { radius: 1e6, speed: get_orbital_speed(3e8, 1e6), start_angle: 0. })
    //    .with_time_step(1)
    //    .with_static_bodies(&[StaticBody::new(StaticPosition::Still, 3000., 1., None)])
    //    .with_dynamic_bodies(&[
    //        DynamicBody::new(DVec2::new(2_000., 0.), DVec2::new(-0.8, 0.7), 1e-10, 100.),
    //        DynamicBody::new(DVec2::new(3_000., 0.), DVec2::new(-6., 0.1), 1e-10, 100.),
    //    ]);
        
    //let test_system = GravitySystemBuilder::new()
    //    .with_radius(2e12)
    //    .with_position(StaticPosition::Still)
    //    .with_time_step(10)
    //    .with_static_bodies(&[StaticBody::new(StaticPosition::Still, 3e8, 100., RED.into())])
    //    .with_dynamic_bodies(&[
    //        DynamicBody::new(DVec2::X*1e5, DVec2::Y*get_orbital_speed(3e8, 1e5)*1e5*1.3, 1e-10, 10., WHITE.into()),
    //        //DynamicBody::new(DVec2::X*1e5, DVec2::Y*get_orbital_speed(1e8, 1e5)*1e5, 1e-10, 10.),
    //        //DynamicBody::new(DVec2::X*1e6, DVec2::Y*get_orbital_speed(1e8, 1e6)*1e6, 1e-10, 10.),
    //    ]);
        //.with_children(&[child]);

    let galaxy_mu = 1e33*G/100.;
    let galaxy_system_radius = 1e20;
    let galaxy_system_time_step = 100;
    let galaxy_radius = 1000.;
    let galaxy_color = Color::from(YELLOW);

    let stellar_orbital_radius = 1e12;
    let stellar_mu = 1.9891e30*G/100.;
    let stellar_system_radius = 1e10;
    let stellar_system_time_step = 10;
    let stellar_radius = 100.;
    let stellar_color = Color::from(ORANGE);

    let planet_orbital_radius = 1.5135e8;
    let planet_mu = 5.972e24*G/100.;
    let planet_system_radius = 5e6;
    let planet_system_time_step = 1;
    let planet_radius = 10.;
    let planet_color = Color::from(GREEN);

    let moon_orbital_radius = 384_400.;
    let moon_mu = 7.35e22*G/100.;
    let moon_radius = 1.;
    let moon_color = Color::from(WHITE);


    let planet_system = GravitySystemBuilder::new()
        .with_radius(planet_system_radius)
        .with_position(StaticPosition::Circular { radius: planet_orbital_radius, speed: get_orbital_speed(stellar_mu, planet_orbital_radius), start_angle: 0. })
        .with_time_step(planet_system_time_step)
        .with_static_bodies(&[
            StaticBody::new(StaticPosition::Still, planet_mu, planet_radius, planet_color),
            StaticBody::new(StaticPosition::Circular { radius: moon_orbital_radius, speed: get_orbital_speed(planet_mu, moon_orbital_radius), start_angle: 0. }, moon_mu, moon_radius, moon_color),
            //StaticBody::new(StaticPosition::Circular { radius: 800., speed: 0.0005, start_angle: 0. }, 0.00000000001, 1., WHITE.into()),
            //StaticBody::new(StaticPosition::Circular { radius: 500., speed: 0.005, start_angle: 0. }, 0.00000000001, 1., WHITE.into()),
        ])
        .with_dynamic_bodies(&[
            DynamicBody::new(DVec2::X*-100_000., -1.*DVec2::Y*get_orbital_speed(planet_mu, 100_000.)*100_000., 1e-20, 1., CORNFLOWER_BLUE.into()),
        ]);
    let stellar_system = GravitySystemBuilder::new()
        .with_radius(stellar_system_radius)
        .with_position(StaticPosition::Circular { radius: stellar_orbital_radius, speed: get_orbital_speed(galaxy_mu, stellar_orbital_radius), start_angle: 0. })
        .with_time_step(stellar_system_time_step)
        .with_static_bodies(&[
            StaticBody::new(StaticPosition::Still, stellar_mu, stellar_radius, stellar_color),
        ])
        .with_children(&[planet_system]);
    let galactic_system = GravitySystemBuilder::new()
        .with_radius(galaxy_system_radius)
        .with_position(StaticPosition::Still)
        .with_time_step(galaxy_system_time_step)
        .with_static_bodies(&[
            StaticBody::new(StaticPosition::Still, galaxy_mu, galaxy_radius, galaxy_color)
        ])
        .with_children(&[stellar_system]);


    let mut manager = GravitySystemManager::new(galactic_system);
    let systems_details = manager.system.get_system_position_gens_and_radii();
    world.insert_resource(SystemCircleResource { draw: true, gens: systems_details });
    manager.spawn_entities(world);
    world.insert_non_send_resource(manager);
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














