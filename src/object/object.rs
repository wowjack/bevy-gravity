use bevy::prelude::*;

/*
Spatial bundle at the top level
Mesh and arrow and such as children
*/


#[derive(Component)]
pub struct MassiveObject {
    pub velocity: Vec2,
    pub mass: f64,
}

impl Default for MassiveObject {
    fn default() -> Self {
        Self { velocity: Vec2::ZERO, mass: 1. }
    }
}