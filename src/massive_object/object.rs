use super::*;

#[derive(Component, Default, Clone)]
pub struct MassiveObject {
    position: DVec2,
    velocity: DVec2,
    mass: f64
}