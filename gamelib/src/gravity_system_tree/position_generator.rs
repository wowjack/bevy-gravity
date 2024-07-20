use std::collections::VecDeque;

use itertools::Itertools;
use particular::math::{DVec2, Zero};
use rand::{thread_rng, Rng};

use super::static_body::StaticPosition;



/*
Compute and cache positions at requested times based on a chain of polar coordinates calculated from orbit parameters of static bodies
*/
#[derive(Debug, Clone)]
pub struct PositionGenerator {
    position_chain: VecDeque<StaticPosition>
}
impl PositionGenerator {
    pub fn new() -> Self {
        Self {
            position_chain: VecDeque::new()
        }
    }

    pub fn get(&self, time: u64) -> DVec2 {
        self.position_chain
            .iter()
            .fold(DVec2::ZERO, |acc, e| acc + e.get_cartesian_position(time))
    }

    pub fn extend(mut self, new: StaticPosition) -> Self {
        if let StaticPosition::Still = &new { return self }
        self.position_chain.push_back(new);
        self
    }

    pub fn prepend(&mut self, new: StaticPosition) {
        if let StaticPosition::Still = new { return }
        self.position_chain.push_front(new);
    }
}