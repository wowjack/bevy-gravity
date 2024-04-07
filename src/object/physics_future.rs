use std::{sync::{mpsc::{Sender, self, Receiver}, Arc, Mutex}, thread::{self, JoinHandle}, collections::VecDeque, time::Duration};

use bevy::{ecs::{schedule::ScheduleNotInitialized, system::Resource}, prelude::*, utils::hashbrown::HashMap};

use super::object::MassiveObject;




pub const TIME_STEP: f32 = 0.001;
const G: f64 = 0.0000000000667;
const FUTURE_MAP_SIZE: usize = 500_000_000;

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
        let t_handle = thread::spawn(|| { physics_worker(receiver, future_clone) });

        Self { sender, future, t_handle }
    }
}
impl PhysicsFuture {
    pub fn len(&self) -> usize {
        self.future.lock().unwrap().values().map(|of| of.len()).sum()
    }
}



fn physics_worker(
    receiver: Receiver<Vec<PhysicsObject>>,
    future: Arc<Mutex<HashMap<Entity, ObjectFuture>>>
) {
    let mut state = receiver.recv().unwrap();
    let mut time = 0u64;
    loop{
        if let Ok(objs) = receiver.try_recv() {
            state = objs;
            time = 0;
            future.lock().unwrap().clear();
            if state.is_empty() { //if the state is empty, wait until the next update
                state = receiver.recv().unwrap();
            }
        }

        //wait for a bit if the future is getting too large
        let points = match future.try_lock() {
            Ok(f) => f.values().map(|of| of.len()).sum(),
            _ => 0
        };
        if points > FUTURE_MAP_SIZE {
            let Ok(objs) = receiver.recv_timeout(Duration::from_secs(1)) else { continue };
            state = objs;
            future.lock().unwrap().clear();
            continue;
        }

        process_physics_frame(&mut state, &future, time);
        time += 1;
    }
}


fn process_physics_frame(objects: &mut Vec<PhysicsObject>, future: &Arc<Mutex<HashMap<Entity, ObjectFuture>>>, time: u64) {
    //let now = std::time::Instant::now();
    for i in 0..objects.len() {
        let c2 = &mut objects[i..];
        let Some((object, c2)) = c2.split_first_mut() else { continue };

        for other_obj in c2.iter_mut() {
            let force = G * object.mass * other_obj.mass / object.position.distance_squared(other_obj.position) as f64;
            let angle = (object.position - other_obj.position).angle_between(Vec2::X) as f64;

            let accel = force / object.mass;
            object.velocity += Vec2::new((angle.cos()*accel*-1.) as f32, (angle.sin()*accel) as f32) * TIME_STEP;

            let other_accel = force/other_obj.mass;
            other_obj.velocity += Vec2::new((angle.cos()*other_accel) as f32, (angle.sin()*other_accel*-1.) as f32) * TIME_STEP;
        }
    }
    let Ok(mut future) = future.lock() else { return };
    for object in objects.iter_mut() {
        object.position += object.velocity * TIME_STEP;
        future.entry(object.object).or_insert(ObjectFuture::default()).try_insert_frame(time, PhysicsState { position: object.position, velocity: object.velocity});
    }
    //println!("{}", now.elapsed().as_micros());
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
    /// get the angle between the last two points and the new point to determine if it should be drawn. How many radians?
    pub fn get_future_linestrip(&self, len: usize) -> Vec<Vec2> {
        let mut points = Vec::new();
        points.extend(self.future.iter().map(|(_, s)| s.position).take(2));
        if points.len() < 2 { return points }

        let mut p1 = points[0];
        let mut p2 = points[1];

        // if the angle between the points becomes too small, insert the previous then current point
        for (_, state) in self.future.iter().skip(2) {
            let p3 = state.position;
            // if the angle is still close to straight, do nothing
            let angle = (p3 - p2).angle_between(p1 - p2);
            if angle.abs() < std::f32::consts::PI - 0.05 { 
                points.push(p3);
                p1 = p2;
                p2 = p3;
            }

            if points.len() >= len {
                return points
            }
        }

        //add the last point
        if let Some((_, last_state)) = self.future.iter().last() {
            points.push(last_state.position);
        }
        return points
    }

    pub fn len(&self) -> usize {
        self.future.len()
    }
}