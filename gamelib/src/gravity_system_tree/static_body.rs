use bevy::{color::Color, math::DVec2};

use crate::G;

use super::{static_generator::StaticGenerator, system_tree::GravitySystemTime, BodyMass, BodyPosition, BodyRadius, BodyVelocity, GravitationalParameter};


#[derive(Clone, Debug)]
pub struct StaticBody {
    relative_position: BodyPosition,
    absolute_position: BodyPosition,
    relative_velocity: BodyVelocity,
    absolute_velocity: BodyVelocity,

    static_position: StaticPosition,
    parent_generator: StaticGenerator,
    system_depth: usize,

    mass: BodyMass,
    mu: GravitationalParameter,
    radius: BodyRadius,
    color: Color,
    name: String
}
impl StaticBody {
    pub fn new(static_position: StaticPosition, mass: BodyMass, radius: BodyRadius, color: Color, name: String) -> Self {
        let (position, velocity) = static_position.get_position_and_velocity(0.);
        Self {
            relative_position: position,
            absolute_position: position,
            relative_velocity: velocity,
            absolute_velocity: velocity,
            static_position,
            parent_generator: StaticGenerator::new(),
            system_depth: 0,
            mass,
            mu: mass*G,
            radius,
            color,
            name
        }
    }

    pub fn set_to_time_with_parent_stats(&mut self, time: GravitySystemTime, parent_stats: (BodyPosition, BodyVelocity)) {
        let (relative_position, relative_velocity) = self.static_position.get_position_and_velocity(time);
        self.relative_position = relative_position;
        self.relative_velocity = relative_velocity;

        self.absolute_position = relative_position + parent_stats.0;
        self.absolute_velocity = relative_velocity + parent_stats.1;
    }


    pub fn initialize_in_system_tree(&mut self, system_depth: usize, parent_generator: &StaticGenerator) {
        self.system_depth = system_depth;

        self.parent_generator = parent_generator.clone();
        let (parent_pos, parent_vel) = parent_generator.get_position_and_velocity(0.);

        self.absolute_position = parent_pos + self.relative_position;
        self.absolute_velocity = parent_vel + self.relative_velocity;
    }


    ////////////////////////////// GETTERS //////////////////////////////
    pub fn get_static_position(&self) -> &StaticPosition { &self.static_position }
    pub fn get_mu(&self) -> GravitationalParameter { self.mu }
    pub fn get_absolute_position(&self) -> BodyPosition { self.absolute_position }
    pub fn get_relative_position(&self) -> BodyPosition { self.relative_position }
    pub fn get_absolute_velocity(&self) -> BodyPosition { self.absolute_velocity }
    pub fn get_relative_velocity(&self) -> BodyPosition { self.relative_velocity }
    pub fn get_mass(&self) -> BodyMass { self.mass }
    pub fn get_radius(&self) -> BodyRadius { self.radius }
    pub fn get_color(&self) -> Color { self.color }
    pub fn get_name(&self) -> String { self.name.clone() }

    /// Get center position and radius of orbit
    pub fn get_orbit_parameters(&self, time: GravitySystemTime) -> (DVec2, f64) {
        match self.static_position {
            StaticPosition::Circular { radius, .. } => {
                return (self.parent_generator.get_position(time), radius)
            },
            StaticPosition::Still => {
                return (
                    self.parent_generator.get_position(time) - self.parent_generator.get_end().map_or(DVec2::ZERO, |x| x.get_position(time)),
                    self.parent_generator.get_end().map_or(0., |x| x.get_radius())
                )
            }
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
    pub fn get_position(&self, time: GravitySystemTime) -> BodyPosition {
        match self {
            Self::Still => DVec2::ZERO,
            Self::Circular { radius, speed, start_angle } => {
                let angle = start_angle+speed*time;
                DVec2 { x: radius*angle.cos(), y: radius*angle.sin() }
            }
        }
    }
    pub fn get_velocity(&self, time: GravitySystemTime) -> BodyVelocity {
        match self {
            Self::Still => DVec2::ZERO,
            Self::Circular { radius, speed, start_angle } => DVec2::from_angle(start_angle+speed*time + std::f64::consts::FRAC_PI_2) * (speed * radius)
        }
    }
    pub fn get_position_and_velocity(&self, time: GravitySystemTime) -> (BodyPosition, BodyVelocity) {
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