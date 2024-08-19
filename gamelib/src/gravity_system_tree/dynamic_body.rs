use std::collections::VecDeque;

use bevy::math::DVec2;

use super::{position_generator::PositionGenerator, static_body::StaticPosition};

#[derive(Clone, PartialEq)]
pub struct DynamicBody {
    /// Position and velocity
    pub relative_stats: DynamicBodyRelativeStats,
    /// Gravitational parameter (mass * G)
    pub mu: f64,
    pub radius: f64,

    /// Acceleration due to gravity \
    /// Pretty sure this acceleration is not relative
    pub gravitational_acceleration: DVec2,

    /// Actions that the body will make in the future like accelerating
    /// TODO!
    pub future_actions: VecDeque<()>,
}
impl DynamicBody {
    pub fn force_scalar(&self, position: DVec2, mu: f64) -> DVec2 {
        let dir = position - self.relative_stats.get_position_relative();
        let norm = dir.length_squared();

        if norm == 0. {
            dir
        } else {
            dir * (mu / (norm * norm.sqrt()))
        }
    }
    pub fn new(position: DVec2, velocity: DVec2, mu: f64, radius: f64) -> Self {
        Self {
            relative_stats: DynamicBodyRelativeStats::new(position, velocity, PositionGenerator::default()),
            mu,
            radius,
            gravitational_acceleration: DVec2::ZERO,
            future_actions: VecDeque::new()
        }
    }
}





#[derive(Default, Clone, Debug, PartialEq)]
pub struct DynamicBodyRelativeStats {
    /// Position relative to the position generator
    position: DVec2,
    /// Velocity relative to the position generator
    velocity: DVec2,
    generator: PositionGenerator,
}
impl DynamicBodyRelativeStats {
    pub fn new(position: DVec2, velocity: DVec2, generator: PositionGenerator) -> Self { Self { position, velocity, generator } }

    /// Number of ancestors in the position generator chain \
    /// This is equivalent to the depth of the dynamic body in the tree \
    /// If num_ancestors == 0 then get_position_relative == get_position_absolute
    pub fn num_ancestors(&self) -> usize { self.generator.len() }

    pub fn get_position_relative(&self) -> DVec2 { self.position }
    /// Get the body position at time relative to an ancestor in the position generator \
    /// get_position_relative_to_ancestor(0, 0) == get_position_relative() \
    /// get_position_relative_to_ancestor(0, num_ancestors) == get_position_absolute()
    pub fn get_position_relative_to_ancestor(&self, time: u64, ancestor_level: usize) -> DVec2 {
        self.generator.get_partial_end(time, ancestor_level) + self.position
    }
    pub fn get_position_absolute(&self, time: u64) -> DVec2 { self.generator.get(time) + self.position }
    pub fn set_position_relative(&mut self, position: DVec2) { self.position = position }

    pub fn get_velocity_relative(&self) -> DVec2 { self.velocity }
    /// Get the body velocity at time relative to an ancestor in the position generator \
    /// get_velocity_relative_to_ancestor(0, 0) == get_velocity_relative() \
    /// get_velocity_relative_to_ancestor(0, num_ancestors) == get_velocity_absolute()
    pub fn get_velocity_relative_to_ancestor(&self, time: u64, ancestor_level: usize) -> DVec2 {
        self.generator.get_partial_end(time+1, ancestor_level) - self.generator.get_partial_end(time, ancestor_level) + self.velocity
    }
    pub fn get_velocity_absolute(&self, time: u64) -> DVec2 {
        self.generator.get(time+1) - self.generator.get(time) + self.velocity
    }
    pub fn set_velocity_relative(&mut self, velocity: DVec2) { self.velocity = velocity }

    pub fn get_generator(&self) -> PositionGenerator { self.generator.clone() }
    pub fn set_generator(&mut self, generator: PositionGenerator) { self.generator = generator }

    pub fn accelerate_and_move(&mut self, acceleration: DVec2) {
        self.velocity += acceleration;
        self.position += self.velocity;
    }

    /// Translate the position and velocity to be relative to the parent system and pop from the end of the generator
    pub fn move_to_parent(&mut self, time: u64) {
        self.velocity = self.get_velocity_relative_to_ancestor(time, 1);
        self.position = self.get_position_relative_to_ancestor(time, 1);
        self.generator.pop_end();
    }
    /// Translate the position and velocity to be relative to the child position and extend the generator
    pub fn move_to_child(&mut self, time: u64, child_position: StaticPosition) {
        let child_system_position = child_position.get_cartesian_position(time);
        self.velocity -= child_position.get_cartesian_position(time+1) - child_system_position;
        self.position -= child_system_position;
        self.generator = self.generator.clone().extend(child_position);
    }
}