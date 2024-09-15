use bevy::prelude::*;
use bevy_vector_shapes::{prelude::ShapePainter, shapes::DiscPainter};
use crate::{gravity_system_tree::system_manager::GravitySystemManager, pseudo_camera::camera::CameraState};
use super::{DrawOptions, SelectedObjects, SimulationState};

#[derive(Default, Reflect, GizmoConfigGroup)]
pub struct FuturePathLineConfig {}


pub fn set_future_path_gizmo_config(mut cs: ResMut<GizmoConfigStore>) {
    cs.config_mut::<FuturePathLineConfig>().0.line_width = 0.05;
}