use bevy::{color::Color, math::DVec2};
use crate::G;
use super::{future_actions::FutureActions, static_body::StaticPosition, static_generator::StaticGenerator, system_tree::{DiscreteGravitySystemTime, GravitySystemTime}, BodyAcceleration, BodyMass, BodyPosition, BodyRadius, BodyVelocity, GravitationalParameter};


/// A body that does not effect gravity but is effected by gravity
#[derive(Clone, Debug)]
pub struct DynamicBody {
    previous_relative_position: BodyPosition,
    current_relative_position: BodyPosition,
    previous_absolute_position: BodyPosition,
    current_absolute_position: BodyPosition,

    previous_relative_velocity: BodyVelocity,
    current_relative_velocity: BodyVelocity,
    previous_absolute_velocity: BodyVelocity,
    current_absolute_velocity: BodyVelocity,

    /// Static generator of the system that this body is currently in. \
    /// This should be used very sparingly since walking the position chain often is expensive. \
    parent_generator: StaticGenerator,
    /// How deep in the tree this body is. The length of the position chain is not reliable for this purpose.
    system_depth: usize,

    mass: BodyMass,
    mu: GravitationalParameter,
    radius: BodyRadius,
    color: Color,
    name: String,

    gravitational_acceleration: BodyAcceleration,
    future_actions: FutureActions,
}
impl DynamicBody {
    pub fn new(
        position: BodyPosition,
        velocity: BodyVelocity,
        mass: BodyMass,
        radius: BodyRadius,
        color: Color,
        name: String,
    ) -> Self {
        Self {
            previous_relative_position: position,
            current_relative_position: position,
            previous_absolute_position: position,
            current_absolute_position: position,
            previous_relative_velocity: velocity,
            current_relative_velocity: velocity,
            previous_absolute_velocity: velocity,
            current_absolute_velocity: velocity,

            parent_generator: StaticGenerator::new(),
            system_depth: 0,

            mass,
            mu: mass * G,
            radius,
            color,
            name,

            gravitational_acceleration: DVec2::ZERO,
            future_actions: FutureActions::new(),
        }
    }






    ////////////////////////////// SYSTEM TREE METHODS //////////////////////////////
    // These methods should only be used by the system tree to calculate acceleration and move bodies

    /// Calculate the gravitational acceleration of the body using its current position and the position and gravitational parameter of the provided masses
    pub fn calculate_gravitational_acceleration(&mut self, masses: &[(BodyPosition, GravitationalParameter)]) {
        let mut accel = DVec2::ZERO;
        let body_position = self.current_relative_position;
        for (static_position, static_mu) in masses {
            let dir = *static_position - body_position;
            let norm = dir.length_squared();
            accel += dir * (static_mu / (norm * norm.sqrt()));
        }
        self.gravitational_acceleration = accel
    }

    /// Use the body's gravitational acceleration, velocity, and future actions to advance position 
    pub fn accelerate_and_move_body(&mut self, new_time: DiscreteGravitySystemTime, should_rotate_acceleration_vector: bool, parent_pos: BodyPosition, parent_vel: BodyVelocity) {
        let acceleration = self.gravitational_acceleration + self.future_actions.get_acceleration(new_time-1, self.mass);

        self.previous_relative_velocity = self.current_relative_velocity;
        self.current_relative_velocity += acceleration;
        self.previous_relative_position = self.current_relative_position;
        self.current_relative_position += self.current_relative_velocity;

        self.previous_absolute_position = self.current_absolute_position;
        self.previous_absolute_velocity = self.current_absolute_velocity;
        self.current_absolute_position = self.current_relative_position + parent_pos;
        self.current_absolute_velocity = self.current_relative_velocity + parent_vel;

        // Only rotate and scale gravitational acceleration vector if gravity will not be recalculated on the next time step
        if should_rotate_acceleration_vector {
            // Get the scalar change in distance to the system center squared
            // This is used to scale the acceleration vector as a body changes distance from the system center within iterations in a system's time_step
            // Without this, elliptical orbits decay into circular ones
            let distance_diff = self.previous_relative_position.length_squared() / self.current_relative_position.length_squared();
            self.gravitational_acceleration = DVec2::from_angle(self.previous_relative_position.angle_between(self.current_relative_position)).rotate(self.gravitational_acceleration)*distance_diff;
        }
    }

    pub fn translate_to_parent(&mut self, time: GravitySystemTime) {
        let (parent_pos, parent_vel) = self.parent_generator.pop_end().get_position_and_velocity(time);
        self.current_relative_position += parent_pos;
        self.previous_relative_position += parent_pos;
        self.current_relative_velocity += parent_vel;
        self.previous_relative_velocity += parent_vel;
        self.system_depth -= 1;
    }
    pub fn translate_to_child(&mut self, time: GravitySystemTime, child_position: &StaticPosition) {
        let (child_pos, child_vel) = child_position.get_position_and_velocity(time);
        self.current_relative_position -= child_pos;
        self.previous_relative_position -= child_pos;
        self.current_relative_velocity -= child_vel;
        self.previous_relative_velocity -= child_vel;
        self.parent_generator.push_end(child_position.clone());
        self.system_depth += 1;
    }
    pub fn distance_squared(&self, other: BodyPosition) -> f64 {
        self.current_relative_position.distance_squared(other)
    }
    /// Get the distance to the parent system center squared
    pub fn relative_magnitude_squared(&self) -> f64 {
        self.current_relative_position.length_squared()
    }





    ////////////////////////////// WRITER METHODS //////////////////////////////
    // These methods should only be used when writing data to visual objects

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




    ////////////////////////////// BUILDER METHODS //////////////////////////////
    // These methods should only be used by the system builder to initialize bodies

    /// Set the body's system_depth and parent_generator, then modify absolute position and velocity to reflect the change
    pub fn initialize_in_system_tree(&mut self, system_depth: usize, parent_generator: &StaticGenerator) {
        self.system_depth = system_depth;

        self.parent_generator = parent_generator.clone();
        let (parent_pos, parent_vel) = parent_generator.get_position_and_velocity(0.);

        self.current_absolute_position = parent_pos + self.current_relative_position;
        self.previous_absolute_position = parent_pos + self.previous_relative_position;

        self.current_absolute_velocity = parent_vel + self.current_relative_velocity;
        self.previous_absolute_velocity = parent_vel + self.previous_relative_velocity;
    }





    ////////////////////////////// GETTERS //////////////////////////////
    pub fn get_mass(&self) -> BodyMass { self.mass }
    pub fn get_radius(&self) -> BodyRadius { self.radius }
    pub fn get_color(&self) -> Color { self.color }
    pub fn get_name(&self) -> String { self.name.clone() }
    pub fn get_system_depth(&self) -> usize { self.system_depth }
    pub fn get_parent_generator(&self) -> &StaticGenerator { &self.parent_generator }
    pub fn get_previous_relative_position(&self) -> BodyPosition { self.previous_relative_position }
}