use std::collections::VecDeque;

use bevy::{color::Color, math::DVec2};

use super::{static_body::StaticPosition, system_tree::{DiscreteGravitySystemTime, GravitySystemTime}};

/// Dynamic bodies will keep track of their previous and and current position for interpolation purposes
/// ABSOLUTE POSITIONS MUST BE CALCULATED WHEN BUILDING THE SYSTEM BECAUSE THE STATIC POSITION CHAIN IS NOT WALKED AFTERWARDS
#[derive(Clone, Default, Debug)]
pub struct BodyStats {
    pub previous_relative_position: DVec2,
    pub current_relative_position: DVec2,
    pub previous_absolute_position: DVec2,
    pub current_absolute_position: DVec2,

    pub previous_relative_velocity: DVec2,
    pub current_relative_velocity: DVec2,
    pub previous_absolute_velocity: DVec2,
    pub current_absolute_velocity: DVec2,
}
impl BodyStats {
    pub fn new(
        parent_absolute_position: DVec2,
        relative_position: DVec2,
        parent_absolute_velocity: DVec2,
        relative_velocity: DVec2,
    ) -> Self {
        Self {
            previous_relative_position: relative_position,
            current_relative_position: relative_position,
            previous_absolute_position: parent_absolute_position+relative_position,
            current_absolute_position: parent_absolute_position+relative_position,
            previous_relative_velocity: relative_velocity,
            current_relative_velocity: relative_velocity,
            previous_absolute_velocity: parent_absolute_velocity+relative_velocity,
            current_absolute_velocity: parent_absolute_velocity+relative_velocity
        }
    }

    pub fn get_interpolated_relative_position(&self, factor: f64) -> DVec2 {
        self.previous_relative_position + ((self.current_relative_position-self.previous_relative_position)*factor)
    }
    pub fn get_interpolated_absolute_position(&self, factor: f64) -> DVec2 {
        self.previous_absolute_position + ((self.current_absolute_position-self.previous_absolute_position)*factor)
    }

    pub fn get_interpolated_relative_velocity(&self, factor: f64) -> DVec2 {
        self.previous_relative_velocity + ((self.current_relative_velocity-self.previous_relative_velocity)*factor)
    }
    pub fn get_interpolated_absolute_velocity(&self, factor: f64) -> DVec2 {
        self.previous_absolute_velocity + ((self.current_absolute_velocity-self.previous_absolute_velocity)*factor)
    }

    pub fn translate_to_parent(&mut self, time: GravitySystemTime, parent_position: &StaticPosition) {
        let parent_pos = parent_position.get_cartesian_position(time);
        self.current_relative_position += parent_pos;
        self.previous_relative_position += parent_pos;

        let parent_vel = parent_position.get_velocity(time);
        self.current_relative_velocity += parent_vel;
        self.previous_relative_velocity += parent_vel;
    }
    pub fn translate_to_child(&mut self, time: GravitySystemTime, child_position: &StaticPosition) {
        let child_pos = child_position.get_cartesian_position(time);
        self.current_relative_position -= child_position.get_cartesian_position(time);
        self.previous_relative_position -= child_pos;

        let child_vel = child_position.get_velocity(time);
        self.current_relative_velocity -= child_position.get_velocity(time);
        self.previous_relative_velocity -= child_vel;
    }

    pub fn set_relative_position(&mut self, new_position: DVec2) {
        self.previous_relative_position = self.current_relative_position;
        self.current_relative_position = new_position;
    }
    pub fn set_absolute_position(&mut self, new_position: DVec2) {
        self.previous_absolute_position = self.current_absolute_position;
        self.current_absolute_position = new_position;
    }

    pub fn set_relative_velocity(&mut self, new_velocity: DVec2) {
        self.previous_relative_velocity = self.current_relative_velocity;
        self.current_relative_velocity = new_velocity;
    }
    pub fn set_absolute_velocity(&mut self, new_velocity: DVec2) {
        self.previous_absolute_velocity = self.current_absolute_velocity;
        self.current_absolute_velocity = new_velocity;
    }
}
#[derive(Clone, Debug)]
pub struct DynamicBody {
    pub stats: BodyStats,
    pub mu: f64,
    pub radius: f64,
    pub color: Color,
    pub gravitational_acceleration: DVec2,
    pub future_actions: VecDeque<(DiscreteGravitySystemTime, DVec2)>,
}
impl DynamicBody {
    pub fn new(position: DVec2, velocity: DVec2, mu: f64, radius: f64, color: Color) -> Self {
        Self {
            stats: BodyStats {
                previous_relative_position: position,
                current_relative_position: position,
                previous_absolute_position: position,
                current_absolute_position: position,
                previous_relative_velocity: velocity,
                current_relative_velocity: velocity,
                previous_absolute_velocity: velocity,
                current_absolute_velocity: velocity
            },
            mu,
            radius,
            color,
            gravitational_acceleration: DVec2::ZERO,
            future_actions: VecDeque::new()
        }
    }

    pub fn force_scalar(&self, position: DVec2, mu: f64) -> DVec2 {
        let dir = position - self.stats.current_relative_position;
        let norm = dir.length_squared();

        if norm == 0. {
            dir
        } else {
            dir * (mu / (norm * norm.sqrt()))
        }
    }
}