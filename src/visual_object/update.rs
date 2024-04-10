use crate::{physics::{MassiveObject, PhysicsFuture}, pseudo_camera::CameraState};

use super::*;


/*
Read position from the physics future, update object positions if they're within the viewport
*/

pub fn update_object_positions(
    mut object_query: Query<(&mut MassiveObject, &mut Transform)>,
    camera_query: Query<&CameraState>,
    future: Res<PhysicsFuture>,
    mut sim_state: ResMut<SimulationState>,
) {
    let camera = camera_query.single();

    // Update the massive objects
    for (entity, frame) in future.get_frame(sim_state.current_time) {
        let (mut object, _) = object_query.get_mut(entity).unwrap();
        object.position = frame.position;
        object.velocity = frame.velocity;
    }

    for (object, mut transform) in object_query.iter_mut() {
        transform.translation = camera.physics_to_world_pos(object.position).extend(0.);
        transform.scale = Vec3::splat(camera.scale);
    }

    sim_state.current_time += sim_state.run_speed;
}

