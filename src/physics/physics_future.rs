use std::{collections::VecDeque, sync::{atomic::AtomicU64, Arc, Mutex, RwLock}, thread::{self, JoinHandle}};
use crossbeam_channel::{Sender, Receiver};
use bevy::utils::{hashbrown::HashMap, tracing::instrument::WithSubscriber};
use itertools::Itertools;

use super::*;

const DISTANCE_STEP: f64 = 1.0;
pub const MAX_FUTURE_SIZE: usize = 50_000_000;

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
impl PhysicsFuture {
    pub fn send_changes(&self, changes: Vec<ChangeEvent>) {
        self.sender.send(WorkerSignal::Changes(changes)).unwrap();
    }

    pub fn get_frame(&self, time: u64) -> Vec<(Entity, FutureFrame)> {
        self.map.get_frame(time)
    }

    pub fn get_map(&self) -> FutureMap {
        self.map.clone()
    }   

    /// Kill the physics worker and clean up
    pub fn stop(self) {
        match self.sender.send(WorkerSignal::Kill) {
            Ok(()) => println!("Successfully killed worker thread."),
            Err(_) => {
                println!("ERROR KILLING WORKER THREAD: {}", if self.t_handle.is_finished() { "Worker encountered panic." } else { "Worker is alive but unresponsive." } );
            }
        }
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }
}


/*
How to improve concurrency?
Concurrent queues dont allow iterating over contents without removing them.
Dont want to wrap the entire map in a mutex.

Maybe have a thread lock a mutex to set a flag indicating which deque is accessing and whether it is reading or writing.
*/
#[derive(Default, Clone)]
pub struct FutureMap {
    pub map: Arc<RwLock<HashMap<Entity, ObjectFuture>>>,
}
impl FutureMap {
    /// Add a new physics frame to the future
    pub fn add_frame(&self, data: &Vec<(Entity, MassiveObject)>, time: u64) {
        let mut map = self.map.write().unwrap();
        for (entity, state) in data {
            let future = map.get_mut(entity).unwrap();
            future.try_insert(FutureFrame { time, position: state.position, velocity: state.velocity });
        }
    }

    /// Get the positions of objects at a certain time and remove them from the map.
    /// Advance the current time
    pub fn get_frame(&self, time: u64) -> Vec<(Entity, FutureFrame)> {
        let mut map = self.map.write().unwrap();
        map.iter_mut().map(|(e, of)| (*e, of.get_state(time))).collect_vec()
    }
    /// Get the positions of objects at a certain time without removing them
    pub fn get_current_frame(&self) -> Vec<(Entity, FutureFrame)>  {
        let mut map = self.map.read().unwrap();
        map.iter()
            .map(|(e, of)| 
                (*e, FutureFrame { time: of.current_time, position: of.current_state.position, velocity: of.current_state.velocity })
            ).collect_vec()
    }

    /// Get all the saved future positions of an entity.
    /// Return the vec of positions and the last time stamp 
    pub fn get_object_future(&self, entity: &Entity) -> Vec<DVec2> {
        self.map.read().unwrap().get(entity).map(|x| x.as_point_vec()).unwrap_or(vec![])
    }
    /// Get the saved future positions of an object after a certain time.
    /// This is used so the path predictor can get new positions without copying the entire buffer.
    pub fn get_object_future_since(&self, entity: &Entity, time: u64) -> Vec<DVec2> {
        self.map.read().unwrap().get(entity).map(|x| x.as_point_vec_since(time)).unwrap_or(vec![])
    }

    /// Returns the total number of items in the map
    pub fn len(&self) -> usize {
        self.map.read().unwrap().values().fold(0, |acc, ef| acc + ef.len())
    }

    pub fn process_changes(&self, changes: Vec<ChangeEvent>) -> Vec<(Entity, MassiveObject)> {
        let mut map = self.map.write().unwrap();
        map.iter_mut().for_each(|(_, of)| { of.clear_future(); });
        for change in changes {
            match change.change { // create and delete object events are handled by the map
                // is it appropriate to use drop like this?
                Change::CreateObject(o) => drop(map.insert(change.entity, ObjectFuture::new(change.entity, o))),
                Change::DeleteObject => drop(map.remove(&change.entity).unwrap()),
                Change::SetPosition(p) => map.get_mut(&change.entity).unwrap().current_state.position = p,
                Change::SetVelocity(v) => map.get_mut(&change.entity).unwrap().current_state.velocity = v,
                Change::SetMass(m) => map.get_mut(&change.entity).unwrap().current_state.mass = m,
            }
        }
        map.iter().map(|(e, of)| (*e, of.current_state.clone())).collect_vec()

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
/// 
/// 
/// If just recording a position list
/// Velocity at pos[i].vel = pos[i-1] - pos[i+1].vel
/// But this doesn't consider time difference between points
#[derive(Debug)]
pub struct ObjectFuture {
    entity: Entity,
    future: VecDeque<FutureFrame>,

    /// Current time according to the object's future.
    /// This should always keep up with simulation time
    /// MIGHT AS WELL HAVE CURRENT STATE BE THE INTERPOLATED RETURN VALUE
    /// There is no simulation consistency unless you save a snapshot of ALL positions and velocities at a certain time
    current_time: u64,
    pub current_state: MassiveObject,
}
impl ObjectFuture {
    pub fn new(entity: Entity, object: MassiveObject) -> Self {
        Self {
            entity,
            future: VecDeque::new(),
            current_time: 0,
            current_state: object,
        }
    }

    /// Get the position and velocity of an object at time
    /// If the position does not exist, it will interpolate using the current and next point position
    pub fn get_state(&mut self, time: u64) -> FutureFrame {
        // Remove from the future until front.time > time or front doesnt exist
        // So self.current_time <= time
        while let Some(next_state) = self.future.front() {
            if next_state.time > time { break }
            let frame = self.future.pop_front().unwrap();
            self.current_state.position = frame.position;
            self.current_state.velocity = frame.velocity;
            self.current_time = frame.time;
        }

        if time == self.current_time || self.future.is_empty() {
            return FutureFrame { time: self.current_time, position: self.current_state.position, velocity: self.current_state.velocity }
        }

        // The exact time doesn't exist in the future.
        // EXPERIMENTAL STUFF GOING ON. INTERPOLATING VELOCITY
        let next_state = self.future.front().unwrap();

        let time_frac = (time - self.current_time) as f64 / (next_state.time - self.current_time) as f64;
        //let pos_diff = next_state.position - self.current_state.position;
        let vel_diff = next_state.velocity - self.current_state.velocity;
        
        self.current_time = time;
        //self.current_state.position += pos_diff * time_frac;
        self.current_state.velocity += vel_diff * time_frac; // Should velocity be interpolated
        self.current_state.position += self.current_state.velocity * TIME_STEP;
        
        return FutureFrame {
            time: self.current_time,
            position: self.current_state.position,
            velocity: self.current_state.velocity,// Should velocity be interpolated as well?
                                                  // I figured editing objects should only change the actual stored values in the future.
                                                  // If interpolated values are modified, resetting them to the interpolated value will not reflect the same simulation behavior
                                                  // since the interpolated values are just approximated and didnt actually occur
        }
    }

    /// Insert a new point into the future if it is far enough away from the previous point
    pub fn try_insert(&mut self, state: FutureFrame) {
        let distance_too_small = self.future.back().map(|FutureFrame { position: last_pos, .. }| last_pos.distance_squared(state.position) < DISTANCE_STEP.powi(2)).unwrap_or(false);
        if distance_too_small { return }
        self.future.push_back(state);
    }

    pub fn as_point_vec(&self) -> Vec<DVec2> {
        self.future.iter().map(|FutureFrame { position, .. }| position.clone()).collect_vec()
    }
    pub fn as_point_vec_since(&self, time: u64) -> Vec<DVec2> {
        self.future.iter().skip_while(|ff| ff.time < time).map(|ff| ff.position).collect_vec()
    }

    pub fn clear_future(&mut self) {
        self.future.clear();
        self.current_time = 0;
    }

    pub fn len(&self) -> usize {
        self.future.len()
    }
}



#[derive(Debug, Default, Clone)]
pub struct FutureFrame {
    pub time: u64,
    pub position: DVec2,
    pub velocity: DVec2,
}
