use std::{collections::VecDeque, sync::Arc, time::Duration};
use crossbeam_channel::Receiver;

use bevy::utils::HashMap;

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
    let mut state: Vec<MassiveObject> = vec![];
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
            Ok(WorkerSignal::Changes(c)) => process_changes(c, &mut state, &map),
            _ => ()
        };

        if map.len() > MAX_FUTURE_SIZE { continue }

        process_physics_frame(&mut state, &map, time);
        time += 1;
    }
}


fn process_changes(changes: Vec<ChangeEvent>, state: &mut Vec<MassiveObject>, map: &FutureMap) {
    for event in changes {
        match event.change {
            Change::CreateObject(object) => {
                state.push(object);
            },
            Change::DeleteObject => {
                
            },
            Change::SetPosition(position) => {

            },
            Change::SetVelocity(velocity) => {

            },
            Change::SetMass(mass) => {

            },
        }
    }
}


fn process_physics_frame(state: &mut Vec<MassiveObject>, map: &FutureMap, time: u64) {
    todo!()
}