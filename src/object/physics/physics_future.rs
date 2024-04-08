use std::{sync::{mpsc::{Sender, self}, Arc, Mutex}, thread::{self, JoinHandle}, collections::VecDeque};

use bevy::{ecs::system::Resource, prelude::*, utils::hashbrown::HashMap};
use itertools::Itertools;

use crate::object::object::MassiveObject;

use super::physics_worker::{self, physics_worker_thread};


pub const FUTURE_MAP_SIZE: usize = 500_000_000;

/// Any time the user changes an object, a physics state change event should be thrown to make sure the physics functions correctly
#[derive(Event, Default)]
pub struct PhysicsStateChangeEvent;
pub fn refresh_physics(
    mut events: EventReader<PhysicsStateChangeEvent>,
    object_query: Query<(Entity, &Transform, &MassiveObject)>,
    future: ResMut<PhysicsFuture>,
    mut update: ResMut<UpdatePhysics>,
) {
    if events.is_empty() { return }
    for _event in events.read() {
        let v: Vec<PhysicsObject> = object_query.iter().map(|(e, t, o)| PhysicsObject {object: e, position: t.translation.truncate(), velocity: o.velocity, mass: o.mass}).collect();
        if let Err(e) = future.sender.send(v) {
            println!("Error sending to physics worker thread: {:?}", e);
        }
        update.time = 0;
    }
}

pub struct PhysicsObject {
    pub object: Entity,
    pub position: Vec2,
    pub velocity: Vec2,
    pub mass: f64
}

#[derive(Resource)]
pub struct PhysicsFuture {
    pub sender: Sender<Vec<PhysicsObject>>,
    pub future: Arc<Mutex<HashMap<Entity, ObjectFuture>>>,
    pub t_handle: JoinHandle<()>
}

impl Default for PhysicsFuture {
    fn default() -> Self {
        let (sender, receiver) = mpsc::channel();
        let future = Arc::new(Mutex::new(HashMap::new()));
        let future_clone = future.clone();
        let t_handle = thread::spawn(|| { physics_worker_thread(receiver, future_clone) });

        Self { sender, future, t_handle }
    }
}
impl PhysicsFuture {
    pub fn len(&self) -> usize {
        self.future.lock().unwrap().values().map(|of| of.len()).sum()
    }
}



#[derive(Resource, Default)]
pub struct UpdatePhysics {
    pub update: bool,
    pub step: usize,
    pub time: u64
}

pub fn update_object_position(
    mut object_query: Query<(Entity, &mut MassiveObject, &mut Transform)>,
    physics_future: Res<PhysicsFuture>,
    mut update: ResMut<UpdatePhysics>
) {
    if update.update == false { return }
    
    let Ok(mut future) = physics_future.future.lock() else { return };
    for (e, mut object, mut trans) in object_query.iter_mut() {
        let Some(future) = future.get_mut(&e) else { continue };
        let Some(new_state) = future.get_frame(update.time) else { continue };

        object.velocity = new_state.velocity;
        trans.translation = new_state.position.extend(0.);
    }
    update.time += update.step as u64;
}



#[derive(Clone)]
pub struct PhysicsState {
    pub position: Vec2,
    pub velocity: Vec2,
}


/// The computed future position and velocity of a physics object
/// 
#[derive(Default)]
pub struct ObjectFuture {
    future: VecDeque<(u64, PhysicsState)>,
    last_recorded_time: Option<u64>,
    last_queried_time: Option<u64>,
    next_available_time: Option<u64>
}
impl ObjectFuture {
    pub fn get_frame(&mut self, time: u64) -> Option<PhysicsState> {
        self.last_queried_time = Some(time);
        if time < self.next_available_time.unwrap_or(u64::MAX) { return None }

        let Some((to_remove, new_state)) = self.future
            .iter()
            .take_while(|(new_time, _)| new_time <= &time)
            .enumerate()
            .last()
            .map(|(index, (_, state))| (index, state.clone()))
            else { return None };
        drop(self.future.drain(0..=to_remove));
        self.next_available_time = self.future.front().map(|(time, _)| *time);
        Some(new_state.clone())
    }

    pub fn try_insert_frame(&mut self, time: u64, state: PhysicsState) {
        if let Some((_, old_state)) = self.future.back() {
            // if the position and velocity dont change much, dont record it
            if old_state.position.distance_squared(state.position) < 0.01 && old_state.velocity.distance_squared(state.velocity) < 0.01 {
                return
            }
        } else {
            // there is nothing in the future
            self.next_available_time = Some(time);
        }
        self.future.push_back((time, state));
        self.last_recorded_time = Some(time);
    }

    /// Get the future path of the object as a linestrip
    /// Perhaps spawn a background thread that writes the points to a vec resource
    #[inline]
    pub fn get_future_linestrip(&self, buffer_len: usize, segment_len_squared: f32) -> Vec<Vec2> {
        if self.future.len() <= buffer_len.max(2) { return self.future.iter().map(|(_, s)| s.position).collect_vec() }

        let mut points = Vec::from_iter(self.future.iter().take(2).map(|(_, PhysicsState{position, ..})| *position));

        for (_, PhysicsState {position, ..}) in self.future.iter() {
            let prev_points = points.last_chunk::<2>().unwrap();
            if 
                prev_points[0].distance_squared(*position) > segment_len_squared 
                && (*position - prev_points[0]).angle_between(prev_points[1] - prev_points[0]).abs() < std::f32::consts::PI - 0.1
            {
                points.push(*position);
                if points.len() > buffer_len { break }
            } else {
                if let Some(last) = points.last_mut() { *last = *position };
            }
        }

        return points;
    }

    pub fn len(&self) -> usize {
        self.future.len()
    }
    
}


/// Receive a linestrip representing the future path an object will take.
/// Work in the background to strategically remove points from the line strip
/// Iteratively add the culled linestrip to a shared buffer resource
/// When done processing the path, check if more has been added to the path
/// Periodically check if there has been a physics update.
struct PathPredictionWorker {

}