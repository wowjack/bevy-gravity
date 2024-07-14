use std::{collections::VecDeque, sync::Arc, time::Duration};
use crossbeam_channel::Receiver;
use itertools::*;

use bevy::{math::DVec2, utils::HashMap};
use particular::particle::Accelerations;
use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};

use super::*;


pub enum WorkerSignal {
    Kill,
    Changes(Vec<ChangeEvent>)
}

pub fn physics_worker(
    receiver: Receiver<WorkerSignal>,
    map: FutureMap
) {
    let mut time: u64 = 0;
    let mut state: Vec<(Entity, MassiveObject)> = vec![];
    loop {
        // If the state is empty, wait forever
        // If the map is too large, wait a second
        // Else, don't wait
        match if state.is_empty() { 
                receiver.recv().or(Err(true)) 
            } else if map.len() > MAX_FUTURE_SIZE {
                receiver.recv_timeout(Duration::from_secs(1)).or(Err(false))
            } else {
                receiver.try_recv().or(Err(false))
            } 
        {
            Ok(WorkerSignal::Kill) | Err(true) => return, // return on kill signal or recv_error
            Ok(WorkerSignal::Changes(c)) => {
                state = map.process_changes(c);
                // Make sure no two objects have the same position otherwise the worker will crash.

                // Start at time 1
                // Time 0 already exists
                time = 1; 
            },
            _ => ()
        };

        if map.len() > MAX_FUTURE_SIZE { continue }

        //println!("Process frame {time}");
        process_physics_frame(&mut state);
        map.add_frame(&state, time);
        time += 1;
    }
}



// Going to use a crate to do the barnes hut simulation because I dont really want to do it myself for now

use nbody_barnes_hut::{vector_2d::Vector2D, particle_2d::Particle2D, barnes_hut_2d::QuadTree};

pub const TIME_STEP: f64 = 0.1;
pub const G: f64 = 6.6743015e-11;

fn process_physics_frame(state: &mut Vec<(Entity, MassiveObject)>) {
    // Figure out which n does quadtree performance overtake brute force performance.

    let accelerations = state
        .iter()
        .map(|(_, mo)| (mo.position.to_array(), mo.mass * G))
        .accelerations(&mut particular::compute_methods::sequential::BruteForceScalar)
        .map(|x| DVec2::from_array(x))
        .zip(state.iter_mut())
        .for_each(|(acceleration, (_, mo))| {
            mo.velocity += acceleration * TIME_STEP;
            mo.position += mo.velocity * TIME_STEP;
        });
    /*  state.iter().map(|(_, obj)| Particle2D { position: Vector2D { x: obj.position.x, y: obj.position.y }, mass: obj.mass }).collect_vec();
    let qtree = QuadTree::new(&particle_vec.iter().collect_vec(), 0.5);
    state.par_iter_mut().for_each(|(_, obj)| {
        let force = qtree.calc_forces_on_particle(
            Vector2D::new(obj.position.x, obj.position.y),
            (),
            |distance_squared, mass, distance_vector, _| G * mass * distance_vector / (distance_squared * distance_squared.sqrt())
        );
        obj.velocity.x += force.x;// * obj.mass;
        obj.velocity.y += force.y;// * obj.mass;
        obj.position += obj.velocity * TIME_STEP;
    });
    */
}