use std::any::Any;

use crate::{gravity_system_tree::system_manager::GravitySystemManager, pseudo_camera::camera::CameraState};

use super::*;


/*
Read position from the physics future, update object positions if they're within the viewport
*/

pub fn update_object_data(
    mut object_query: Query<(&mut VisualObjectData, &mut Transform)>,
    //future: Res<PhysicsFuture>,
    mut sim_state: ResMut<SimulationState>,
    delta_time: Res<Time>,
    mut gravity_system_manager: ResMut<GravitySystemManager>,
) {
    sim_state.timer.tick(delta_time.delta());

    for (entity, change) in gravity_system_manager.get_state_at_time(sim_state.current_time) {
        let Ok((mut object, _)) = object_query.get_mut(entity) else { return };
        object.position = change.position;
        object.velocity = change.velocity;
    }


    // About 60 updates per second
    if sim_state.running == false { return }
    sim_state.current_time = sim_state.current_time + sim_state.timer.times_finished_this_tick() as u64;



    /*
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
    */
}


pub fn update_object_positions(
    mut object_query: Query<(&mut VisualObjectData, &mut Transform)>,
    mut camera_query: Query<&mut CameraState>,
    follow_resource: Res<FollowObjectResource>,
    selected_objects: Res<SelectedObjects>,
) {
    let mut camera = camera_query.single_mut();

    if follow_resource.follow_object {
        if let Some((e, data)) = &selected_objects.focused {
            if data.position.distance_squared(camera.position) < data.velocity.length_squared() {
                camera.position = object_query.get(*e).unwrap().0.position;
            }
        }
    }
    
    for (object, mut transform) in object_query.iter_mut() {
        transform.translation = camera.physics_to_world_pos(object.position).extend(0.);
        transform.scale = Vec3::splat(camera.get_scale() * object.radius);
    }
}

