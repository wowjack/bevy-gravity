use bevy::prelude::*;
use bevy_vector_shapes::{painter, prelude::ShapePainter, shapes::DiscPainter};
use crate::{gravity_system_tree::system_manager::GravitySystemManager, pseudo_camera::camera::CameraState};
use super::{follow_object::StaticBody, DrawOptions, SelectedObjects};

#[derive(Default, Reflect, GizmoConfigGroup)]
pub struct FuturePathLineConfig {}

pub fn draw_future_paths(
    mut painter: ShapePainter,
    camera_query: Query<&CameraState>,
    mut gizmos: Gizmos<FuturePathLineConfig>,
    draw_options: Res<DrawOptions>,
    selected_objects: Res<SelectedObjects>,
    gravity_system: Res<GravitySystemManager>,
) {
    if draw_options.draw_future_path == false { return }
    let Some((entity, _)) = selected_objects.focused else { return };
    let camera_state = camera_query.single();


    
    if let Some(i) = gravity_system.body_store.static_entities.iter().position(|x| *x == entity) {
        // This does not work for still bodies in a moving system
        let Some(static_body) = gravity_system.body_store.static_bodies.get(i) else { return };
        let position = static_body.stats.current_absolute_position - static_body.stats.current_relative_position;
        let radius = static_body.static_position.get_radius();
        let center_pos = camera_state.physics_to_world_pos(position);

        painter.hollow = true;
        painter.transform = painter.transform.with_translation(center_pos.extend(0.)).with_scale(Vec3::splat(camera_state.get_scale()));
        painter.circle(radius as f32);

    }
    /*
    else if let Some(i) = gravity_system.dynamic_body_entities.iter().position(|x| *x == entity) {
        let body = &gravity_system.dynamic_bodies[i];
        let mut new_system = gravity_system.system.empty_copy(body.clone());
            let mut new_dynamic_bodies = vec![];
            new_system.get_dynamic_bodies_recursive(&mut new_dynamic_bodies);
            let body = new_dynamic_bodies.first().unwrap().clone();
            let system_depth = body.borrow().relative_stats.num_ancestors();

            let iter = (1..50_000).map_while(|i| {
                let body = body.borrow();
                if body.relative_stats.num_ancestors() != system_depth {
                    return None
                }
                let ret = Some(camera_state.physics_to_world_pos(body.relative_stats.get_ancestor_position(gravity_system.latest_time, 1) + body.relative_stats.get_position_relative_to_ancestor(gravity_system.latest_time, 1)));
                std::mem::drop(body);
                new_system.accelerate_and_move_bodies_recursive(gravity_system.latest_time+i, &mut vec![]);
                
                ret
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
    */
}



pub fn set_future_path_gizmo_config(mut cs: ResMut<GizmoConfigStore>) {
    cs.config_mut::<FuturePathLineConfig>().0.line_width = 1.;
}