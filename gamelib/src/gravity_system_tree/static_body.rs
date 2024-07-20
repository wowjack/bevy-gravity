// A Body that effects gravity but is not affected.
// Can follow a set path or be still

use bevy::{math::DVec2, prelude::Entity};
use crate::math;

#[derive(Debug, Clone)]
pub enum StaticPosition {
    /// A Body that remains motionless relative to the system, staying perfectly in the center
    Still,
    /// A body that exhibits a circular orbit around the system center
    Circular {
        radius: f64,
        /// radians per second
        speed: f64,
        start_angle: f64,
    },
    // elliptical orbits
    // Arbitrary orbits determined by a vec of positions?
}
impl StaticPosition {
    pub fn get_polar_position(&self, time: u64) -> [f64;2] {
        match self {
            Self::Still => [0., 0.],
            Self::Circular { radius, speed, start_angle } => [*radius, (start_angle+speed*time as f64) % std::f64::consts::TAU]
        }
    }
    pub fn get_radius(&self) -> f64 {
        match self {
            Self::Still => 0.,
            Self::Circular { radius, .. } => *radius
        }
    }


    /// Get cartesian coordinates at time t assuming the center of the orbit is (0, 0)
    pub fn get_cartesian_position(&self, time: u64) -> DVec2 {
        math::polar_to_cartesian(self.get_polar_position(time))
    }
}

#[derive(Debug, Clone)]
pub struct StaticBody {
    pub position: StaticPosition,
    pub mass: f64, //or gravitational parameter G*mass?
    pub radius: f64,
    pub entity: Option<Entity>
}

impl StaticBody {
    pub fn new(position: StaticPosition, mass: f64, radius: f64, entity: Option<Entity>) -> Self {
        Self { position, mass, radius, entity}
    }
}