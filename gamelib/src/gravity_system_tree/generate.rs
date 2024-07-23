use rand::{rngs::StdRng, SeedableRng};
use super::static_body::{StaticBody, StaticPosition};
use super::builder::GravitySystemBuilder;



fn generate_galaxy(rng: &mut StdRng) -> GravitySystemBuilder {
    let galaxy_center = GravitySystemBuilder::new()
        .with_radius(1e6)
        .with_position(StaticPosition::Still)
        .with_time_step(10)
        .with_static_bodies(&[
            StaticBody::new(StaticPosition::Still, 1e13, 1e5, None)
        ]);
    GravitySystemBuilder::new()
        .with_radius(1e100)
        .with_time_step(1000)
        .with_position(StaticPosition::Still)
        .with_children(&[
            
        ])
}

fn generate_star_system(rng: &mut StdRng, galaxy_center_mass: Option<f64>) -> GravitySystemBuilder {
    todo!()
}

fn generate_planet_system(rng: &mut StdRng) -> GravitySystemBuilder {
    todo!()
}