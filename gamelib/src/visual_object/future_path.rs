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
    gravity_system: NonSend<GravitySystemManager>,
    /*
    physics_future: Res<PhysicsFuture>,
    mut gizmos: Gizmos,
    draw_options: Res<DrawOptions>,
    selected_objects: Res<SelectedObjects>,
    ref_frame_resource: ResMut<ReferenceFrameResource>,
    */
) { 
    if draw_options.draw_future_path == false { return }
    let Some((entity, _)) = selected_objects.focused else { return };
    let camera_state = camera_query.single();
    match gravity_system.future_map.get(&entity) {
        Some(ObjectFuture::Static { generator, .. }) => {
            let iter = (0..100_000).map(|i| camera_state.physics_to_world_pos(generator.get(gravity_system.latest_time + i.max((i as f32/(1000.*camera_state.get_scale())) as u64))));
            gizmos.linestrip_2d(
                iter,
                Color::linear_rgb(0.75, 0.75, 0.75)
            );
        },
        _ => {
            let target = gravity_system.future_map.get(&entity).unwrap();
            let ObjectFuture::Dynamic { body } = target else { return };
            let mut new_system = gravity_system.system.empty_copy(body.clone());
            let mut new_dynamic_bodies = vec![];
            new_system.get_dynamic_bodies_recursive(&mut new_dynamic_bodies);
            let body = new_dynamic_bodies.first().unwrap().clone();
            let iter = (1..50_000).map(|i| {
                new_system.accelerate_and_move_bodies_recursive(gravity_system.latest_time+i, &mut vec![]);
                camera_state.physics_to_world_pos(body.borrow().relative_stats.get_position_absolute(gravity_system.latest_time+i))
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