use super::*;

#[derive(Debug, Component, Clone)]
pub struct MassiveObject {
    pub position: DVec2,
    pub velocity: DVec2,
    pub mass: f64
}
impl Default for MassiveObject {
    fn default() -> Self { 
        Self {
            position: DVec2::ZERO, velocity: DVec2::ZERO, mass: 1.
        }
    }
}