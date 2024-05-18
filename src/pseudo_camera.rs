use super::*;


/// Component representing the "state" of the camera
/// This is not the actual state of the camera since I want to allow for correct rendering of far away objects.
/// In reality the camera / projection does not move or scale, instead everything else does.
/// This way objects are always close to the origin when you can see them, so there isn't any float precision rendering nonsense
#[derive(Component, Clone)]
pub struct CameraState {
    // viewing far-away objects may still be a problem.
    // when a faraway object is translated to the origin, the object will render correctly but move in clearly discrete steps.
    // Same problem, different issue. It all stems from floating point precision
    pub position: DVec2, // maybe change to multiple precision in the future (if gravity calculation is optimized enough)
    pub scale: f32, // f32 should be fine for scale,
    dimensions: Vec2
}
impl Default for CameraState {
    fn default() -> Self {
        Self { position: Default::default(), scale: 1., dimensions: Vec2::ZERO }
    }
}
impl CameraState {
    /// Convert a coordinate in the physics sim to a coordinate in the bevy world by translating and scaling
    #[inline]
    pub fn physics_to_world_pos(&self, point: DVec2) -> Vec2 {
        (point - self.position).as_vec2() * self.scale
    }
    /// Convert a coordinate in the bevy world to a coordinate in the physics world by scaling and translating
    #[inline]
    pub fn world_to_physics_pos(&self, point: Vec2) -> DVec2 {
        (point / self.scale).as_dvec2() + self.position
    }
    /// Convert a screen coordinate to a coordinate in the physics world
    #[inline]
    pub fn viewport_to_physics_pos(&self, point: Vec2, camera: &Camera, camera_gtrans: &GlobalTransform) -> Option<DVec2> {
        let Some(world_pos) = camera.viewport_to_world_2d(camera_gtrans, point) else { return None };
        Some(self.world_to_physics_pos(world_pos))
    }
}