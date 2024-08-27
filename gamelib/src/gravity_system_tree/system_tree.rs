use core::f64;
use std::collections::VecDeque;

use bevy::{color::Color, math::DVec2, prelude::{Commands, Entity, Query}};

use crate::visual_object::{VisualObjectBundle, VisualObjectData};

use super::{builder::GravitySystemBuilder, dynamic_body::DynamicBody, static_body::{StaticBody, StaticPosition}};

pub type DiscreteGravitySystemTime = u64;
pub type GravitySystemTime = f64;



#[derive(Clone)]
pub struct GravitySystemTree {
    /// Index into dynamic body array where the body can be found
    pub dynamic_body_indices: Vec<usize>,
    /// Lone bodies in the system. \
    /// This should really only be used for the leaf nodes of the tree and the center body of systems.
    pub static_body_indices: Vec<usize>,
    pub child_systems: Vec<GravitySystemTree>,
    /// Child system masses come first then static bodies \
    /// Used to reduce allocations and the number of times the static position of systems and bodies is calculated each iteration
    pub static_masses: Vec<(DVec2, f64)>, 
    /// Gravitational acceleration will only be updated if new_time % time_step == 0
    pub time_step: u64,
    /// The size of the entire system. \
    /// If a dynamic body is within a distance of radius from the system center, it is part of the system or one of its children. 
    pub radius: f64,
    /// Used to calculate the position of the system at a point in time
    pub position: StaticPosition,
    /// Total gravitational parameter of all static bodies in the system, including bodies in child systems. Mass of dynamic bodies is negligible.
    pub mu: f64,
    /// Total number of dynamic bodies that exist under this system. \
    pub total_child_dynamic_bodies: usize,
}
impl GravitySystemTree {
    /// Updates dynamic bodies 
    fn calculate_gravity(
        &mut self,
        current_time: GravitySystemTime,
        static_body_vec: &Vec<StaticBody>,
        dynamic_body_vec: &mut Vec<DynamicBody>,
    ) {
        self.update_static_masses(static_body_vec, current_time);
        for index in &self.dynamic_body_indices {
            let body = unsafe { dynamic_body_vec.get_unchecked_mut(*index) };
            let acceleration = self.static_masses.iter().fold(DVec2::ZERO, |acceleration, static_mass| { acceleration + body.force_scalar(static_mass.0, static_mass.1) });
            body.gravitational_acceleration = acceleration;
        }
    }

    pub fn move_dynamic_bodies(&mut self, new_time: DiscreteGravitySystemTime, body_vec: &mut Vec<DynamicBody>, parent_pos: DVec2, parent_vel: DVec2, step: u64) {
        for index in &self.dynamic_body_indices {
            let body = unsafe { body_vec.get_unchecked_mut(*index) };
            let mut acceleration = body.gravitational_acceleration;
            ////////////////////////////////////////////////////////////
            // This is where I will get acceleration from future actions
            loop {
                match body.future_actions.front() {
                    None => break,
                    Some((time, accel)) => {
                        if *time > new_time-1 { break }
                        if *time < new_time-1 {
                            body.future_actions.pop_front();
                            continue;
                        }
                        acceleration += *accel;
                        body.future_actions.pop_front();
                    }
                }
            }
            ////////////////////////////////////////////////////////////
                        
            let new_velocity = body.stats.current_relative_velocity + acceleration;
            let old_position = body.stats.current_relative_position;
            let new_position = old_position + new_velocity;

            // Only rotate and scale gravitational acceleration vector if gravity will not be recalculated on the next time step
            if step+1 == self.time_step {
                // Get the scalar change in distance to the system center squared
                // This is used to scale the acceleration vector as a body changes distance from the system center within iterations in a system's time_step
                // Without this, elliptical orbits decay into circular ones
                let distance_diff = old_position.length_squared() / new_position.length_squared();
                body.gravitational_acceleration = DVec2::from_angle(old_position.angle_between(new_position)).rotate(body.gravitational_acceleration)*distance_diff;
            }

            body.stats.set_relative_velocity(new_velocity);
            body.stats.set_absolute_velocity(body.stats.current_relative_velocity+parent_vel);

            body.stats.set_relative_position(new_position);
            body.stats.set_absolute_position(body.stats.current_relative_position+parent_pos);
        }
    }

    fn ascend_or_descend_bodies(&mut self, new_time: GravitySystemTime, bodies_vec: &mut Vec<DynamicBody>, elevator: &mut Vec<usize>) {
        let mut remove_list = vec![];

        for (index, body_index) in self.dynamic_body_indices.iter().enumerate() {
            let body_mut = unsafe { bodies_vec.get_unchecked_mut(*body_index) };
            if body_mut.stats.current_relative_position.length_squared() > self.radius.powi(2) {
                body_mut.stats.translate_to_parent(new_time, &self.position);
                elevator.push(*body_index);
                remove_list.push(index);
                self.total_child_dynamic_bodies -= 1;
                continue;
            }
            for child_system in &mut self.child_systems {
                let system_position = child_system.position.get_cartesian_position(new_time as f64);
                if body_mut.stats.current_relative_position.distance_squared(system_position) > child_system.radius.powi(2) { continue }
                body_mut.stats.translate_to_child(new_time, &child_system.position);
                child_system.insert_body_index(*body_index);
                remove_list.push(index);
                break;
            }
        }
        for index in remove_list {
            self.dynamic_body_indices.swap_remove(index);
        }
    }

    fn insert_body_index(&mut self, index: usize) {
        self.total_child_dynamic_bodies += 1;
        self.dynamic_body_indices.push(index);
    }

    /// Clear then populate the static_masses vec of the system using the provided time
    #[inline]
    fn update_static_masses(&mut self, body_vec: &Vec<StaticBody>, time: GravitySystemTime) {
        self.static_masses.clear();
        for child_system in &self.child_systems {
            child_system.position.get_cartesian_position(time);
        }
        for body_index in &self.static_body_indices {
            let body = unsafe { body_vec.get_unchecked(*body_index) };
            self.static_masses.push((body.static_position.get_cartesian_position(time), body.mu));
        }
    }
}


/// Used to keep track of dynamic and static bodies and their associated entities
#[derive(Default, Debug, Clone)]
pub struct BodyStore {
    dynamic_bodies: Vec<DynamicBody>,
    dynamic_entities: Vec<Entity>,

    static_bodies: Vec<StaticBody>,
    static_entities: Vec<Entity>,
}
impl BodyStore {
    /// Performs one time step of gravity calculation \
    /// Note that this does not update all the static bodies in the body store. This method only updates static bodies when needed to calculate gravity. \
    /// This method assumes that the current position and velocity of dynamic bodies is new_time-1 \
    pub fn update_dynamic_bodies(&mut self, system_tree: &mut GravitySystemTree, new_time: DiscreteGravitySystemTime) {
        self.update_dynamic_bodies_recursive(system_tree, new_time, DVec2::ZERO, DVec2::ZERO, &mut vec![]);
    }
    pub fn update_dynamic_bodies_recursive(
        &mut self,
        system_tree: &mut GravitySystemTree,
        new_time: DiscreteGravitySystemTime,
        parent_pos: DVec2,
        parent_vel: DVec2,
        elevator: &mut Vec<usize>,
    ) {
        let new_ftime = new_time as GravitySystemTime;
        if system_tree.dynamic_body_indices.len() != 0 {
            let step = new_time % system_tree.time_step;
            if step == 0 {
                system_tree.calculate_gravity(new_ftime-1., &self.static_bodies, &mut self.dynamic_bodies);
            }
            system_tree.move_dynamic_bodies(new_time, &mut self.dynamic_bodies, parent_pos, parent_vel, step);
        } 
        

        let mut child_elevator = vec![];
        for child_system in &mut system_tree.child_systems {
            if child_system.total_child_dynamic_bodies < 1 { continue }
            let (child_pos, child_vel) = child_system.position.get_position_and_velocity(new_ftime); // Should current time or new time be used here? I think new time since its used to set absolute position of dynamic bodies
            self.update_dynamic_bodies_recursive(
                child_system,
                new_time,
                parent_pos+child_pos,
                parent_vel+child_vel,
                &mut child_elevator
            )
        }
        system_tree.dynamic_body_indices.extend_from_slice(&child_elevator);

        system_tree.ascend_or_descend_bodies(new_ftime, &mut self.dynamic_bodies, elevator);
    }




    /// Recurse through the tree and set the position and velocity of all static bodies. \
    /// Only use this before updating visual objects. \
    /// Static bodies are only updated selectively when calculating gravity. \
    pub fn update_static_bodies(&mut self, system_tree: &GravitySystemTree, time: GravitySystemTime) {
        self.update_static_bodies_recursive(system_tree, time as f64, (DVec2::ZERO, DVec2::ZERO));
    }
    fn update_static_bodies_recursive(&mut self, system_tree: &GravitySystemTree, time: GravitySystemTime, parent_stats: (DVec2, DVec2)) {
        for i in &system_tree.static_body_indices {
            let static_body = unsafe { self.static_bodies.get_unchecked_mut(*i) };
            let (pos, vel) = static_body.static_position.get_position_and_velocity(time);
            static_body.stats.current_relative_position = pos;
            static_body.stats.previous_relative_position = pos;
            static_body.stats.current_relative_velocity = vel;
            static_body.stats.previous_relative_velocity = vel;
            static_body.stats.current_absolute_position = pos + parent_stats.0;
            static_body.stats.previous_absolute_position = pos + parent_stats.0;
            static_body.stats.current_absolute_velocity = vel + parent_stats.1;
            static_body.stats.previous_absolute_velocity = vel + parent_stats.1;
        }
        for child_system in &system_tree.child_systems {
            let child_stats = child_system.position.get_position_and_velocity(time);
            self.update_static_bodies_recursive(child_system, time, (parent_stats.0+child_stats.0, parent_stats.1+child_stats.1));
        }
    }
    /// Updates visual objects in the query based on the values currently in the body store. \
    /// You probably want to call update_static_bodies with your requested time before calling this.
    pub fn update_visual_objects(&self, object_query: &mut Query<&mut VisualObjectData>, interpolation_factor: f64) {
        for (db, e) in self.dynamic_bodies.iter().zip(self.dynamic_entities.iter()) {
            let Ok(mut vo) = object_query.get_mut(*e) else { continue };
            vo.position = db.stats.get_interpolated_absolute_position(interpolation_factor);
            vo.velocity = db.stats.get_interpolated_relative_velocity(interpolation_factor);
        }
        for (sb, e) in self.static_bodies.iter().zip(self.static_entities.iter()) {
            let Ok(mut vo) = object_query.get_mut(*e) else { continue };
            // I am not interpolating here since the update_static_bodies method sets the same value for current and previous
            vo.position = sb.stats.current_absolute_position; //get_interpolated_absolute_position(interpolation_factor);
            vo.velocity = sb.stats.current_relative_velocity; //get_interpolated_relative_velocity(interpolation_factor);
        }
    }




    /// Spawns visual objects using the dynamic and static bodies currently in the body store. \
    /// Populates the dynamic and static entities arrays with the visual objects that were just spawned. \
    pub fn spawn_visual_objects(&mut self, commands: &mut Commands) {
        self.dynamic_entities.clear();
        for db in &self.dynamic_bodies {
            let bundle = VisualObjectBundle::new(VisualObjectData::from_dynamic_body(db));
            let e = commands.spawn(bundle).id();
            self.dynamic_entities.push(e);
        }

        self.static_entities.clear();
        for sb in &self.static_bodies {
            let bundle = VisualObjectBundle::new(VisualObjectData::from_static_body(sb));
            let e = commands.spawn(bundle).id();
            self.static_entities.push(e);
        }
    }
    /// Insert a dynamic body into the store and return the index used to access it. \
    /// This is only used when building the system.
    pub fn add_dynamic_body_to_store(&mut self, body: DynamicBody) -> usize {
        self.dynamic_bodies.push(body);
        self.dynamic_bodies.len()-1
    }
    /// Insert a static body into the store and return the index used to access it. \
    /// This is only used when building the system. \
    /// THIS METHOD DOES NOT MUTATE THE SYSTEM TREE. If you want to add a brand new body to the system tree, you will need to consult the system manager methods.
    pub fn add_static_body_to_store(&mut self, body: StaticBody) -> usize {
        self.static_bodies.push(body);
        self.static_bodies.len()-1
    }
}




impl Default for GravitySystemTree {
    fn default() -> Self {
        Self {
            dynamic_body_indices: vec![],
            static_body_indices: vec![],
            child_systems: vec![],
            static_masses: vec![], 
            time_step: 1,
            radius: 1.,
            position: StaticPosition::Still,
            mu: 0.,
            total_child_dynamic_bodies: 0,
        }
    }
}