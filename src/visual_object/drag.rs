use crate::{physics::{Change, ChangeEvent}, pseudo_camera::{self, camera::CameraState}};

use super::*;

pub fn drag_object(
    listener: Listener<Pointer<Drag>>,
    mut ew: EventWriter<ChangeEvent>,
    object_query: Query<&VisualObjectData>,
    camera_query: Query<&CameraState>
) {
    let camera = camera_query.single();
    
    let obj = object_query.get(listener.target).unwrap();
    let mut delta = (listener.delta / camera.get_scale()).as_dvec2();
    delta.y *= -1.;
    let event = ChangeEvent::new(listener.target, Change::SetPosition(obj.position + delta));
    ew.send(event);
}