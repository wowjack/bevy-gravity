
use std::{cell::RefCell, rc::Rc};

use itertools::Itertools;

use super::{dynamic_body::DynamicBody, position_generator::PositionGenerator, static_body::{StaticBody, StaticPosition}, system_tree::GravitySystemTree};


/// Only way to construct SystemTree objects
/// When constructing the tree, it makes sure all parameters are correct before returning it.
#[derive(Clone)]
pub struct GravitySystemBuilder {
    system: GravitySystemTree,
    set_position: bool,
    /// Store child system builders so they can be built with proper coordinates from the top down
    child_systems: Vec<GravitySystemBuilder>,
}

impl GravitySystemBuilder {
    pub fn new() -> Self { 
        Self { system: Default::default(), set_position: false, child_systems: vec![] }
    }

    pub fn with_static_bodies(mut self, bodies: &[StaticBody]) -> Self {
        self.system.static_bodies.extend_from_slice(bodies);
        self
    }
    /// Add dynamic bodies relative to the current system
    pub fn with_dynamic_bodies(mut self, bodies: &[DynamicBody]) -> Self {
        self.system.dynamic_bodies.extend(bodies.iter().map(|b| Rc::new(RefCell::new(b.clone()))));
        self
    }
    pub fn with_position(mut self, position: StaticPosition) -> Self {
        self.system.position_generator = PositionGenerator::from(position);
        self.set_position = true;
        self
    }
    /// The radius you ask for is not always the radius you get
    /// The minimum radius size is two times the distance of the furthest child system? 1.5 time distance?
    /// The radii of child systems must be ensured to never overlap
    pub fn with_radius(mut self, radius: f64) -> Self {
        self.system.radius = radius;
        self
    }
    pub fn with_time_step(mut self, time_step: u64) -> Self {
        self.system.time_step = time_step;
        self
    }
    pub fn with_children(mut self, builders: &[GravitySystemBuilder]) -> Self {
        self.child_systems.extend_from_slice(builders);
        self
    }

    /// Fill in any SystemTree parameters that need to be calculated, then validate the tree to make sure everything makes sense
    /// Position needs to be calculated from the top down
    /// mass and child bodies needs to be calculated from the bottom up
    /// Assign each static and dynamic body with a bevy entity used to associate it with a visual object
    pub fn build(self) -> Result<GravitySystemTree, SystemTreeError> {
        GravitySystemBuilder::validate_tree(self.build_recursive(PositionGenerator::default())?)
    }

    fn build_recursive(mut self, parent_generator: PositionGenerator) -> Result<GravitySystemTree, SystemTreeError> {
        if !self.set_position { return Err(SystemTreeError::NoPosition) }

        self.system.position_generator = parent_generator.extend_generator(self.system.position_generator);

        for body in &self.system.dynamic_bodies {
            body.borrow_mut().relative_stats.set_generator(self.system.position_generator.clone());
        }

        for body in &mut self.system.static_bodies {
            body.position_generator.prepend_generator(&self.system.position_generator);
        }

        for child_system in self.child_systems {
            let child_system = child_system.build_recursive(self.system.position_generator.clone())?;
            self.system.child_systems.push(child_system);
        }

        // Calculate system mu by sum of mu of child systems
        self.system.mu = self.system.child_systems
            .iter()
            .map(|x| x.mu)
            .chain(self.system.static_bodies.iter().map(|x| x.mu))
            .sum();
        self.system.total_child_dynamic_bodies = self.system.child_systems
            .iter()
            .map(|s| s.total_child_dynamic_bodies)
            .sum::<usize>() + self.system.dynamic_bodies.len();

        self.system.set_static_masses_to(0);
        self.system.calculate_accelerations();

        return Ok(self.system)
    }

    fn validate_tree(tree: GravitySystemTree) -> Result<GravitySystemTree, SystemTreeError> {
        for (index, child) in tree.child_systems.iter().enumerate() {
            if child.time_step > tree.time_step {
                return Err(SystemTreeError::MinTimeScale)
            } else if tree.time_step % child.time_step != 0 {
                return Err(SystemTreeError::NonDivisibleTimeScale)
            }

            // Going to leave this part for another time
            // I want the possibility of multiple star systems around the galaxy to be able to exist in the same orbital radius with similar velocity so they dont collide
            /*
            let has_close_children = self.system.child_systems
                .iter()
                .enumerate()
                .find(|(i, os)| *i != index && are_systems_near(os, child))
                .is_some();
            if has_close_children { return Some(SystemTreeError::ChildRadiusOverlap) }

            self.system.stat
            */

        }
        return Ok(tree)
    }

    pub fn total_bodies(&self) -> usize {
        self.system.dynamic_bodies.len() + 
        self.system.static_bodies.len() + 
        self.child_systems.iter().map(|x| x.total_bodies()).sum::<usize>()
    }
}

/// Difference between orbital radii must be greater than the sum of system radii to ensure they dont potentially
fn are_systems_near(system1: &GravitySystemTree, system2: &GravitySystemTree) -> bool {
    (system1.position_generator.get_end().get_radius() - system2.position_generator.get_end().get_radius()).abs() > (system1.radius + system2.radius)
}


#[derive(Debug)]
pub enum SystemTreeError {
    /// Parent time scale must be larger than or equal to all child time scales
    MinTimeScale,
    /// Parent time scale must be a multiple of all child time scales
    NonDivisibleTimeScale,
    /// Child systems' radii must never overlap with eachother or static bodies 
    ChildRadiusOverlap,
    /// You must set a position for the system
    NoPosition
}