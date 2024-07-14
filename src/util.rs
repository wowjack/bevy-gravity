use std::{fs, ops::Range};
use bevy::{color::Color, math::DVec2};
use bincode::deserialize;
use rand::{rngs::ThreadRng, Rng};

use crate::{physics::{G, TIME_STEP}, visual_object::VisualObjectData};

pub fn load_from_file() -> Result<(), ()> {
    let Ok(data) = fs::read("./save.dat") else { return Err(()) };

    let Ok(data) = deserialize::<Vec<u8>>(&data[..]) else { return Err(()) };

    Err(())
}

pub fn save_to_file(data: Vec<u8>) -> Result<(), ()> {
    todo!();
}

/// Maximum depth 6?
/// Mass decreases by a factor of 100 to 10_000?
/// period decreases by a factor of 10? 100?
pub fn generate_system() -> Vec<VisualObjectData> {
    let mut bodies: Vec<VisualObjectData> = vec![];
    let masses = [1e36, 1e30, 1e24, 1e20]; 
    let periods = [100_000_000.0f32, 1_000_000., 1_000., 10.];
    let radii = [100_000., 10_000., 1_000., 10.];
    let num_bodies = [10usize, 6, 2];
    let center_body = VisualObjectData { position: DVec2::ZERO, velocity: DVec2::ZERO, mass: masses[0], radius: radii[0] as f32, color: Color::WHITE };
    bodies.push(center_body.clone());
    generate_system_recursive(1, &mut bodies, center_body, &periods, &num_bodies, &masses);
    return bodies;
}

fn generate_system_recursive(index: usize, objects: &mut Vec<VisualObjectData>, center: VisualObjectData, periods: &[f32; 4], num_bodies: &[usize; 3], masses: &[f64; 4]) {
    if index == 4 { return }
    let mut rng = rand::thread_rng();
    let direction = rng.gen_bool(0.5);
    for _ in 0..num_bodies[index-1] {
        let mut new_period = periods[index];
        new_period += new_period * if rng.gen_bool(0.5) {-0.3} else {0.3};
        let mut new_mass = masses[index];
        new_mass += new_mass * if rng.gen_bool(0.5) {-0.3} else {0.3};
        let new_object = get_orbital_body(
            &mut rng,
            center.clone(),
            new_period,
            new_mass,
            direction
        );
        objects.push(new_object.clone());
        generate_system_recursive(index+1, objects, new_object, periods, num_bodies, masses);
    }
}


pub fn get_orbital_body(rng: &mut ThreadRng, object_data: VisualObjectData, period: f32, mass: f64, orbit_direction: bool) -> VisualObjectData {
    let distance = (object_data.mass*G).powf(1./3.) * period.powf(2./3.) as f64;
    let angle = rng.gen_range(0.0..(std::f64::consts::PI*2.0));
    let position = object_data.position + DVec2::from_angle(angle)*distance;
    let speed = (G * object_data.mass / distance).sqrt();
    let vector = DVec2::from_angle(angle + (std::f64::consts::FRAC_PI_2 * if orbit_direction {1.} else {-1.}));
    let velocity = vector.normalize() * speed + object_data.velocity;
    VisualObjectData {
        position,
        velocity,
        mass: mass,
        radius: object_data.radius / 10.,
        color: Color::linear_rgb(1., 1., 1.),
    }
}