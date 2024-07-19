use std::collections::VecDeque;
use bevy::prelude::Entity;
use builder::GravitySystemBuilder;
use dynamic_body::DynamicBody;
use itertools::{multizip, Itertools};
pub use particular::{math::{DVec2, FloatVector, Zero}, ComputeMethod, ParticleSliceSystem, ParticleSystem, PointMass};
use position_generator::PositionGenerator;
use static_body::{StaticBody, StaticPosition};


pub mod static_body;
pub mod dynamic_body;
pub mod position_generator;
pub mod builder;
pub mod system_manager;
pub mod massive_object;

/*
Lower levels of the tree must calculate gravity more often proportional to the change in time step between levels.
So if level 0 uses a time step 1 and level 1 a time step 0.1, for every 1 iteration of level 0, level 1 must do 10 iterations.
This is true for game tick calculations and future path calculations.
This way the transfer of dynamic bodies between systems wont cause time conflicts.


ScheduledEnter: Allow a dynamic body to exit the system it is currently in and enter a lower level system at any time.
                The body does not actually enter the system until the system is ready to calculate its time step.

What about bodies leaving the current system for a higher level one?
Lower level systems will still have to calculate more often than higher level ones so bodies go up levels at discrete intervals
This still has potential to cause issues if bodies are travelling so fast that they travel considerable distance from the system center
    before entering the parent system. They will not experience acceleration from bodies in the parent system.
Perhaps fast forwarding. Bodies exiting a system will enter the fast forward list where bodies are fast forwarded to the next
    discrete system time step before entering the normal pool of bodies.
To calculate the future path of a dynamic body, just make a copy of the system tree with your dynamic body as the only body.

Higher levels of the tree will perform their calculations first. So when a particle enters a lower system, its time is
guaranteed to be greater than or equal to the lower system so it will enter the wait list. When a particle enters a higher
system, its time is guaranteed to be less than or equal to the higher system so it can be fast forwarded.

What happens if a body is travelling so quickly that it skips over a system in one time step?
Need some way to calculate if the velocity vector intersects the system. If it does intersect, fast forward the body using the child
    system's timescale until it enters the system. 
    Get the shortest distance to the line segment: https://stackoverflow.com/questions/849211/shortest-distance-between-a-point-and-a-line-segment
    If this distance is less than the system's radius, then calculate the line segment's intersection with the circle created by the radius.
    Then based on the current position and velocity and intersection point, calculate what time the body enters the system.

A system's time scale should only depend the orbital period of its children. If a system has children with a large orbital period,
    then the system should use a large time scale. If it has children with a small orbital period then it should use a small time step.
    If you want a system with some large period bodies and small period bodies, just make the center of the system a smaller system.
Or should the time scale depend on the speed of its fastest static child? The speed of dynamic bodies will resemble the systems they come from.

It is possible that a body entering a child system will exist at a time step that the child system never exactly calculates.
    e.g. the body is schedules to enter at time 8, and the child system exists at time 6 with a time scale of 3
Should you reverse or fast forward the body to the next logical time step?
Reversing the body may cause issues with the detection of bodies exiting the system, and would likely be a shitty patch fix.
Just fast forward the body to the next logical time step.

the time scale of child systems should be a perfect divisor of the parent's time scale. Otherwise there is an edgecase where
a body exiting a system may do so with a time ahead of the parent's time since the parent waits for the child to catch up before continuing.


Consider a system operating on a time scale of 100, with a child system on a scale of 1. Both systems have one child dynamic body.
On the first iteration, the parent system calculates to time 100, but calculates that its dynamic body will enter the child system at time 70.
Therefore it enters the waitlist of the child system since it has only calculated to time 1.
On the next iteration, the child system detects its child leaves the system at time 2, so it gets forwarded to time 100 to enter the parent system.
The parent system notices that its child is not yet caught up to its time, so it does nothing.
On the next iteration, the child system should pull the body from the wait list since it no longer has any active dynamic bodies at its current time.


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
*/


#[derive(Debug, Clone)]
pub struct SystemTree {
    /// How large of a time step this level of the tree takes each iteration \
    /// This depends on the 
    time_step: usize,
    /// Time associated with the position of dynamic bodies \
    /// Last calculated time for this system
    current_time: usize,

    /// The size of the entire system. \
    /// If a dynamic body is within a distance of radius from the system center, it is part of the system or one of its children. 
    radius: f64,
    /// Center position of the system relative to the parent system. \
    /// Polar coordinates
    position: StaticPosition,
    position_generator: PositionGenerator,
    /// Total mass of all static bodies in the system. Mass of dynamic bodies is negligible.
    mass: f64,

    /// Total number of dynamic bodies that exist under this system. \
    total_child_dynamic_bodies: usize,

    /// Dynamic bodies that are entering the system from a higher one. 
    wait_list: VecDeque<(usize, DynamicBody)>,

    /// Dynamic bodies currently in the system.
    /// Position is relative to the current system to ensure dynamic bodies can properly orbit
    dynamic_bodies: Vec<DynamicBody>,
    dynamic_masses: Vec<PointMass<DVec2, f64>>,

    /// Lone bodies in the system. \
    /// This should really only be used for the leaf nodes of the tree and the center body of systems. (or some extremely massive single object like a black hole) \
    /// If you want something like a rogue planet orbiting the galactic center, a child system containing only one center body is better since any dynamic bodies that approach it will use a finer time scale.
    static_bodies: Vec<StaticBody>,
    child_systems: Vec<SystemTree>,
    /// Used for the gravity calculation
    /// Child system masses come first then static bodies
    static_masses: Vec<PointMass<DVec2, f64>>, 
}

impl SystemTree {
    /// Generate a random system based on the seed
    fn generate(seed: u128) -> Self {
        todo!()
    }
    
    /// Recursively get the smallest current time value of all child systems 
    fn smallest_child_time(&self) -> usize {
        if self.total_child_dynamic_bodies == 0 {
            return usize::MAX
        }
        let children_smallest = self.child_systems.iter().map(|x| x.smallest_child_time()).min().unwrap_or(self.current_time).min(self.current_time);
        let waitlist_smallest = self.wait_list.iter().next().map_or(usize::MAX, |(time, _)| *time);
        return children_smallest.min(waitlist_smallest)
    }

    /// Insert a body coming from a higher system into the wait list
    fn insert_body(&mut self, time: usize, body: DynamicBody) {
        // Increase the total child count even though the body is sitting in the wait list
        self.total_child_dynamic_bodies += 1;
        if self.dynamic_bodies.is_empty() || time == self.current_time {
            self.dynamic_masses.push(body.as_point_mass());
            self.dynamic_bodies.push(body);
            self.current_time = time;
        } else {
            self.wait_list.push_back((time, body));
        }
        
    }

    fn process_elevator(&mut self, elevator: Vec<(DynamicBody, usize)>, changes: &mut Vec<(usize, DynamicBody)>) {
        // Fast forward the bodies to the system's current time
        self.dynamic_bodies.extend(elevator.into_iter().map(|(mut db, t)| {
            let time_diff = self.current_time - t;
            db.set_position(db.position() + db.velocity() * time_diff as f64);
            db
        }));
        let iter = self.dynamic_bodies.iter().skip(self.dynamic_masses.len());
        changes.extend(iter.clone().map(|db| (self.current_time, db.clone())));
        self.dynamic_masses.extend(iter.map(|db| db.as_point_mass()));
    }


    /// Performs one time step of gravity calculation
    pub fn calculate_gravity(&mut self) -> Vec<(usize, DynamicBody)> {
        let mut elevator = Vec::new();
        let mut changes = Vec::new();
        self.calculate_gravity_recursive(&mut elevator, &mut changes);
        return changes
    }

    /// The elevator is a waiting spot for dynamic bodies that are exiting the system they are currently in
    fn calculate_gravity_recursive(&mut self, elevator: &mut Vec<(DynamicBody, usize)>, changes: &mut Vec<(usize, DynamicBody)>) {
        // Changes need to propogate to the acceleration method and elevator handler
        if self.total_child_dynamic_bodies < 1 {
            return
        }

        if self.smallest_child_time() >= self.current_time {
            self.check_wait_list();
            self.current_time += self.time_step;
            self.accelerate_dynamic_bodies(elevator, changes);
            self.check_wait_list();
        }

        let mut new_elevator = Vec::new();
        for child_system in self.child_systems.iter_mut().filter(|x| x.requires_calculation(self.current_time)) {
            child_system.calculate_gravity_recursive(&mut new_elevator, changes);
        }
        self.process_elevator(new_elevator, changes);

    }

    /// Whether this level of the tree should get a recursive call when calculating gravity
    fn requires_calculation(&self, time: usize) -> bool {
        self.total_child_dynamic_bodies > 0 && self.smallest_child_time() < time
    }

    /// Calculate gravitational acceleration for all bodies, then update velocity and position
    /// Maybe report the changes made for storage in a future map
    /// 
    /// Dynamic bodies need to first move with their parent system to ensure orbits around them remain stable.
    /// Only after moving with the parent system do they accelerate
    fn accelerate_dynamic_bodies(&mut self, elevator: &mut Vec<(DynamicBody, usize)>, changes: &mut Vec<(usize, DynamicBody)>) {
        // The elevator fast forwards particles to a newer time, so dont report changes in the elevator
        // As of right now, the wait list doesnt do anything so changes entering the wait list should be reported
        if self.dynamic_bodies.is_empty() { return }
        self.set_static_masses_to(self.current_time);

        let particle_storage = ParticleSliceSystem::with(&self.dynamic_masses, &self.static_masses);
        let mut remove_list: Vec<usize> = Vec::new();
        for (i, (a, b, m)) in multizip((particular::sequential::BruteForceScalar.compute(particle_storage), &mut self.dynamic_bodies, &mut self.dynamic_masses)).enumerate() {
            b.set_velocity(b.velocity() + a*self.time_step as f64);
            b.set_position(b.position() + b.velocity()*self.time_step as f64);
            //detect if the body is exiting the system
            if b.position().norm_squared() > self.radius.powi(2) {
                //B'S POSITION MUST BE CONVERTED TO BE RELATIVE TO THE PARENT SYSTEM BEFORE ENTERING THE ELEVATOR
                let system_center = self.position.get_cartesian_position(self.current_time);
                b.set_position(b.position() + system_center);
                b.set_velocity(b.velocity() + ((self.position.get_cartesian_position(self.current_time+self.time_step) - system_center)/self.time_step as f64));
                elevator.push((b.clone(), self.current_time));
                remove_list.push(i);
                self.total_child_dynamic_bodies -= 1;
                continue;
            }
            //Place body with absolute position and velocity in the changes vec
            let absolute_center = self.position_generator.get(self.current_time);
            let absolute_velocity = self.position_generator.get(self.current_time+self.time_step) / self.time_step as f64;
            let mut absolute_b = b.clone();
            absolute_b.set_position(b.position() + absolute_center);
            absolute_b.set_velocity(b.velocity() + absolute_velocity);
            changes.push((self.current_time, absolute_b));
            // Detect if the body is entering a child system
            if let Some((s, _)) = multizip((&mut self.child_systems, &self.static_masses)).find(|(s, m)| (b.position()-m.position).norm_squared() < s.radius.powi(2)) {
                //B'S POSITION MUST BE CONVERTED TO BE RELATIVE TO THE CHILD SYSTEM BEFORE ENTERING THE WAIT SET
                b.set_position(b.position() - m.position);
                //Keeping velocity consistent is a little more tricky, It should be done according to the child system's discrete velocity
                b.set_velocity(b.velocity() - ((s.position.get_cartesian_position(self.current_time+s.time_step) - m.position) / s.time_step as f64));
                s.insert_body(self.current_time, b.clone());
                remove_list.push(i);
            } else {
                m.position = b.position();
            }
        }
        //remove any dynamic bodies that left the system or entered a lower one.
        for i in remove_list.into_iter().rev() {
            self.dynamic_bodies.swap_remove(i);
            self.dynamic_masses.swap_remove(i);
        }
    }


    /// Calculate the position of all static masses at time.
    /// Place these positions in the static_masses vector
    /// Return the center position of the system at time
    fn set_static_masses_to(&mut self, time: usize) {
        self.static_masses.clear();
        for s in &self.child_systems {
            let pos = DVec2::from(s.position.get_cartesian_position(time));
            self.static_masses.push(PointMass { position: pos, mass: s.mass });
        }
        for sb in &self.static_bodies {
            let pos = DVec2::from(sb.position.get_cartesian_position(time));
            self.static_masses.push(PointMass { position: pos, mass: sb.mass });
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
            self.dynamic_masses.clear();
            self.dynamic_masses.extend(self.dynamic_bodies.iter().map(|x| x.as_point_mass()));
            return
        }

        let num_items_to_drain = self.wait_list
            .iter()
            .find_position(|(time, _)| *time > self.current_time)
            .map(|x| x.0)
            .unwrap_or(self.wait_list.len());
        if num_items_to_drain == 0 { return }
        for (_, body) in self.wait_list.drain(0..num_items_to_drain) {
            self.dynamic_masses.push(body.as_point_mass());
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
    pub fn distribute_entities(&mut self, entities: &[Entity]) {
        if entities.len() < self.total_bodies() { panic!() }
        self.distribute_entities_recursive(entities, &mut 0)
    }
    fn distribute_entities_recursive(&mut self, entities: &[Entity], index: &mut usize) {
        for static_body in &mut self.static_bodies {
            static_body.entity = Some(entities[*index]);
            *index += 1;
        }
        for dynamic_body in &mut self.dynamic_bodies {
            dynamic_body.set_entity(Some(entities[*index]));
            *index += 1;
        }
        for child in &mut self.child_systems {
            child.distribute_entities_recursive(entities, index);
        }
    }

    /// Get the mass and position generator for every static body \
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
    pub fn get_dynamic_bodies_recursive(&self, bodies: &mut Vec<DynamicBody>) {
        let system_center = self.position_generator.get(0);
        let system_velocity = (self.position_generator.get(self.time_step) - system_center) / self.time_step as f64;
        for body in &self.dynamic_bodies {
            bodies.push(DynamicBody::new(system_center+body.position(), system_velocity+body.velocity(), body.mass(), body.get_entity()));
        }
        for child in &self.child_systems {
            child.get_dynamic_bodies_recursive(bodies);
        }
    } 

    pub fn total_bodies(&self) -> usize {
        self.total_child_dynamic_bodies + self.total_static_bodies()
    }
    pub fn total_static_bodies(&self) -> usize {
        self.static_bodies.len() + self.child_systems.iter().map(|x| x.total_static_bodies()).sum::<usize>()
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
            mass: 0.,
            total_child_dynamic_bodies: 0,
            wait_list: VecDeque::new(),
            dynamic_bodies: Default::default(),
            dynamic_masses: Default::default(),
            static_bodies: Default::default(),
            child_systems: Default::default(),
            static_masses: Default::default()
        }
    }
}








#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_update_check() {
        let mut test_system = SystemTree {
            radius: 100.,
            total_child_dynamic_bodies: 1,
            dynamic_bodies: vec![DynamicBody::new(DVec2::ZERO, DVec2::new(0., 1.), 1., None)],
            dynamic_masses: vec![PointMass { position: DVec2::ZERO, mass: 1.}],
            ..SystemTree::default()
        };
        test_system.calculate_gravity();
        let body = test_system.dynamic_bodies.first().unwrap();
        assert_eq!(*body, DynamicBody::new(DVec2::new(0., 1.), DVec2::new(0., 1.), 1., None));
    }
}