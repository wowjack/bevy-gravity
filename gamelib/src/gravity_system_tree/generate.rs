use bevy::color::palettes::css::WHITE;
use rand::Rng;
use rand::{rngs::StdRng, SeedableRng};
use crate::math::get_orbital_speed;

use super::static_body::{StaticBody, StaticPosition};
use super::builder::GravitySystemBuilder;

/*
Stars will orbit the galaxy center every one to four hours
Planets:
    mass: 1e24 to
Moons will orbit the planet every 30 seconds to 2 minutes
*/

/*
const ONE_HOUR_SPEED: f64 = std::f64::consts::TAU/(60.*60.*60.);
const ONE_MINUTE_SPEED: f64 = std::f64::consts::TAU/(60.*60.);


fn generate_galaxy(rng: &mut StdRng) -> GravitySystemBuilder {
    //gives an orbit time of a bit under two hours
    let center_mass = 3e20;
    let child_orbit_radius = 1e10;
    let child_speed = get_orbital_speed(center_mass, child_orbit_radius)/child_orbit_radius;

    let galaxy_center = GravitySystemBuilder::new()
        .with_radius(1e6)
        .with_position(StaticPosition::Still)
        .with_time_step(10)
        .with_static_bodies(&[
            StaticBody::new(StaticPosition::Still, center_mass, 1e6, WHITE.into())
        ]);
    GravitySystemBuilder::new()
        .with_radius(1e100)
        .with_time_step(1000)
        .with_position(StaticPosition::Still)
        .with_children(&[
            galaxy_center,
            generate_medium_star_system(rng, StaticPosition::Circular { radius: child_orbit_radius, speed: child_speed, start_angle: 0. })
        ])
}

fn generate_large_star_system(rng: &mut StdRng, position: StaticPosition) -> GravitySystemBuilder {
    let system_radius = 1e7;
    todo!()
}
fn generate_medium_star_system(rng: &mut StdRng, position: StaticPosition) -> GravitySystemBuilder {
    let system_radius = 1e7;
    todo!()
}
fn generate_small_star_system(rng: &mut StdRng, position: StaticPosition) -> GravitySystemBuilder {
    let system_radius = 1e7;
    todo!()
}

fn generate_planet_system(rng: &mut StdRng, position: StaticPosition) -> GravitySystemBuilder {
    let num_moons = rng.gen_range(1usize..10);
    // moon orbit time range = MINUTE_TIME_SPEED
    let planet_mass = rng.gen_range(1e24..5e25)*(num_moons as f64).powi(2);
    GravitySystemBuilder::new();
    todo!()
}


*/