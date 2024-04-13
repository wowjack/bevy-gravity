use bevy::gizmos;

use super::*;
use crate::{pseudo_camera::CameraState, MassiveObject};

#[derive(Component, Clone, Copy)]
pub struct VelocityArrow;


// Do some kind of logarithmic scaling for the velocity arrow?
// As of right now I think it gets a little large
pub fn draw_velocity_arrows(
    object_query: Query<(&MassiveObject, &Transform), With<VelocityArrow>>, 
    camera_query: Query<&CameraState>,
    mut gizmos: Gizmos,
    draw_options: Res<DrawOptions>
) {
    if draw_options.draw_velocity_arrow == false { return }
    let scale = camera_query.single().scale;
    for (object, transform) in &object_query {
        let pos = transform.translation.xy();
        gizmos.arrow_2d(pos, pos+(object.velocity*scale as f64).as_vec2(), Color::BLACK);
    }
}