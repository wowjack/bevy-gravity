use std::collections::VecDeque;

use bevy::math::DVec2;

use super::{static_body::StaticPosition, system_tree::GravitySystemTime, BodyPosition, BodyVelocity};



#[derive(Clone, Debug, Default)]
pub struct StaticGenerator {
    chain: VecDeque<StaticPosition>
}
impl StaticGenerator {
    pub fn new() -> Self {
        Self { chain: VecDeque::new() }
    }

    pub fn get_position(&self, time: GravitySystemTime) -> BodyPosition {
        self.chain
            .iter()
            .fold(DVec2::ZERO, |acc, e| acc + e.get_position(time))
    }
    pub fn get_last_position(&self, time: GravitySystemTime) -> BodyPosition {
        todo!()
    }

    pub fn get_velocity(&self, time: GravitySystemTime) -> BodyVelocity {
        self.chain
            .iter()
            .fold(DVec2::ZERO, |acc, e| acc + e.get_velocity(time))
    }
    pub fn get_last_velocity(&self, time: GravitySystemTime) -> BodyVelocity {
        todo!()
    }

    pub fn get_position_and_velocity(&self, time: GravitySystemTime) -> (BodyPosition, BodyVelocity) {
        self.chain
            .iter()
            .fold((DVec2::ZERO, DVec2::ZERO), |mut acc, e| {
                let (pos, vel) = e.get_position_and_velocity(time);
                acc.0 += pos; acc.1 += vel;
                acc
            })
    }
    pub fn get_last_position_and_velocity(&self, time: GravitySystemTime) -> (BodyPosition, BodyVelocity) {
        todo!()
    }

    pub fn pop_end(&mut self) -> StaticPosition {
        self.chain.pop_back().unwrap_or(StaticPosition::Still)
    }
    pub fn get_end(&self) -> Option<&StaticPosition> {
        self.chain.back()
    }
    pub fn push_end(&mut self, static_position: StaticPosition) {
        if let StaticPosition::Still = static_position { return }
        self.chain.push_back(static_position)
    }

    pub fn len(&self) -> usize {
        self.chain.len()
    }
}