use bevy::{gizmos::gizmos, math::DVec2, prelude::*};
use crate::pseudo_camera::camera::CameraState;

#[derive(Default, Reflect, GizmoConfigGroup)]
pub struct LargeGridConfig {}
#[derive(Default, Reflect, GizmoConfigGroup)]
pub struct SmallGridConfig {}
#[derive(Default, Reflect, GizmoConfigGroup)]
pub struct AxesConfig {}

pub fn update_gizmo_config(mut cs: ResMut<GizmoConfigStore>, camera_query: Query<&CameraState>) {
    let camera = camera_query.single();
    let line_width = camera.get_scale() * 10.0f32.powf(-camera.get_scale().log10().ceil()-2.);
    cs.config_mut::<SmallGridConfig>().0.line_width = line_width;
    cs.config_mut::<LargeGridConfig>().0.line_width =  (line_width * 10.).clamp(0., 0.025);
}

pub fn draw_scale_grid(
    mut large_grid: Gizmos<LargeGridConfig>,
    mut small_grid: Gizmos<SmallGridConfig>,
    mut axes: Gizmos<AxesConfig>,
    camera_query: Query<&CameraState>
) {
    let camera = camera_query.single();

    //calculate grid spacing
    let scalar = camera.get_scale()*10.0f32.powf(-camera.get_scale().log10().ceil() + 2.);

    //chose center position
    let size = 10.0f32.powf(-camera.get_scale().log10().ceil() + 3.) as f64;
    let pos = (camera.position/size).round()*size;
    let world_pos = camera.physics_to_world_pos(pos);

    //draw grids
    large_grid.grid_2d(world_pos, 0., UVec2::splat(20), Vec2::splat(scalar*10.), Color::linear_rgb(0.5, 0.5, 0.5));
    small_grid.grid_2d(world_pos, 0., UVec2::splat(200), Vec2::splat(scalar), Color::linear_rgb(0.5, 0.5, 0.5));

    //draw axes
    axes.grid_2d(camera.physics_to_world_pos(DVec2::ZERO), 0., UVec2::splat(2), Vec2::splat(5000.), Color::linear_rgb(0.5, 0.5, 0.5));
}