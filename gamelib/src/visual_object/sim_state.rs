use std::time::Duration;

use super::*;


#[derive(Resource)]
pub struct SimulationState {
    pub running: bool,
    pub current_time: f64,
    pub run_speed: f64,
}
impl Default for SimulationState {
    fn default() -> Self {
        Self {
            running: false,
            current_time: 0.,
            run_speed: 1.,
        }
    }
}