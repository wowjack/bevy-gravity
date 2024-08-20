use std::{arch::x86_64, cell::RefCell, collections::VecDeque, rc::Rc};

use bevy::{math::DVec2, prelude::Entity};
use itertools::Itertools;

use super::{dynamic_body::DynamicBody, position_generator::PositionGenerator, static_body::{StaticBody, StaticPosition}};

#[derive(Clone)]
pub struct GravitySystemTree {
    /// References to dynamic bodies within this system \
    /// Most of the time only an immutable reference is required
    pub dynamic_bodies: Vec<Rc<RefCell<DynamicBody>>>,

    /// Lone bodies in the system. \
    /// This should really only be used for the leaf nodes of the tree and the center body of systems. (or some extremely massive single object like a black hole) \
    /// If you want something like a rogue planet orbiting the galactic center, a child system containing only one center body is better since any dynamic bodies that approach it will use a finer time scale.
    pub static_bodies: Vec<StaticBody>,
    pub child_systems: Vec<GravitySystemTree>,
    /// Used for the gravity calculation
    /// Child system masses come first then static bodies
    pub static_masses: Vec<(DVec2, f64)>, 

    /// Gravitational acceleration will only be updated if new_time % time_step == 0
    pub time_step: u64,

    /// The size of the entire system. \
    /// If a dynamic body is within a distance of radius from the system center, it is part of the system or one of its children. 
    pub radius: f64,

    /// Used to calculate the position of the system at a point in time
    pub position_generator: PositionGenerator,

    /// Total gravitational parameter of all static bodies in the system, including bodies in child systems. Mass of dynamic bodies is negligible.
    pub mu: f64,

    /// Total number of dynamic bodies that exist under this system. \
    pub total_child_dynamic_bodies: usize,
}
impl GravitySystemTree {
    /// new_time must be last_time+1 for the position of static bodies to be correct
    pub fn accelerate_and_move_bodies_recursive(&mut self, new_time: u64, elevator: &mut Vec<Rc<RefCell<DynamicBody>>>) {
        // calculate acceleration if needed
        if new_time % self.time_step == 0 {
            self.set_static_masses_to(new_time-1);
            self.calculate_accelerations();
        }
        
        self.accelerate_and_move_bodies();

        let mut child_elevator = vec![];
        for child_system in &mut self.child_systems {
            if child_system.total_child_dynamic_bodies < 1 { continue }
            child_system.accelerate_and_move_bodies_recursive(new_time, &mut child_elevator);
        }
        // Process bodies coming up in the elevator
        self.dynamic_bodies.extend(child_elevator.into_iter());

        self.ascend_or_descend_bodies(new_time, elevator);
    }

    fn calculate_accelerations(&self) {
        for body in &self.dynamic_bodies {
            let mut body = body.borrow_mut();
            let acceleration = self.static_masses.iter().fold(DVec2::ZERO, |acceleration, static_mass| { acceleration + body.force_scalar(static_mass.0, static_mass.1) });
            body.gravitational_acceleration = acceleration;
        }
    }

    fn accelerate_and_move_bodies(&self) {
        for body in &self.dynamic_bodies {
            let mut body = body.borrow_mut();
            let acceleration = body.gravitational_acceleration;
            ////////////////////////////////////////////////////////////
            // This is where I will get acceleration from future actions
            ////////////////////////////////////////////////////////////
            let new_velocity = body.relative_stats.get_velocity_relative() + acceleration;
            let old_position = body.relative_stats.get_position_relative();
            let new_position = old_position + new_velocity;

            // Get the scalar change in distance to the system center squared
            // This is used to scale the acceleration vector as a body changes distance from the system center within iterations in a system's time_step
            // Without this, elliptical orbits decay into circular ones
            let distance_diff = old_position.length_squared() / new_position.length_squared();

            body.gravitational_acceleration = DVec2::from_angle(old_position.angle_between(new_position)).rotate(body.gravitational_acceleration)*distance_diff;

            body.relative_stats.set_velocity_relative(new_velocity);
            body.relative_stats.set_position_relative(new_position);
        }
    }

    fn set_static_masses_to(&mut self, time: u64) {
        self.static_masses.clear();
        for s in &self.child_systems {
            self.static_masses.push((s.position_generator.get_partial_end(time, 1), s.mu));
        }
        for sb in &self.static_bodies {
            self.static_masses.push((sb.position_generator.get_partial_end(time, 1), sb.mu));
        }
    }

    fn ascend_or_descend_bodies(&mut self, new_time: u64, elevator: &mut Vec<Rc<RefCell<DynamicBody>>>) {
        let mut remove_list = vec![];
        for (index, body) in self.dynamic_bodies.iter().enumerate() {
            let mut body_mut = body.borrow_mut();
            if body_mut.relative_stats.get_position_relative().length_squared() > self.radius.powi(2) {
                body_mut.relative_stats.translate_to_parent(new_time);
                elevator.push(body.clone());
                remove_list.push(index);
                continue;
            }
            for child_system in &mut self.child_systems {
                let system_position = child_system.position_generator.get_partial_end(new_time, 1);
                if body_mut.relative_stats.get_position_relative().distance_squared(system_position) > child_system.radius.powi(2) { continue }
                body_mut.relative_stats.translate_to_child(new_time, child_system.position_generator.clone());
                child_system.insert_body(body.clone());
                remove_list.push(index);
                break;
            }
        }
        for index in remove_list {
            self.dynamic_bodies.swap_remove(index);
        }
    }

    /// Only for bodies moving down the tree, not up
    fn insert_body(&mut self, body: Rc<RefCell<DynamicBody>>) {
        self.total_child_dynamic_bodies += 1;
        self.dynamic_bodies.push(body);
    }

    pub fn get_system_position_gens_and_radii(&self) -> Vec<(PositionGenerator, f64)> {
        let mut res = vec![];
        self.get_system_position_gens_and_radii_recursive(&mut res);
        return res
    }
    pub fn get_system_position_gens_and_radii_recursive(&self, res: &mut Vec<(PositionGenerator, f64)>) {
        res.push((self.position_generator.clone(), self.radius));
        for system in &self.child_systems {
            system.get_system_position_gens_and_radii_recursive(res);
        }
    }

    pub fn get_dynamic_bodies_recursive(&self, res: &mut Vec<Rc<RefCell<DynamicBody>>>) {
        res.extend(self.dynamic_bodies.iter().map(|b| b.clone()));
        for child_system in &self.child_systems {
            child_system.get_dynamic_bodies_recursive(res);
        }
    }
    pub fn get_static_bodies_recursive(&self, res: &mut Vec<StaticBody>) {
        res.extend(self.static_bodies.iter().map(|b| b.clone()));
        for child_system in &self.child_systems {
            child_system.get_static_bodies_recursive(res);
        }
    }
    
    pub fn empty_copy(&self, retain: Rc<RefCell<DynamicBody>>) -> GravitySystemTree {
        let child_systems = self.child_systems.iter().map(|s| s.empty_copy(retain.clone())).collect_vec();
        let retained_bodies = self.dynamic_bodies
            .iter()
            .find(|b| Rc::ptr_eq(*b, &retain))
            .map_or(vec![], |x| vec![Rc::new(RefCell::new(x.borrow().clone()))]);
        return Self {
            total_child_dynamic_bodies: retained_bodies.len() + self.child_systems.iter().map(|s| s.total_child_dynamic_bodies).sum::<usize>(),
            dynamic_bodies: retained_bodies,
            child_systems: child_systems,
            ..self.clone()
        }
    }
}

impl Default for GravitySystemTree {
    fn default() -> Self {
        Self {
            time_step: 1,
            radius: 1.,
            position_generator: PositionGenerator::from(StaticPosition::Still),
            mu: 0.,
            total_child_dynamic_bodies: 0,
            dynamic_bodies: Default::default(),
            static_bodies: Default::default(),
            child_systems: Default::default(),
            static_masses: Default::default()
        }
    }
}



