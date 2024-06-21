use crate::pseudo_camera::CameraState;

pub use super::*;

const SPEED: f64 = -0.005;

#[derive(Resource, Default)]
pub struct FollowObjectResource {
    pub follow_object: bool,
}

pub fn move_pseudo_camera(follow_resource: Res<FollowObjectResource>, selected_objects: Res<SelectedObjects>, mut camera_query: Query<&mut CameraState>, time: Res<Time>) {
    if follow_resource.follow_object == false { return }
    let Some((_, VisualObjectData { position, .. })) = &selected_objects.focused else { return };
    let Ok(mut camera_state) = camera_query.get_single_mut() else { return };
    let camera_position = camera_state.position.clone();
    camera_state.position += (position.clone() - camera_position) * (1.0 - (SPEED*time.delta().as_millis() as f64).exp());
}