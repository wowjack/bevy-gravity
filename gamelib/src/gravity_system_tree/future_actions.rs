use bevy::math::DVec2;

use super::{system_tree::DiscreteGravitySystemTime, BodyAcceleration};



#[derive(Clone, Debug)]
pub struct FutureActions {

}
impl FutureActions {
    pub fn new() -> Self {
        Self {}
    }

    pub fn get_acceleration(&mut self, time: DiscreteGravitySystemTime, mass: f64) -> BodyAcceleration {
        DVec2::ZERO
    }
}