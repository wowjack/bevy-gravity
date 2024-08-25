use core::f64;
use std::collections::VecDeque;

use bevy::color::palettes::css::{ALICE_BLUE, BLUE, CORNFLOWER_BLUE, GRAY, GREEN, MAGENTA, ORANGE, PURPLE, WHITE, YELLOW};
use bevy::color::palettes::tailwind::YELLOW_100;
use bevy::math::DVec2;
use bevy::window::WindowResized;
use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*, render::camera::Viewport};
use bevy_egui::EguiPlugin;
use bevy_mod_picking::prelude::*;
use bevy_vector_shapes::prelude::ShapePainter;
use bevy_vector_shapes::shapes::DiscPainter;
use bevy_vector_shapes::Shape2dPlugin;
use gravity_system_tree::builder::GravitySystemBuilder;
use gravity_system_tree::dynamic_body::DynamicBody;
use gravity_system_tree::position_generator::PositionGenerator;
use gravity_system_tree::static_body::StaticBody;
use math::{get_orbital_radius, get_orbital_speed};
use pseudo_camera::camera::CameraState;
use pseudo_camera::pseudo_camera_plugin;
use gravity_system_tree::{static_body::StaticPosition, system_manager::GravitySystemManager};
use ui::SIDE_PANEL_WIDTH;
use visual_object::SimulationState;
pub use bevy;
pub use itertools;



mod ui;
mod visual_object;
mod pseudo_camera;
mod util;
pub mod math;
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
        .insert_resource(ClearColor(Color::linear_rgb(0.001, 0.001, 0.001)))
        .add_systems(PostStartup, init)
        .add_systems(Update, (
            window_resize,
            ui::side_panel,
            draw_system_circles,
        ))
        .run();
}

pub const G: f64 = 6.6743015e-11;

const jupiter_mu: f64 = 1.898e27*G/1e6;
const jupiter_radius: f64 = 69_911.;
const jupiter_color: Srgba = ORANGE;
const jupiter_orbital_radius: f64 = 7.78e8;
const jupiter_system_radius: f64 = 3e6;
const jupiter_system_time_step: u64 = 1;

const io_mu: f64 = 8.9319e22*G/1e6;
const io_radius: f64 = 1_821.6;
const io_color: Srgba = ALICE_BLUE;
const io_orbital_radius: f64 = 422_000.;

const europa_mu: f64 = 4.799844e22*G/1e6;
const europa_radius: f64 = 1_560.8;
const europa_color: Srgba = BLUE;
const europa_orbital_radius: f64 = 671_000.;

const callisto_mu: f64 = 1.075938e23*G/1e6;
const callisto_radius: f64 = 2_410.3;
const callisto_color: Srgba = YELLOW_100;
const callisto_orbital_radius: f64 = 1_883_000.;

const ganymede_mu: f64 = 1.4819e23*G/1e6;
const ganymede_radius: f64 = 2_634.1;
const ganymede_color: Srgba = GRAY;
const ganymede_orbital_radius: f64 = 1_070_000.;



fn init(
    world: &mut World
) {
    let galaxy_mu = 1e34*G/1e6;
    let galaxy_system_radius = 1e20;
    let galaxy_system_time_step = 100;
    let galaxy_radius = 1000.;
    let galaxy_color = Color::from(PURPLE);

    let stellar_orbital_radius = 1e12;
    let stellar_mu = 1.9891e30*G/1e6;
    let stellar_system_radius = 5e9;
    let stellar_system_time_step = 10;
    let stellar_radius = 69340.;
    let stellar_color = Color::from(YELLOW);

    let planet_orbital_radius = 1.5135e8;
    let planet_mu = 5.972e24*G/1e6;
    let planet_system_radius = 3e6;
    let planet_system_time_step = 1;
    let planet_radius = 6378.14;
    let planet_color = Color::from(GREEN);

    let moon_orbital_radius = 384_400.;
    let moon_mu = 7.35e22*G/1e6;
    let moon_radius = 1737.4;
    let moon_color = Color::from(WHITE);


    let mut planet_orbiter = DynamicBody::new(DVec2::X*-7_000., DVec2::Y*get_orbital_speed(planet_mu, 7_000.)*7_000., 1e-30, 1., CORNFLOWER_BLUE.into());
    
    planet_orbiter.future_actions.extend((1u64..98).map(|x| (x, DVec2::Y*2.)));
    planet_orbiter.future_actions.extend((200u64..203).map(|x| (x, DVec2::X*-1.)));
    planet_orbiter.future_actions.push_back((203, DVec2::Y));
    planet_orbiter.future_actions.push_back((204, DVec2::Y*0.5));
    planet_orbiter.future_actions.push_back((4500, DVec2::X*-1.));
    planet_orbiter.future_actions.push_back((4682, DVec2::splat(-20.)));
    planet_orbiter.future_actions.push_back((4683, DVec2::X*-10.));
    planet_orbiter.future_actions.push_back((4684, DVec2::X*-10.));
    planet_orbiter.future_actions.push_back((4685, DVec2::X*-10.));
    planet_orbiter.future_actions.push_back((4686, DVec2::Y*10.));
    planet_orbiter.future_actions.push_back((4687, DVec2::Y*3.5));


    let earth_system = GravitySystemBuilder::new()
        .with_radius(planet_system_radius)
        .with_position(StaticPosition::Circular { radius: planet_orbital_radius, speed: get_orbital_speed(stellar_mu, planet_orbital_radius), start_angle: 0. })
        .with_time_step(planet_system_time_step)
        .with_static_bodies(&[
            StaticBody::new(StaticPosition::Still, planet_mu, planet_radius, planet_color),
            StaticBody::new(StaticPosition::Circular { radius: moon_orbital_radius, speed: get_orbital_speed(planet_mu, moon_orbital_radius), start_angle: 0. }, moon_mu, moon_radius, moon_color),
        ])
        .with_dynamic_bodies(&[
            planet_orbiter,
            //DynamicBody::new(DVec2::X*(moon_orbital_radius+500.), DVec2::Y*get_orbital_speed(moon_mu, 500.)*500., 1e-30, 1., MAGENTA.into())
        ]);
    let jupiter_system = GravitySystemBuilder::new()
            .with_radius(jupiter_system_radius)
            .with_position(StaticPosition::Circular { radius: jupiter_orbital_radius, speed: get_orbital_speed(stellar_mu, jupiter_orbital_radius), start_angle: 1. })
            .with_time_step(jupiter_system_time_step)
            .with_static_bodies(&[
                StaticBody::new(StaticPosition::Still, jupiter_mu, jupiter_radius, jupiter_color.into()),
                StaticBody::new(StaticPosition::Circular { radius: io_orbital_radius, speed: get_orbital_speed(jupiter_mu, io_orbital_radius), start_angle: 0. }, io_mu, io_radius, io_color.into()),
                StaticBody::new(StaticPosition::Circular { radius: europa_orbital_radius, speed: get_orbital_speed(jupiter_mu, europa_orbital_radius), start_angle: 0. }, europa_mu, europa_radius, europa_color.into()),
                StaticBody::new(StaticPosition::Circular { radius: callisto_orbital_radius, speed: get_orbital_speed(jupiter_mu, callisto_orbital_radius), start_angle: 0. }, callisto_mu, callisto_radius, callisto_color.into()),
                StaticBody::new(StaticPosition::Circular { radius: ganymede_orbital_radius, speed: get_orbital_speed(jupiter_mu, ganymede_orbital_radius), start_angle: 0. }, ganymede_mu, ganymede_radius, ganymede_color.into()),
            ]);
    let stellar_system = GravitySystemBuilder::new()
        .with_radius(stellar_system_radius)
        .with_position(StaticPosition::Circular { radius: stellar_orbital_radius, speed: get_orbital_speed(galaxy_mu, stellar_orbital_radius), start_angle: 0. })
        .with_time_step(stellar_system_time_step)
        .with_static_bodies(&[
            StaticBody::new(StaticPosition::Still, stellar_mu, stellar_radius, stellar_color),
        ])
        .with_children(&[
            earth_system,
            jupiter_system
        ]);
    let galactic_system = GravitySystemBuilder::new()
        .with_radius(galaxy_system_radius)
        .with_position(StaticPosition::Still)
        .with_time_step(galaxy_system_time_step)
        .with_static_bodies(&[
            StaticBody::new(StaticPosition::Still, galaxy_mu, galaxy_radius, galaxy_color)
        ])
        .with_dynamic_bodies(&[
            DynamicBody::new(DVec2::X*100_000_000., DVec2::Y*50_000., 1e-30, 1., CORNFLOWER_BLUE.into()),
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
    mut painter: ShapePainter,
    systems_resource: Res<SystemCircleResource>,
    state: Res<SimulationState>,
    camera_query: Query<&CameraState>
) {
    if systems_resource.draw == false { return }
    let camera = camera_query.single();
    painter.hollow = true;
    painter.color = CORNFLOWER_BLUE.into();

    for (gen, radius) in &systems_resource.gens {
        let position = camera.physics_to_world_pos(gen.get(state.current_time));
        painter.transform.translation = position.extend(0.);
        painter.circle(*radius as f32*camera.get_scale());
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














