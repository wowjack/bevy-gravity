use std::{collections::VecDeque, sync::{Arc, Mutex, RwLock}, thread::{self, JoinHandle}};
use crossbeam_channel::{Sender, Receiver};
use bevy::utils::{hashbrown::HashMap, tracing::instrument::WithSubscriber};
use itertools::Itertools;

use super::*;

const DISTANCE_STEP: f64 = 0.1;
const TIME_STEP: f64 = 0.001;
pub const G: f64 = 0.0000000000667;

/*
The min distance step size in the buffer can be quite large while keeping movement smooth if
the method of querying the current state of objects interpolates between the next point if one isn't available
based on time and physical distance.

Maybe even artificially bend the interpolated path when zoomed too far using velocity calculated from the previous point .
*/

#[derive(Resource)]
pub struct PhysicsFuture {
    /// The worker should be able to write to the end of a queue without blocking the main thread from reading the front of the queue.
    /// The hashmap must be mutable to allow for inserting new massive objects. Maybe wrap in an rwlock where it only 
    map: FutureMap,
    sender: Sender<WorkerSignal>,
    t_handle: JoinHandle<()>,
}
impl Default for PhysicsFuture {
    fn default() -> Self {
        let map = FutureMap::default();
        let (sender, receiver) = crossbeam_channel::unbounded();
        let future_clone = map.clone();

        let t_handle = thread::spawn(|| { physics_worker(receiver, future_clone) });

        Self { map, sender, t_handle }
    }
}




pub const MAX_FUTURE_SIZE: usize = 50_000_000;
/*
How to improve concurrency?
Concurrent queues dont allow iterating over contents without removing them.
Dont want to wrap the entire map in a mutex.

Maybe have a thread lock a mutex to set a flag indicating which deque is accessing and whether it is reading or writing.
*/
#[derive(Default, Clone)]
pub struct FutureMap {
    map: Arc<RwLock<HashMap<Entity, ObjectFuture>>>,
    current_time: u64
}
impl FutureMap {
    /// Add a new physics frame to the future
    pub fn add_frame(&self, time: u64, positions: Vec<(Entity, DVec2)>) {
        let mut map = self.map.write().unwrap();
        for (entity, position) in positions {
            let future = map.get_mut(&entity).unwrap();
            future.try_insert(time, position);
        }
    }

    /// Get the positions of objects at a certain time and remove them from the map.
    /// Advance the current time
    pub fn get_frame(&self, time: u64) -> Vec<(Entity, DVec2)> {
        let mut map = self.map.write().unwrap();
        map.iter_mut().map(|(e, of)| (*e, of.get_position(time))).collect_vec()
    }
    /// Get the positions of objects at a certain time without removing them
    pub fn get_current_frame(&self) -> Vec<(Entity, DVec2)>  {
        let mut map = self.map.read().unwrap();
        map.iter().map(|(e, of)| (*e, of.current_position)).collect_vec()
    }

    /// Get all the saved future positions of an entity
    pub fn get_object_future(&self, entity: Entity) -> Vec<DVec2> {
        self.map.read().unwrap().get(&entity).map(|x| x.as_point_vec()).unwrap_or(vec![])
    }

    /// Returns the total number of items in the map
    pub fn len(&self) -> usize {
        self.map.read().unwrap().values().fold(0, |acc, ef| acc + ef.len())
    }

    /// Clear the future (probably due to a change event)
    /// Return the position
    pub fn reset(&mut self) {

    }
}


/// Holds the future position of one entity.
/// A rough velocity can be deduced using new_pos-old_pos, but I'm not sure this is good enough.
/// Adding velocity to the future will make it twice as large, but is it worth the reduced velocity display accuracy?
/// 
/// Possibly make the future just store velocity?
/// This way nothing is stored for an object unless its accelerating.
/// When an object's position gets too far from its last record, new_position - old_position is recorded as velocity.
/// If actual velocity is recorded instead of difference of positions, calculated future position of objects may become off since velocity wont be recorded at every step.
/// Recording the difference of positions maintains the real position, but velocity will be slightly off.
struct ObjectFuture {
    entity: Entity,
    pub current_position: DVec2,
    future: VecDeque<(u64, DVec2)>
}
impl ObjectFuture {
    pub fn new(entity: Entity, position: Option<DVec2>) -> Self {
        Self {
            entity,
            current_position: position.unwrap_or_default(),
            future: VecDeque::new()
        }
    }

    /// Get the position of the object at time
    /// If the position does not exist, it will interpolate using the current and next point position
    pub fn get_position(&mut self, time: u64) -> DVec2 {
        let Some((next_time, next_pos)) = self.future.front() else { return self.current_position };
        if time == *next_time {
            return self.future.pop_front().unwrap().1;
        }
        let time_diff = next_time - time + 1;
        self.current_position += (*next_pos - self.current_position) / time_diff as f64;
        return self.current_position;
    }

    /// Insert a new point into the future if it is far enough away from the previous point
    pub fn try_insert(&mut self, time: u64, position: DVec2) {
        let distance_too_small = self.future.back().map(|(_, last_pos)| last_pos.distance_squared(position) < DISTANCE_STEP.powi(2)).unwrap_or(false);
        if distance_too_small { return }
        self.future.push_back((time, position));
    }

    pub fn as_point_vec(&self) -> Vec<DVec2> {
        self.future.iter().map(|(_, x)| x.clone()).collect()
    }

    pub fn len(&self) -> usize {
        self.future.len()
    }
}