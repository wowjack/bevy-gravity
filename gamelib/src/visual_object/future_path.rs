use bevy::prelude::*;
use crate::{gravity_system_tree::system_manager::{GravitySystemManager, ObjectFuture}, pseudo_camera::{self, camera::CameraState}};
use super::{DrawOptions, ReferenceFrameResource, SelectedObjects, VisualObjectData};


/// Use a marker type used for deciding which objects to draw a future path for?
/// 
/// Ideally this future path should only be created when it needs to.
/// First read The full buffer from the future.
/// Afterwards only read starting at whatever time the previous read ended at.
/// And reread the entire buffer is a change happens.


pub fn draw_future_paths(
    
    //object_query: Query<Entity, With<VisualObjectData>>,
    camera_query: Query<&CameraState>,
    mut gizmos: Gizmos,
    draw_options: Res<DrawOptions>,
    selected_objects: Res<SelectedObjects>,
    gravity_system: Res<GravitySystemManager>,
    /*
    physics_future: Res<PhysicsFuture>,
    mut gizmos: Gizmos,
    draw_options: Res<DrawOptions>,
    selected_objects: Res<SelectedObjects>,
    ref_frame_resource: ResMut<ReferenceFrameResource>,
    */
) {
    
    // My new system tree can calculate 100_000 steps into the future every single frame while only bringing the fps down to 15-20
    // Very long calculations can be made if this calculation does not take place every frame

    if draw_options.draw_future_path == false { return }
    let Some((entity, _)) = selected_objects.focused else { return };
    let camera_state = camera_query.single();
    match gravity_system.future_map.get(&entity) {
        Some(ObjectFuture::Static { generator, .. }) => {
            let smallest_time = gravity_system.system.calculate_latest_time();
            let iter = (0..100_000).map(|i| camera_state.physics_to_world_pos(generator.get(smallest_time + i.max((i as f32/(1000.*camera_state.get_scale())) as u64))));
            gizmos.linestrip_2d(
                iter,
                Color::linear_rgb(0.75, 0.75, 0.75)
            );
        },
        _ => {
            let mut new_system = gravity_system.system.empty_copy(Some(entity));
            let iter = (0..50_000).filter_map(|_| {
                let changes = new_system.calculate_gravity();
                //println!("Got changes: {changes:?}");
                //assert_eq!(changes.len(), 1);
                if changes.len() < 1 { return None }
                Some(camera_state.physics_to_world_pos(changes[0].1.position()))
            });
            gizmos.linestrip_2d(
                iter,
                Color::linear_rgb(0.75, 0.75, 0.75)
            );
        }
    }
    //let mut new_system = gravity_system.system.empty_copy(Some(entity));
    //let iter = 
    /*
    let future_map = physics_future.get_map();
    let map = future_map.map.read().unwrap();
    let Some(object_future) = map.get(&entity) else { return };
    let path = ref_frame_resource.ref_entity
        .map(|e| map.get(&e))
        .flatten()
        .map_or(object_future.as_point_vec(), |ref_future| {
            object_future.as_point_vec_with_reference_frame(ref_future)
    });
    */
    
}