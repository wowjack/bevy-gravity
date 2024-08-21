use std::{clone, collections::VecDeque};
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

    /// Get position only using the first n positions in the position generator \
    /// If n == position_chain.len() then get_partial_start == get \
    pub fn get_partial_start(&self, time: u64, n: usize) -> DVec2 {
        assert!(n <= self.position_chain.len());
        self.position_chain
            .iter()
            .take(n)
            .fold(DVec2::ZERO, |acc, e| acc + e.get_cartesian_position(time))
    }

    pub fn get_orbit_circle(&self, time: u64) -> Option<(DVec2, f64)> {
        let mut orbit_radius = 0.;
        let mut index = 0;

        let mut iter = self.position_chain.iter().enumerate().rev();
        loop {
            match iter.next() {
                None => return None,
                Some((_, StaticPosition::Still)) => continue,
                Some((i, StaticPosition::Circular { radius, .. })) => {
                    orbit_radius = *radius;
                    index = i;
                    break;
                }
            }
        }

        return Some((self.get_partial_start(time, index), orbit_radius))
    }

    pub fn extend(mut self, new: StaticPosition) -> Self {
        if let StaticPosition::Still = &new { return self }
        self.position_chain.push_back(new);
        self
    }
    pub fn extend_generator(mut self, new: PositionGenerator) -> Self {
        self.position_chain.extend(new.position_chain.into_iter());
        self
    }

    pub fn prepend(&mut self, new: StaticPosition) {
        if let StaticPosition::Still = new { return }
        self.position_chain.push_front(new);
    }
    pub fn prepend_generator(&mut self, gen: &PositionGenerator) {
        for pos in gen.position_chain.iter().rev() {
            self.position_chain.push_front(pos.clone());
        }
    }

    pub fn pop_end(&mut self) {
        self.position_chain.pop_back();
    }

    pub fn get_end(&self) -> StaticPosition {
        self.position_chain.back().map_or(StaticPosition::Still, clone::Clone::clone)
    }

    pub fn len(&self) -> usize {
        self.position_chain.len()
    }
}

impl From<StaticPosition> for PositionGenerator {
    fn from(value: StaticPosition) -> Self {
        let mut position_chain = VecDeque::new();
        position_chain.push_back(value);
        Self { position_chain }
    }
}