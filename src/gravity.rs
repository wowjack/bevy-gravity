use bevy::prelude::*;

use crate::{GameState, object::MassiveObject};

const G: f32 = 0.0000000000667;

pub fn object_gravity(
    state: Res<GameState>,
    time: Res<Time>,
    mut object_query: Query<(&GlobalTransform, &mut MassiveObject)>,
) {
    if state.play == false { return; }

    let delta_time = time.delta().as_millis() as f32 / 1000.;

    let mut v: Vec<_> = object_query.iter_mut().collect();
    for i in 0..v.len() {
        let (_, c2) = v.split_at_mut(i);
        let ((trans, object), c2) = c2.split_first_mut().unwrap(); //safe

        c2.iter_mut().for_each(|(other_trans, other_obj)| {
            let force = G * object.mass * other_obj.mass / trans.translation().distance_squared(other_trans.translation());
            let angle = (trans.translation() - other_trans.translation()).truncate().angle_between(Vec2::X);

            let accel = force/object.mass; //a = f/m
            object.velocity += Vec2::new(angle.cos()*accel*-1., angle.sin()*accel) * delta_time;

            let other_accel = force/other_obj.mass; //a = f/m
            other_obj.velocity += Vec2::new(angle.cos()*other_accel, angle.sin()*other_accel*-1.) * delta_time;
        });
    }
}