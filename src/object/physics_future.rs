use std::{sync::{mpsc::{Sender, self, Receiver}, Arc, Mutex}, thread::{self, JoinHandle}, collections::{HashMap, VecDeque}};

use bevy::{prelude::*, ecs::system::Resource};

use super::object::MassiveObject;

const TIME_STEP: f32 = 0.001;
const G: f64 = 0.0000000000667;

/// Any time the user changes an object, a physics state change event should be thrown to make sure the physics functions correctly
#[derive(Event, Default)]
pub struct PhysicsStateChange;
pub fn refresh_physics(
    mut events: EventReader<PhysicsStateChange>,
    object_query: Query<(Entity, &Transform, &MassiveObject)>,
    future: ResMut<PhysicsFuture>
) {
    if events.is_empty() { return }
    for _event in events.read() {
        let v: Vec<PhysicsObject> = object_query.iter().map(|(e, t, o)| PhysicsObject {object: e, position: t.translation.truncate(), velocity: o.velocity, mass: o.mass}).collect();
        if let Err(e) = future.sender.send(v) {
            println!("Error sending to physics worker thread: {:?}", e);
        }
    }
}

pub struct PhysicsObject {
    object: Entity,
    position: Vec2,
    velocity: Vec2,
    mass: f64
}

#[derive(Resource)]
pub struct PhysicsFuture {
    pub sender: Sender<Vec<PhysicsObject>>,
    pub future: Arc<Mutex<HashMap<Entity, VecDeque<Vec2>>>>,
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



fn physics_worker(
    receiver: Receiver<Vec<PhysicsObject>>,
    future: Arc<Mutex<HashMap<Entity, VecDeque<Vec2>>>>
) {
    let mut state = vec![];
    loop{
        if let Ok(objs) = receiver.try_recv() {
            state = objs;
            future.lock().unwrap().clear();
            println!("{:?}", state.len());
        }

        process_physics_frame(&mut state, &future)
    }
}


fn process_physics_frame(objects: &mut Vec<PhysicsObject>, future: &Arc<Mutex<HashMap<Entity, VecDeque<Vec2>>>>) {
    for i in 0..objects.len() {
        let (_, c2) = objects.split_at_mut(i);
        let (object, c2) = c2.split_first_mut().unwrap();

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
        object.position += object.velocity;
        future.entry(object.object).or_insert(VecDeque::new()).push_back(object.position);
    }

}


pub fn update_object_position(mut object_query: Query<(Entity, &mut MassiveObject, &mut Transform)>, physics_future: Res<PhysicsFuture>) {
    let Ok(mut future) = physics_future.future.lock() else { return };
    for (e, mut object, mut trans) in object_query.iter_mut() {
        let Some(future) = future.get_mut(&e) else { continue };
        let Some(new_pos) = future.pop_front() else { continue };

        object.velocity = new_pos - trans.translation.truncate();
        trans.translation = new_pos.extend(0.);
    }
}