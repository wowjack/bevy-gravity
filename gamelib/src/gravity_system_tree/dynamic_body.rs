// Bodies that do not affect gravity but are affected by it

use crate::G;

use super::massive_object::MassiveObject;
use super::position_generator::PositionGenerator;
use bevy::prelude::Entity;
use bevy::math::DVec2;


#[derive(Clone, Debug, PartialEq)]
pub struct DynamicBody {
    position: DVec2,
    velocity: DVec2,
    /// Gravitational parameter (mass * G)
    mu: f64,
    radius: f64,
    entity: Option<Entity> // Index into flat vec of bodies?
}
impl DynamicBody {
    pub fn new(position: DVec2, velocity: DVec2, mu: f64, radius: f64, entity: Option<Entity>) -> Self {
        Self { position, velocity, mu, radius, entity }
    }

    pub fn position(&self) -> DVec2 {
        self.position
    }
    pub fn set_position(&mut self, position: DVec2) {
        self.position = position;
    }
    pub fn velocity(&self) -> DVec2 {
        self.velocity
    }
    pub fn set_velocity(&mut self, velocity: DVec2) {
        self.velocity = velocity;
    }
    pub fn mass(&self) -> f64 {
        self.mu / G
    }
    pub fn mu(&self) -> f64 {
        self.mu
    }
    pub fn set_mu(&mut self, mu: f64) {
        self.mu = mu;
    }
    pub fn radius(&self) -> f64 {
        self.radius
    }
    pub fn get_entity(&self) -> Option<Entity> {
        self.entity
    }
    pub fn set_entity(&mut self, entity: Option<Entity>) {
        self.entity = entity;
    }
    pub fn force_scalar(&self, position: DVec2, mu: f64) -> DVec2 {
        let dir = position - self.position;
        let norm = dir.length_squared();

        // Branch removed by the compiler when `CHECK_ZERO` is false.
        if norm == 0. {
            dir
        } else {
            dir * (mu / (norm * norm.sqrt()))
        }
    }

    pub fn fast_forward(mut self, current_time: u64, next_time: u64) -> Self {
        self.position += self.velocity * (next_time - current_time) as f64;
        self
    }

    pub fn make_absolute(mut self, generator: &PositionGenerator, time: u64, time_step: u64) -> Self {
        let position = generator.get(time);
        let velocity = generator.get(time+time_step);
        self.position += position;
        self.velocity += velocity;
        self
    }
}

impl Into<MassiveObject> for DynamicBody {
    fn into(self) -> MassiveObject {
        MassiveObject { 
            position: bevy::math::DVec2::new(self.position.x, self.position.y),
            velocity: bevy::math::DVec2::new(self.velocity.x, self.velocity.y),
            mass: self.mu / G
        }
    }
}