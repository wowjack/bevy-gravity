use bevy::prelude::*;
use crate::{gravity_system_tree::system_manager::GravitySystemManager, pseudo_camera::camera::CameraState};
use super::{follow_object::StaticBody, DrawOptions, SelectedObjects};

#[derive(Default, Reflect, GizmoConfigGroup)]
pub struct FuturePathLineConfig {}

pub fn draw_future_paths(
    camera_query: Query<&CameraState>,
    mut gizmos: Gizmos<FuturePathLineConfig>,
    draw_options: Res<DrawOptions>,
    selected_objects: Res<SelectedObjects>,
    gravity_system: NonSend<GravitySystemManager>,
) {
    if draw_options.draw_future_path == false { return }
    let Some((entity, _)) = selected_objects.focused else { return };
    let camera_state = camera_query.single();

    if let Some(i) = gravity_system.static_body_entities.iter().position(|x| *x == entity) {
        let StaticBody { position_generator, .. } = &gravity_system.static_bodies[i];
        let iter = (0..100_000).map(|i| camera_state.physics_to_world_pos(position_generator.get(gravity_system.latest_time + i.max((i as f32/(1000.*camera_state.get_scale())) as u64))));
            gizmos.linestrip_2d(
                iter,
                Color::linear_rgb(0.75, 0.75, 0.75)
            );
    }
    else if let Some(i) = gravity_system.dynamic_body_entities.iter().position(|x| *x == entity) {
        let body = &gravity_system.dynamic_bodies[i];
        let mut new_system = gravity_system.system.empty_copy(body.clone());
            let mut new_dynamic_bodies = vec![];
            new_system.get_dynamic_bodies_recursive(&mut new_dynamic_bodies);
            let body = new_dynamic_bodies.first().unwrap().clone();
            let iter = (1..100_000).map(|i| {
                new_system.accelerate_and_move_bodies_recursive(gravity_system.latest_time+i, &mut vec![]);
                camera_state.physics_to_world_pos(body.borrow().relative_stats.get_position_absolute(gravity_system.latest_time+i))
            });
            gizmos.linestrip_2d(
                iter,
                Color::linear_rgb(0.75, 0.75, 0.75)
            );
    }
    else {
        panic!("
        How did you get here.
        You tried to get the future position of a visual object that doesnt appear to exist in the gravity system.
        ")
    }
}



pub fn set_future_path_gizmo_config(mut cs: ResMut<GizmoConfigStore>) {
    cs.config_mut::<FuturePathLineConfig>().0.line_width = 1.;
}