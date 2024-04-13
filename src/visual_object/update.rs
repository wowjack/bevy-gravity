use crate::{physics::{MassiveObject, PhysicsFuture}, pseudo_camera::CameraState};

use super::*;


/*
Read position from the physics future, update object positions if they're within the viewport
*/

pub fn update_object_positions(
    mut object_query: Query<(&mut MassiveObject, &mut Transform, &Appearance)>,
    camera_query: Query<&CameraState>,
    future: Res<PhysicsFuture>,
    mut sim_state: ResMut<SimulationState>,
    delta_time: Res<Time>,
) {
    sim_state.timer.tick(delta_time.delta());

    let camera = camera_query.single();

    // Update the massive objects
    if sim_state.running == true {
        let mut latest_time = 0;
        for (entity, frame) in future.get_frame(sim_state.current_time) {
            let (mut object, _, _) = object_query.get_mut(entity).unwrap();
            object.position = frame.position;
            object.velocity = frame.velocity;
            if frame.time > latest_time { latest_time = frame.time }
        }

        // About 60 updates per second
        sim_state.current_time = latest_time + sim_state.run_speed * sim_state.timer.times_finished_this_tick() as u64;
    }


    //TODO: Only update objects if their position is within the camera frame.
    //      Objects that are not in the frame should have no visibility.
    //      I don't think this is particularly important for performance or functionality so I'll just leave this here

    for (object, mut transform, appearance) in object_query.iter_mut() {
        transform.translation = camera.physics_to_world_pos(object.position).extend(0.);
        transform.scale = Vec3::splat(camera.scale*appearance.radius);
    }
}

