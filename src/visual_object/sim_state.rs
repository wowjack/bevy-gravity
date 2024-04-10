use super::*;


#[derive(Resource)]
pub struct SimulationState {
    pub current_time: u64,
    pub run_speed: u64,
}
impl Default for SimulationState {
    fn default() -> Self {
        Self { current_time: 0, run_speed: 1 }
    }
}