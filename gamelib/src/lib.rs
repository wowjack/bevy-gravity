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
use solar_system::*;



mod ui;
mod visual_object;
mod pseudo_camera;
mod util;
pub mod math;
pub mod gravity_system_tree;
mod solar_system;


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


fn init(
    world: &mut World
) {
    let galaxy_mu = 1e34*G/1e5;
    let galaxy_system_radius = 1e20;
    let galaxy_system_time_step = 100;
    let galaxy_radius = 1000.;
    let galaxy_color = Color::from(PURPLE);

    //let galactic_system = GravitySystemBuilder::new()
    //    .with_radius(galaxy_system_radius)
    //    .with_position(StaticPosition::Still)
    //    .with_time_step(galaxy_system_time_step)
    //    .with_static_bodies(&[
    //        StaticBody::new(StaticPosition::Still, galaxy_mu, galaxy_radius, galaxy_color)
    //    ])
    //    .with_dynamic_bodies(&[
    //        DynamicBody::new(DVec2::X*100_000_000., DVec2::Y*50_000., 1e-30, 1., CORNFLOWER_BLUE.into()),
    //    ])
    //    .with_children(&[stellar_system]);


    let mut manager = GravitySystemManager::new(solar_system());
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














