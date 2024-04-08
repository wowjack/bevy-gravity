use std::{sync::{mpsc::Receiver, Arc, Mutex}, time::Duration};

use bevy::{prelude::*, utils::hashbrown::HashMap};


// use quadtree in the future to go from n^2 to nlogn
//use quadtree_rs::Quadtree;

use super::physics_future::{ObjectFuture, PhysicsObject, PhysicsState, FUTURE_MAP_SIZE};

pub const G: f64 = 0.0000000000667;
pub const TIME_STEP: f32 = 0.001;


pub fn physics_worker_thread(
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

#[inline]
pub fn process_physics_frame(objects: &mut Vec<PhysicsObject>, future: &Arc<Mutex<HashMap<Entity, ObjectFuture>>>, time: u64) {
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