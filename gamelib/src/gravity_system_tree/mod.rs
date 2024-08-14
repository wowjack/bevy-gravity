use std::collections::VecDeque;
use bevy::{color::palettes::css::{LIGHT_GRAY, WHITE}, math::DVec2, prelude::{Commands, Entity}};
use dynamic_body::DynamicBody;
use itertools::{multizip, Itertools};
use position_generator::PositionGenerator;
use rayon::iter::{IndexedParallelIterator, IntoParallelRefMutIterator};
use static_body::{StaticBody, StaticPosition};

use crate::visual_object::{VisualObjectBundle, VisualObjectData};


pub mod static_body;
pub mod dynamic_body;
pub mod position_generator;
pub mod builder;
pub mod system_manager;
pub mod massive_object;
pub mod generate;

/*
Use a binary search pattern to calculate when a body enters a lower system. parent has time scale 100, child 10
If the body is outside the system at time 0, and inside at time 100, check time 90. If outside at 90 use 100, else check 50.
If the body is outside at time 0 and 50, check 80.
If inside at 80, check 70. If outside at 70, use 80. Always choose the first timestep where the body is inside the system according to the child time scale.
This problem would be very hard to solve analytically since the position of the system changes with each time step.
*/


/*
After performing a gravity calculation, all changes should be reported and stored in a future map.
Previous systems can read from the future map like previously to draw objects onto the screen.
This means dynamic bodies need some kind of hashable identifier.


Also factoring in static bodies and sibling systems when calculating gravity for a system's
dynamic bodies could be a good idea if it doesnt impact performance too much.

Using relative coordinates for dynamic bodies causes small accuracy problems.
It really only works well when bodies become trapped in the gravity well of the system, or enter then exit before the velocity of the child system changes too much 

Consider if a body has a velocity of (1,0) when it enters a child system travelling at (0, 1)
The body will get a relative velocity of (1,-1) to offset the velocity of the system.
On the next time step, the body is still in the system, but now the system's velocity is slightly different from (0, 1), more like (-.001, 0.99)
*/

/*
Gravitational acceleration will be updated based on the time step of individual bodies and stored in a map.
Every dynamic body will be updated every individual time step.

For a gravity acceleration update step, it is O(n*k) where n is dynamic bodies and k is static bodies within the system.
For a regular update it is an O(n) operation where n is the total number of dynamic bodies.

This way bodies in high level systems are able to accelerate themselves at any discrete time step instead of only at time step intervals.

Large time jumps are still possible of the dynamic body does not accelerate too much.
A body in a 1000 time scale system can easily calculate its position at time 1000 as long as it doesn't accelerate itself.
If that body begins accelerating itself at time 500, then you can calculate position at time 500, accelerate it, then use the new acceleration to calculate time at 1000.
*/


#[derive(Debug, Clone)]
pub struct SystemTree {
    /// How large of a time step this level of the tree takes each iteration \
    time_step: u64,
    /// Time associated with the position of dynamic bodies \
    /// Last calculated time for this system
    current_time: u64,

    /// The size of the entire system. \
    /// If a dynamic body is within a distance of radius from the system center, it is part of the system or one of its children. 
    radius: f64,
    /// Center position of the system relative to the parent system. \
    /// Polar coordinates
    position: StaticPosition,
    position_generator: PositionGenerator,
    /// Total gravitational parameter of all static bodies in the system. Mass of dynamic bodies is negligible.
    mu: f64,

    /// Total number of dynamic bodies that exist under this system. \
    total_child_dynamic_bodies: usize,

    /// Dynamic bodies that are entering the system from a higher one. 
    wait_list: VecDeque<(u64, DynamicBody)>,

    /// Dynamic bodies currently in the system.
    /// Position is relative to the current system to ensure dynamic bodies can properly orbit
    dynamic_bodies: Vec<DynamicBody>,

    /// Lone bodies in the system. \
    /// This should really only be used for the leaf nodes of the tree and the center body of systems. (or some extremely massive single object like a black hole) \
    /// If you want something like a rogue planet orbiting the galactic center, a child system containing only one center body is better since any dynamic bodies that approach it will use a finer time scale.
    static_bodies: Vec<StaticBody>,
    child_systems: Vec<SystemTree>,
    /// Used for the gravity calculation
    /// Child system masses come first then static bodies
    static_masses: Vec<(DVec2, f64)>, 
}

impl SystemTree {
    /// Recursively get the smallest current time value of all child systems 
    pub fn calculate_latest_time(&self) -> u64 {
        if self.total_child_dynamic_bodies == 0 {
            return u64::MAX
        }
        let children_smallest = self.child_systems.iter().map(|x| x.calculate_latest_time()).min().unwrap_or(self.current_time).min(self.current_time);
        let waitlist_smallest = self.wait_list.iter().next().map_or(u64::MAX, |(time, _)| *time);
        return children_smallest.min(waitlist_smallest);
    }

    /// Insert a body coming from a higher system into the wait list
    fn insert_body(&mut self, time: u64, body: DynamicBody) {
        // Increase the total child count even though the body is sitting in the wait list
        self.total_child_dynamic_bodies += 1;
        if self.dynamic_bodies.is_empty() || time == self.current_time {
            self.dynamic_bodies.push(body);
            self.current_time = time;
        } else {
            self.wait_list.push_back((time, body));
        }
        
    }


    /// Performs one time step of gravity calculation
    pub fn calculate_gravity(&mut self) -> Vec<(u64, DynamicBody)> {
        let mut changes = Vec::new();
        self.calculate_gravity_recursive(&mut changes, &mut vec![]);
        return changes
    }

    /// Returns a vector of dynamic bodies that are exiting the system
    fn calculate_gravity_recursive(&mut self, changes: &mut Vec<(u64, DynamicBody)>, elevator: &mut Vec<(u64, DynamicBody)>) {
        let should_calculate = self.calculate_latest_time() >= self.current_time;
        if should_calculate {
        //  Accelerate dynamic bodies
            self.accelerate_dynamic_bodies();
            self.current_time += self.time_step;
        //  check if anything in the wait list needs to enter
            self.check_wait_list();
        //  check for bodies moving down the tree
            self.descend_dynamic_bodies(changes);
        }
        //  recursive call on child systems
        let mut new_elevator = vec![];
        for system in &mut self.child_systems {
            if system.total_child_dynamic_bodies < 1 || system.calculate_latest_time() >= self.current_time { continue } 
            system.calculate_gravity_recursive(changes, &mut new_elevator);
        }

        //  Find bodies exiting system and place in elevator
        self.ascend_dynamic_bodies(elevator);

        //  Handle bodies moving up from children
        let num_bodies = new_elevator.len();
        self.dynamic_bodies.extend(new_elevator.into_iter().map(|(time, body)| body.fast_forward(time, self.current_time)));

        //  Report any changes made here
        // If this system was updated, report all dynamic bodies, otherwise only report the ones that got fast forwarded
        if should_calculate {
            changes.extend(self.dynamic_bodies.iter().map(|b| (self.current_time, b.clone().make_absolute(&self.position_generator, self.current_time, self.time_step))));
        } else {
            changes.extend(self.dynamic_bodies.iter().rev().take(num_bodies).map(|b| (self.current_time, b.clone().make_absolute(&self.position_generator, self.current_time, self.time_step))));
        }

    }

    /// Search for any dynamic bodies near child systems, and move them to the child system if required
    /// Do the required calculation of translating relative coordinates 
    fn descend_dynamic_bodies(&mut self, changes: &mut Vec<(u64, DynamicBody)>) {
        let remove_list = self.dynamic_bodies
            .iter()
            .enumerate()
            .filter_map(|(index, body)| {
                for system in &mut self.child_systems {
                    let system_position = system.position.get_cartesian_position(self.current_time);
                    let position_difference = body.position() - system_position;
                    if position_difference.length_squared() < system.radius.powi(2) {
                        
                        // Reverse body and system until they no longer intersect
                        //for now I will just move it directly into the child system
                        //translate position and velocity to be relative to the child 
                        let system_velocity = (system.position.get_cartesian_position(self.current_time+system.time_step) - system_position) / system.time_step as f64;
                        let new_velocity = body.velocity() - system_velocity;
                        let new_body = DynamicBody::new(position_difference, new_velocity, body.mu(), body.radius(), body.get_entity());
                        system.insert_body(self.current_time, new_body.clone());
                        changes.push((self.current_time, new_body.make_absolute(&system.position_generator, self.current_time, system.time_step)));
                        return Some(index);
                    }
                }
                return None;
            }).collect_vec();
        for index in remove_list.iter().rev() {
            self.dynamic_bodies.swap_remove(*index);
        }
    }
    //same thing as descend but for bodies moving up
    fn ascend_dynamic_bodies(&mut self, elevator: &mut Vec<(u64, DynamicBody)>) {
        let system_position = self.position.get_cartesian_position(self.current_time);
        let system_velocity = (self.position.get_cartesian_position(self.current_time+self.time_step) - system_position) / self.time_step as f64;
        let remove_list = self.dynamic_bodies
            .iter()
            .enumerate()
            .filter_map(|(index, body)| {
                if body.position().length_squared() <= self.radius.powi(2) { return None }
                // bodies are fast forwarded after entering the elevator
                let new_body = DynamicBody::new(system_position+body.position(), body.velocity()+system_velocity, body.mu(), body.radius(), body.get_entity());
                elevator.push((self.current_time, new_body));
                return Some(index);
            }).collect_vec();
        self.total_child_dynamic_bodies -= remove_list.len();
        for index in remove_list.iter().rev() {
            self.dynamic_bodies.swap_remove(*index);
        }
    }


    /// Calculate gravitational acceleration for all bodies, then update velocity and position
    /// Maybe report the changes made for storage in a future map
    /// 
    /// Dynamic bodies need to first move with their parent system to ensure orbits around them remain stable.
    /// Only after moving with the parent system do they accelerate
    fn accelerate_dynamic_bodies(&mut self) {
        if self.dynamic_bodies.is_empty() { return }
        self.set_static_masses_to(self.current_time);

        for body in self.dynamic_bodies.iter_mut() {
            let acceleration = self.static_masses.iter().fold(DVec2::ZERO, |acceleration, static_mass| { acceleration + body.force_scalar(static_mass.0, static_mass.1) });

            body.set_velocity(body.velocity() + acceleration*self.time_step as f64);
            body.set_position(body.position() + body.velocity()*self.time_step as f64);
        }
    }


    /// Calculate the position of all static masses at time.
    /// Place these positions in the static_masses vector
    /// Return the center position of the system at time
    fn set_static_masses_to(&mut self, time: u64) {
        self.static_masses.clear();
        for s in &self.child_systems {
            self.static_masses.push((s.position.get_cartesian_position(time), s.mu));
        }
        for sb in &self.static_bodies {
            self.static_masses.push((sb.position.get_cartesian_position(time), sb.mu));
        }
    }

    /// Get any bodies associated with the current time out of the wait list \
    /// This should be called before calling accelerate_dynamic_bodies to ensure any bodies from upper levels make it in at the proper time
    fn check_wait_list(&mut self) {
        if self.wait_list.is_empty() { return }

        if self.dynamic_bodies.is_empty() {
            let (time, first) = self.wait_list.pop_front().unwrap();
            self.current_time = time;
            let index = self.wait_list.iter().find_position(|x| x.0 > time).map_or(self.wait_list.len(), |x| x.0);
            self.dynamic_bodies.push(first);
            self.dynamic_bodies.extend(self.wait_list.drain(0..index).map(|(_, b)| b));
            return
        }

        let num_items_to_drain = self.wait_list
            .iter()
            .find_position(|(time, _)| *time > self.current_time)
            .map(|x| x.0)
            .unwrap_or(self.wait_list.len());
        if num_items_to_drain == 0 { return }
        for (_, body) in self.wait_list.drain(0..num_items_to_drain) {
            self.dynamic_bodies.push(body);
        }
    }


    pub fn print_bodies(&self) {
        self.print_bodies_recursive();
        println!("");
    }
    pub fn print_bodies_recursive(&self) {
        for body in self.dynamic_bodies.iter() {
            println!("{body:?}\t time: {}\t scale: {}", self.current_time, self.time_step);
        }
        for (time, body) in self.wait_list.iter() {
            println!("waiting {body:?}\t time: {}\t scale: {}", time, self.time_step);
        }
        for child_system in &self.child_systems {
            child_system.print_bodies();
        }
    }

    // Distribute bevy entity references to the dynamic and static bodies
    pub fn distribute_entities(&mut self, commands: &mut Commands) {
        self.distribute_entities_recursive(commands);
    }
    fn distribute_entities_recursive(&mut self, commands: &mut Commands) {
        for static_body in &mut self.static_bodies {
            static_body.entity = Some(commands.spawn(VisualObjectBundle::new(VisualObjectData::new(DVec2::ZERO, DVec2::ZERO, static_body.mass(), static_body.radius, LIGHT_GRAY.into()))).id());
        }
        for dynamic_body in &mut self.dynamic_bodies {
            dynamic_body.set_entity(Some(commands.spawn(VisualObjectBundle::new(VisualObjectData::new(DVec2::ZERO, DVec2::ZERO, dynamic_body.mass(), dynamic_body.radius(), WHITE.into()))).id()));
        }
        for child in &mut self.child_systems {
            child.distribute_entities_recursive(commands);
        }
    }

    /// Get the mu and position generator for every static body \
    /// (This is useful for drawing the static objects without having to look at the tree
    pub fn get_static_body_positions(&self) -> Vec<(StaticBody, PositionGenerator)> {
        let mut res = vec![];
        self.get_static_bodies_recursive(&mut res);
        return res;
    }
    fn get_static_bodies_recursive(&self, bodies: &mut Vec<(StaticBody, PositionGenerator)>) {
        for body in &self.static_bodies {
            bodies.push((body.clone(), self.position_generator.clone().extend(body.position.clone())))
        }
        for child in &self.child_systems {
            child.get_static_bodies_recursive(bodies);
        }
    }


    pub fn get_dynamic_body_positions(&self) -> Vec<DynamicBody> {
        let mut bodies = vec![];
        self.get_dynamic_bodies_recursive(&mut bodies);
        return bodies
    }
    fn get_dynamic_bodies_recursive(&self, bodies: &mut Vec<DynamicBody>) {
        let system_center = self.position_generator.get(0);
        let system_velocity = (self.position_generator.get(self.time_step) - system_center) / self.time_step as f64;
        for body in &self.dynamic_bodies {
            bodies.push(DynamicBody::new(system_center+body.position(), system_velocity+body.velocity(), body.mu(), body.radius(), body.get_entity()));
        }
        for child in &self.child_systems {
            child.get_dynamic_bodies_recursive(bodies);
        }
    }

    pub fn get_system_position_gens_and_radii(&self) -> Vec<(PositionGenerator, f64)> {
        let mut res = vec![];
        self.get_system_position_gens_and_radii_recursive(&mut res);
        return res;
    }
    fn get_system_position_gens_and_radii_recursive(&self, res: &mut Vec<(PositionGenerator, f64)>) {
        res.push((self.position_generator.clone(), self.radius));
        for system in &self.child_systems {
            system.get_system_position_gens_and_radii_recursive(res);
        }
    }

    pub fn total_bodies(&self) -> usize {
        self.total_child_dynamic_bodies + self.total_static_bodies()
    }
    pub fn total_static_bodies(&self) -> usize {
        self.static_bodies.len() + self.child_systems.iter().map(|x| x.total_static_bodies()).sum::<usize>()
    }

    pub fn empty_copy(&self, retain: Option<Entity>) -> SystemTree {
        let child_systems = self.child_systems.iter().map(|s| s.empty_copy(retain)).collect_vec();
        let retained_bodies = self.dynamic_bodies
            .iter()
            .find(|b| b.get_entity() == retain)
            .map_or(
                self.wait_list.iter().find(|(_,b)| b.get_entity() == retain).map_or(vec![], |(_, b)| vec![b.clone()]),
                |b| vec![b.clone()]
            );
        return Self {
            total_child_dynamic_bodies: retained_bodies.len() + self.child_systems.iter().map(|s| s.total_child_dynamic_bodies).sum::<usize>(),
            wait_list: VecDeque::new(),
            dynamic_bodies: retained_bodies,
            child_systems: child_systems,
            ..self.clone()
        }
    }
}

impl Default for SystemTree {
    fn default() -> Self {
        Self {
            time_step: 1,
            current_time: 0,
            radius: 1.,
            position: StaticPosition::Still,
            position_generator: PositionGenerator::new().extend(StaticPosition::Still),
            mu: 0.,
            total_child_dynamic_bodies: 0,
            wait_list: VecDeque::new(),
            dynamic_bodies: Default::default(),
            static_bodies: Default::default(),
            child_systems: Default::default(),
            static_masses: Default::default()
        }
    }
}








#[cfg(test)]
mod tests {
    use bevy::prelude::SystemBuilder;
    use builder::GravitySystemBuilder;

    use super::*;

    #[test]
    fn simple_update_check() {
        let mut test_system = SystemTree {
            radius: 100.,
            total_child_dynamic_bodies: 1,
            dynamic_bodies: vec![DynamicBody::new(DVec2::ZERO, DVec2::new(0., 1.), 1., 1., None)],
            ..SystemTree::default()
        };
        test_system.calculate_gravity();
        let body = test_system.dynamic_bodies.first().unwrap();
        assert_eq!(*body, DynamicBody::new(DVec2::new(0., 1.), DVec2::new(0., 1.), 1., 1., None));
    }


    /// Make sure that child systems update correctly and more frequently than parent systems
    #[test]
    fn proper_system_iteration() {
        let grandchild = GravitySystemBuilder::new()
            .with_position(StaticPosition::Still)
            .with_time_step(1)
            .with_radius(500.)
            .with_dynamic_bodies(&[DynamicBody::new(DVec2::ZERO, DVec2::Y, 0., 1., None)]);
        let child = GravitySystemBuilder::new()
            .with_position(StaticPosition::Still)
            .with_time_step(5)
            .with_radius(10_000.)
            .with_dynamic_bodies(&[DynamicBody::new(DVec2::X*5000., DVec2::Y, 1., 1., None)])
            .with_children(&[grandchild]);
        let mut parent = GravitySystemBuilder::new()
            .with_position(StaticPosition::Still)
            .with_time_step(10)
            .with_radius(1_000_000.)
            .with_dynamic_bodies(&[DynamicBody::new(DVec2::X*50_000., DVec2::Y, 2., 1., None)])
            .with_children(&[child])
            .build()
            .unwrap();
        
        let res = parent.calculate_gravity();
        println!("{res:?}");
        assert!(res.contains(&(1, DynamicBody::new(DVec2::Y, DVec2::Y, 0., 1., None))));
        assert!(res.contains(&(5, DynamicBody::new(DVec2::new(5000., 5.), DVec2::Y, 1., 1., None))));
        assert!(res.contains(&(10, DynamicBody::new(DVec2::new(50_000., 10.), DVec2::Y, 2., 1., None))));



        let res = parent.calculate_gravity();
        println!("{:?}", res);
        assert!(res.contains(&(2, DynamicBody::new(DVec2::Y*2., DVec2::Y, 0., 1., None))));

        let res = parent.calculate_gravity();
        assert!(res.contains(&(3, DynamicBody::new(DVec2::Y*3., DVec2::Y, 0., 1., None))));

        let res = parent.calculate_gravity();
        assert!(res.contains(&(4, DynamicBody::new(DVec2::Y*4., DVec2::Y, 0., 1., None))));

        let res = parent.calculate_gravity();
        assert!(res.contains(&(5, DynamicBody::new(DVec2::Y*5., DVec2::Y, 0., 1., None))));

        let res = parent.calculate_gravity();
        assert!(res.contains(&(6, DynamicBody::new(DVec2::Y*6., DVec2::Y, 0., 1., None))));
        assert!(res.contains(&(10, DynamicBody::new(DVec2::new(5000., 10.), DVec2::Y, 1., 1., None))));

        let res = parent.calculate_gravity();
        assert!(res.contains(&(7, DynamicBody::new(DVec2::Y*7., DVec2::Y, 0., 1., None))));

        let res = parent.calculate_gravity();
        assert!(res.contains(&(8, DynamicBody::new(DVec2::Y*8., DVec2::Y, 0., 1., None))));

        let res = parent.calculate_gravity();
        assert!(res.contains(&(9, DynamicBody::new(DVec2::Y*9., DVec2::Y, 0., 1., None))));

        let res = parent.calculate_gravity();
        assert!(res.contains(&(10, DynamicBody::new(DVec2::Y*10., DVec2::Y, 0., 1., None))));

        let res = parent.calculate_gravity();
        assert!(res.contains(&(11, DynamicBody::new(DVec2::Y*11., DVec2::Y, 0., 1., None))));
        assert!(res.contains(&(15, DynamicBody::new(DVec2::new(5000., 15.), DVec2::Y, 1., 1., None))));
        assert!(res.contains(&(20, DynamicBody::new(DVec2::new(50_000., 20.), DVec2::Y, 2., 1., None))));

        let res = parent.calculate_gravity();
        assert!(res.contains(&(12, DynamicBody::new(DVec2::Y*12., DVec2::Y, 0., 1., None))));
    }


    #[test]
    fn ascend_single_body() {
        let child = GravitySystemBuilder::new()
            .with_position(StaticPosition::Still)
            .with_time_step(1)
            .with_radius(3.)
            .with_dynamic_bodies(&[DynamicBody::new(DVec2::ZERO, DVec2::Y, 0., 1., None)]);
        let mut parent = GravitySystemBuilder::new()
            .with_position(StaticPosition::Still)
            .with_time_step(10)
            .with_radius(500.)
            .with_children(&[child])
            .build()
            .unwrap();

        let res = parent.calculate_gravity();
        println!("{res:?}");
        assert!(res.contains(&(1, DynamicBody::new(DVec2::Y, DVec2::Y, 0., 1., None))));

        let res = parent.calculate_gravity();
        println!("{res:?}");
        assert!(res.contains(&(2, DynamicBody::new(DVec2::Y*2., DVec2::Y, 0., 1., None))));

        let res = parent.calculate_gravity();
        println!("{res:?}");
        assert!(res.contains(&(3, DynamicBody::new(DVec2::Y*3., DVec2::Y, 0., 1., None))));

        let res = parent.calculate_gravity();
        println!("{res:?}");
        assert!(res.contains(&(10, DynamicBody::new(DVec2::Y*10., DVec2::Y, 0., 1., None))));

        let res = parent.calculate_gravity();
        println!("{res:?}");
        assert!(res.contains(&(20, DynamicBody::new(DVec2::Y*20., DVec2::Y, 0., 1., None))));
    }
}