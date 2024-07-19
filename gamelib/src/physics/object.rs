use bevy::math::DVec2;

use self::visual_object::VisualObjectData;

use super::*;

#[derive(Debug, Clone)]
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

impl From<VisualObjectData> for MassiveObject {
    fn from(value: VisualObjectData) -> Self {
        Self { position: value.position, velocity: value.velocity, mass: value.mass }
    }
}