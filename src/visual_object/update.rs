use crate::{physics::PhysicsFuture, pseudo_camera::CameraState};

use super::*;


/*
Read position from the physics future, update object positions if they're within the viewport
*/

pub fn update_object_data(
    mut object_query: Query<(&mut VisualObjectData, &mut Transform)>,
    future: Res<PhysicsFuture>,
    mut sim_state: ResMut<SimulationState>,
    delta_time: Res<Time>,
) {
    sim_state.timer.tick(delta_time.delta());


    // Update the massive objects
    let mut latest_time = 0;
    for (entity, frame) in future.get_frame(sim_state.current_time) {
        let (mut object, _) = object_query.get_mut(entity).unwrap();
        object.position = frame.position;
        object.velocity = frame.velocity;
        if frame.time > latest_time { latest_time = frame.time }
    }

    // About 60 updates per second
    if sim_state.running == false { return }
    sim_state.current_time = latest_time + sim_state.run_speed * sim_state.timer.times_finished_this_tick() as u64;
}


pub fn update_object_positions(
    mut object_query: Query<(&mut VisualObjectData, &mut Transform)>,
    camera_query: Query<&CameraState>,
) {
    let camera = camera_query.single();
    for (object, mut transform) in object_query.iter_mut() {
        transform.translation = camera.physics_to_world_pos(object.position).extend(0.);
        transform.scale = Vec3::splat(camera.scale * object.radius);
    }
}

