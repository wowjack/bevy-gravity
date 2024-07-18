use itertools::Itertools;
use particular::math::{DVec2, Zero};

use super::static_body::StaticPosition;



/*
Compute and cache positions at requested times based on a chain of polar coordinates calculated from orbit parameters of static bodies
*/
#[derive(Debug)]
pub struct PositionGenerator {
    position_chain: Vec<StaticPosition>
}
impl PositionGenerator {
    pub fn new() -> Self {
        Self {
            position_chain: vec![]
        }
    }

    pub fn get(&self, time: usize) -> DVec2 {
        self.position_chain
            .iter()
            .fold(DVec2::ZERO, |acc, e| acc + e.get_cartesian_position(time))
    }

    pub fn extend(mut self, new: StaticPosition) -> Self {
        self.position_chain.push(new);
        self
    }
}