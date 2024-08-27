use bevy::{color::Color, math::DVec2};

use super::dynamic_body::BodyStats;


#[derive(Clone, Debug)]
pub struct StaticBody {
    pub stats: BodyStats,
    pub static_position: StaticPosition,
    pub mu: f64,
    pub radius: f64,
    pub color: Color,
}
impl StaticBody {
    pub fn new(static_position: StaticPosition, mu: f64, radius: f64, color: Color) -> Self {
        Self {
            stats: BodyStats::default(),
            static_position,
            mu,
            radius,
            color
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
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
    pub fn get_polar_position(&self, time: f64) -> [f64;2] {
        match self {
            Self::Still => [0., 0.],
            Self::Circular { radius, speed, start_angle } => [*radius, (start_angle+speed*time)]
        }
    }
    pub fn get_radius(&self) -> f64 {
        match self {
            Self::Still => 0.,
            Self::Circular { radius, .. } => *radius
        }
    }


    /// Get cartesian coordinates at time t assuming the center of the orbit is (0, 0)
    pub fn get_cartesian_position(&self, time: f64) -> DVec2 {
        match self {
            Self::Still => DVec2::ZERO,
            Self::Circular { radius, speed, start_angle } => {
                let angle = start_angle+speed*time;
                DVec2 { x: radius*angle.cos(), y: radius*angle.sin() }
            }
        }
    }
    pub fn get_velocity(&self, time: f64) -> DVec2 {
        match self {
            Self::Still => DVec2::ZERO,
            Self::Circular { radius, speed, start_angle } => DVec2::from_angle(start_angle+speed*time + std::f64::consts::FRAC_PI_2) * (speed * radius)
        }
    }
    pub fn get_position_and_velocity(&self, time: f64) -> (DVec2, DVec2) {
        match self {
            Self::Still => (DVec2::ZERO, DVec2::ZERO),
            Self::Circular { radius, speed, start_angle } => {
                let angle = start_angle+speed*time;
                (
                    DVec2::new(radius*angle.cos(), radius*angle.sin()),
                    DVec2::from_angle(angle + std::f64::consts::FRAC_PI_2) * (speed * radius)
                )
            }
        }
    }
}