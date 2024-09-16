use core::f64;
use bevy::color::palettes::css::PURPLE;
use bevy::window::WindowResized;
use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*, render::camera::Viewport};
use bevy_egui::EguiPlugin;
use bevy_mod_picking::prelude::*;
use bevy_vector_shapes::Shape2dPlugin;
use gravity_system_tree::builder::GravitySystemBuilder;
use gravity_system_tree::static_body::{StaticBody, StaticPosition};
use math::get_orbital_speed;
use pseudo_camera::camera::CameraState;
use pseudo_camera::pseudo_camera_plugin;
use gravity_system_tree::system_manager::GravitySystemManager;
use ui::SIDE_PANEL_WIDTH;
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
mod path_calculator;


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
        ))
        .run();
}

pub const G: f64 = 6.6743015e-11;


fn init(
    mut commands: Commands
) {
    let galaxy_mass = 1e34;
    let galaxy_system_radius = 1e20;
    let galaxy_system_time_step = 1000;
    let galaxy_radius = 51.8e6;
    let galaxy_color = Color::from(PURPLE);
    let galaxy_name = "Galactic Center".to_string();

    let galactic_system = GravitySystemBuilder::new()
        .with_radius(galaxy_system_radius)
        .with_position(StaticPosition::Still)
        .with_time_step(galaxy_system_time_step)
        .with_static_bodies(&[
            StaticBody::new(StaticPosition::Still, galaxy_mass, galaxy_radius, galaxy_color, galaxy_name)
        ])
        .with_dynamic_bodies(&[
            //DynamicBody::new(DVec2::X*100_000_000., DVec2::Y*50_000., 1e-30, 1., CORNFLOWER_BLUE.into()),
        ])
        .with_children(&[
            solar_system().with_position(StaticPosition::Circular { radius: SUN_ORBITAL_RADIUS, speed: get_orbital_speed(galaxy_mass, SUN_ORBITAL_RADIUS), start_angle: 0. })
        ]);


    let mut manager = GravitySystemManager::new(solar_system());
    manager.spawn_bodies(&mut commands);
    commands.insert_resource(manager);
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














