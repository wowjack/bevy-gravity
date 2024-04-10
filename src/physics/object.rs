use super::*;

#[derive(Debug, Component, Default, Clone)]
pub struct MassiveObject {
    pub position: DVec2,
    pub velocity: DVec2,
    pub mass: f64
}