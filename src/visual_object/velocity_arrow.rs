use super::*;
use crate::pseudo_camera::CameraState;



// Do some kind of logarithmic scaling for the velocity arrow?
// As of right now I think it gets a little large
pub fn draw_velocity_arrows(
    object_query: Query<(&VisualObjectData, &Transform)>, 
    camera_query: Query<&CameraState>,
    mut gizmos: Gizmos,
    draw_options: Res<DrawOptions>,
    selected_objects: Res<SelectedObjects>,
) {
    if selected_objects.selected.is_empty() { return }
    if draw_options.draw_velocity_arrow == false { return }
    let scale = camera_query.single().scale;
    for (object, transform) in selected_objects.selected.iter().filter_map(|e| object_query.get(*e).ok()) {
        let pos = transform.translation.xy();
        gizmos.arrow_2d(pos, pos+(object.velocity*scale as f64).as_vec2(), Color::GRAY);
    }
}