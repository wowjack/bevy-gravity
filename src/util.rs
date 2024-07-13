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
pub fn generate_system(depth: usize, bodies_per_system: Range<usize>) -> Vec<VisualObjectData> {
    let mut bodies: Vec<VisualObjectData> = vec![];
    let start_mass = 1e3 * 1e5f64.powi(depth as i32);
    let start_period = 10. * 1000.0f32.powi(depth as i32-1);
    let center_body = VisualObjectData { position: DVec2::ZERO, velocity: DVec2::ZERO, mass: start_mass, radius: depth.pow(5) as f32, color: Color::WHITE };
    bodies.push(center_body.clone());
    generate_system_recursive(depth-1, &mut bodies, center_body, start_period, bodies_per_system);
    return bodies;
}

fn generate_system_recursive(depth: usize, objects: &mut Vec<VisualObjectData>, center: VisualObjectData, parent_period: f32, num_bodies_range: Range<usize>) {
    if depth == 0 { return }
    let mut rng = rand::thread_rng();
    let direction = rng.gen_bool(0.5);
    for _ in 0..=rng.gen_range(num_bodies_range.clone()) {
        let new_period = parent_period / 1000.0;
        let new_mass = center.mass / 10000.0;
        let new_object = get_orbital_body(
            &mut rng,
            center.clone(),
            (new_period-new_period*0.5)..(new_period+new_period*0.5),
            (new_mass-new_mass*0.5)..(new_mass+new_mass*0.5),
            direction
        );
        objects.push(new_object.clone());
        generate_system_recursive(depth-1, objects, new_object, new_period, num_bodies_range.clone())
    }
}


pub fn get_orbital_body(rng: &mut ThreadRng, object_data: VisualObjectData, period_range: Range<f32>, mass_range: Range<f64>, orbit_direction: bool) -> VisualObjectData {
    let period: f32 = rng.gen_range(period_range);
    let distance = (object_data.mass*G).powf(1./3.) * period.powf(2./3.) as f64;
    let angle = rng.gen_range(0.0..(std::f64::consts::PI*2.0));
    let position = object_data.position + DVec2::from_angle(angle)*distance;
    let speed = (G * object_data.mass / (distance * TIME_STEP)).sqrt();
    let vector = DVec2::from_angle(angle + (std::f64::consts::FRAC_PI_2 * if orbit_direction {1.} else {-1.}));
    let velocity = vector.normalize() * speed + object_data.velocity;
    VisualObjectData {
        position,
        velocity,
        mass: rng.gen_range(mass_range),
        radius: object_data.radius / 10.,
        color: Color::linear_rgb(1., 1., 1.),
    }
}