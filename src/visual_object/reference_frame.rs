use crate::pseudo_camera::CameraState;
use super::*;

#[derive(Resource, Default)]
pub struct ReferenceFrameResource {
    pub ref_entity: Option<Entity>,
    pub is_setting_ref_frame: bool
}


pub fn draw_ref_object_halo(
    ref_frame_resource: ResMut<ReferenceFrameResource>,
    object_query: Query<(&Transform, &VisualObjectData)>,
    camera_query: Query<&CameraState>,
    mut gizmos: Gizmos
) {
    let Some(e) = ref_frame_resource.ref_entity else { return };
    let camera = camera_query.single();
    let Ok((trans, object)) = object_query.get(e.clone()) else { return };
    gizmos.circle_2d(trans.translation.truncate(), object.radius*camera.scale, Color::PURPLE).segments(CIRCLE_VERTICES);
}