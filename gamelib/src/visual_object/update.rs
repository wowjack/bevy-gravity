use crate::{gravity_system_tree::system_manager::GravitySystemManager, pseudo_camera::camera::CameraState};
use super::*;

pub fn update_object_data(
    mut object_query: Query<(&mut VisualObjectData, &mut Visibility)>,
    camera_query: Query<&CameraState>,
    mut sim_state: ResMut<SimulationState>,
    delta_time: Res<Time>,
    mut gravity_system_manager: ResMut<GravitySystemManager>,
) {
    if sim_state.running {
        sim_state.current_time += delta_time.delta().as_millis() as f64 * sim_state.run_speed;
    }

    let Ok(camera) = camera_query.get_single() else { return };

    gravity_system_manager.update_visual_objects(sim_state.current_time as f64, &mut object_query, camera);
}


pub fn update_object_positions(
    mut object_query: Query<(&VisualObjectData, &mut Transform)>,
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
        transform.translation = camera.physics_to_world_pos(&object.position).extend(0.);
        transform.scale = Vec3::splat(camera.get_scale() * object.radius as f32);
    }
}

