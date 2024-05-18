use std::time::Duration;

use super::*;


#[derive(Resource)]
pub struct SimulationState {
    pub running: bool,
    pub current_time: u64,
    pub run_speed: u64,
    pub timer: Timer
}
impl Default for SimulationState {
    fn default() -> Self {
        Self {
            running: true,
            current_time: 0,
            run_speed: 1,
            timer: Timer::new(Duration::from_millis(17), TimerMode::Repeating)
        }
    }
}