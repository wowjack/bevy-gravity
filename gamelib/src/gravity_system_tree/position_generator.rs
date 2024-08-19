use std::collections::VecDeque;
use bevy::math::DVec2;

use super::static_body::StaticPosition;



/*
Compute and cache positions at requested times based on a chain of polar coordinates calculated from orbit parameters of static bodies
*/
#[derive(Debug, Clone, PartialEq, Default)]
pub struct PositionGenerator {
    position_chain: VecDeque<StaticPosition>
}
impl PositionGenerator {
    pub fn get(&self, time: u64) -> DVec2 {
        self.position_chain
            .iter()
            .fold(DVec2::ZERO, |acc, e| acc + e.get_cartesian_position(time))
    }

    /// Get position only using the last n positions in the position generator \
    /// If n == position_chain.len() then get_partial_end == get \
    pub fn get_partial_end(&self, time: u64, n: usize) -> DVec2 {
        assert!(n <= self.position_chain.len());
        self.position_chain
            .iter()
            .rev()
            .take(n)
            .fold(DVec2::ZERO, |acc, e| acc + e.get_cartesian_position(time))
    }

    /// Get position only using the last n positions in the position generator \
    /// If n == position_chain.len() then get_partial_start == get \
    pub fn get_partial_start(&self, time: u64, n: usize) -> DVec2 {
        assert!(n <= self.position_chain.len());
        self.position_chain
            .iter()
            .take(n)
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

    pub fn pop_end(&mut self) {
        self.position_chain.pop_back();
    }

    pub fn len(&self) -> usize {
        self.position_chain.len()
    }
}