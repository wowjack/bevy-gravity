use gamelib::{G, math::*, gravity_system_tree::{builder::*, dynamic_body::*, static_body::*, system_tree::*, system_manager::*}};
use bevy::math::DVec2;
use bevy::color::palettes::css::{CORNFLOWER_BLUE, GREEN, PURPLE, WHITE, YELLOW};
use bevy::color::Color;

fn main() {
    let galaxy_mu = 1e33/100.;
    let galaxy_system_radius = 1e20;
    let galaxy_system_time_step = 100;
    let galaxy_radius = 1000.;
    let galaxy_color = Color::from(PURPLE);

    let stellar_orbital_radius = 1e12;
    let stellar_mu = 1.9891e30/100.;
    let stellar_system_radius = 5e9;
    let stellar_system_time_step = 10;
    let stellar_radius = 100.;
    let stellar_color = Color::from(YELLOW);

    let planet_orbital_radius = 1.5135e8;
    let planet_mu = 5.972e24/100.;
    let planet_system_radius = 5e6;
    let planet_system_time_step = 1;
    let planet_radius = 10.;
    let planet_color = Color::from(GREEN);

    let moon_orbital_radius = 384_400.;
    let moon_mu = 7.35e22/100.;
    let moon_radius = 1.;
    let moon_color = Color::from(WHITE);


    let planet_system = GravitySystemBuilder::new()
        .with_radius(planet_system_radius)
        .with_position(StaticPosition::Circular { radius: planet_orbital_radius, speed: get_orbital_speed(stellar_mu, planet_orbital_radius), start_angle: 0. })
        .with_time_step(planet_system_time_step)
        .with_static_bodies(&[
            StaticBody::new(StaticPosition::Still, planet_mu, planet_radius, planet_color, "".into()),
            StaticBody::new(StaticPosition::Circular { radius: moon_orbital_radius, speed: get_orbital_speed(planet_mu, moon_orbital_radius), start_angle: 0. }, moon_mu, moon_radius, moon_color, "".into()),
        ])
        .with_dynamic_bodies(&[
            DynamicBody::new(DVec2::X*-100_000., 1.3*DVec2::Y*get_orbital_speed(planet_mu, 100_000.)*100_000., 1e-20, 1., CORNFLOWER_BLUE.into(), "".into()),
        ]);
    let stellar_system = GravitySystemBuilder::new()
        .with_radius(stellar_system_radius)
        .with_position(StaticPosition::Circular { radius: stellar_orbital_radius, speed: get_orbital_speed(galaxy_mu, stellar_orbital_radius), start_angle: 0. })
        .with_time_step(stellar_system_time_step)
        .with_static_bodies(&[
            StaticBody::new(StaticPosition::Still, stellar_mu, stellar_radius, stellar_color, "".into()),
        ])
        .with_children(&[planet_system]);
    let galactic_system = GravitySystemBuilder::new()
        .with_radius(galaxy_system_radius)
        .with_position(StaticPosition::Still)
        .with_time_step(galaxy_system_time_step)
        .with_static_bodies(&[
            StaticBody::new(StaticPosition::Still, galaxy_mu, galaxy_radius, galaxy_color, "".into())
        ])
        .with_children(&[stellar_system]);


    let mut system = GravitySystemManager::new(galactic_system);
    for i in 1..1_000_000 {
        system.step();
    }
    assert_eq!(system.body_store.dynamic_bodies.len(), 1);
}
