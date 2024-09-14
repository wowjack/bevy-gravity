use bevy::prelude::*;
use bevy_vector_shapes::{prelude::ShapePainter, shapes::DiscPainter};
use crate::{gravity_system_tree::system_manager::GravitySystemManager, pseudo_camera::camera::CameraState};
use super::{DrawOptions, SelectedObjects, SimulationState};

#[derive(Default, Reflect, GizmoConfigGroup)]
pub struct FuturePathLineConfig {}

pub fn draw_future_paths(
    mut painter: ShapePainter,
    camera_query: Query<&CameraState>,
    mut gizmos: Gizmos<FuturePathLineConfig>,
    draw_options: Res<DrawOptions>,
    selected_objects: Res<SelectedObjects>,
    gravity_system: Res<GravitySystemManager>,
    sim_state: Res<SimulationState>,
) {
    if draw_options.draw_future_path == false { return }
    let Some((entity, _)) = selected_objects.focused else { return };
    let camera_state = camera_query.single();


    
    if let Some(i) = gravity_system.body_store.static_entities.iter().position(|x| *x == entity) {
        // This does not work for still bodies in a moving system
        let Some(static_body) = gravity_system.body_store.static_bodies.get(i) else { return };
        let (position, radius) = static_body.get_orbit_parameters(sim_state.current_time);
        let center_pos = camera_state.physics_to_world_pos(&position);

        painter.hollow = true;
        painter.transform = painter.transform.with_translation(center_pos.extend(0.)).with_scale(Vec3::splat(camera_state.get_scale()));
        painter.circle(radius as f32);

    }
    else if let Some(mut new_system) = gravity_system.retain_clone(entity) {
        let body = unsafe { new_system.body_store.dynamic_bodies.get_unchecked(0) };
        let center_pos = body.get_parent_generator().get_position(sim_state.current_time);
        let depth = body.get_system_depth();
        let iter = (0..100_000).map_while(|_| {
            let body = unsafe { new_system.body_store.dynamic_bodies.get_unchecked(0) };
            if body.get_system_depth() != depth { return None }
            let ret = camera_state.physics_to_world_pos(&(center_pos + body.get_previous_relative_position()));
            new_system.step();
            Some(ret)
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