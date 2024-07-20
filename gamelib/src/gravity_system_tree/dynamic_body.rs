// Bodies that do not affect gravity but are affected by it

use super::massive_object::MassiveObject;
use bevy::prelude::Entity;
use bevy::math::DVec2;


#[derive(Clone, Debug, PartialEq)]
pub struct DynamicBody {
    position: DVec2,
    velocity: DVec2,
    mass: f64, // maybe gravitational parameter
    entity: Option<Entity> // Index into flat vec of bodies?
}
impl DynamicBody {
    pub fn new(position: DVec2, velocity: DVec2, mass: f64, entity: Option<Entity>) -> Self {
        Self { position, velocity, mass, entity }
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
        self.mass
    }
    pub fn set_mass(&mut self, mass: f64) {
        self.mass = mass;
    }
    pub fn get_entity(&self) -> Option<Entity> {
        self.entity
    }
    pub fn set_entity(&mut self, entity: Option<Entity>) {
        self.entity = entity;
    }
    pub fn force_scalar(&self, position: DVec2, mass: f64) -> DVec2 {
        let dir = position - self.position;
        let norm = dir.length_squared();

        // Branch removed by the compiler when `CHECK_ZERO` is false.
        if norm == 0. {
            dir
        } else {
            dir * (mass / (norm * norm.sqrt()))
        }
    }
}

impl Into<MassiveObject> for DynamicBody {
    fn into(self) -> MassiveObject {
        MassiveObject { 
            position: bevy::math::DVec2::new(self.position.x, self.position.y),
            velocity: bevy::math::DVec2::new(self.velocity.x, self.velocity.y),
            mass: self.mass
        }
    }
}