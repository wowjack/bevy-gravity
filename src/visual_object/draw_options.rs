use super::*;


#[derive(Resource)]
pub struct DrawOptions {
    pub draw_velocity_arrow: bool,
    pub draw_future_path: bool
}
impl Default for DrawOptions {
    fn default() -> Self {
        Self {
            draw_velocity_arrow: true,
            draw_future_path: true
        }
    }
}