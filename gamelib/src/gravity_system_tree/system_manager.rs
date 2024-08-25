use std::{cell::RefCell, rc::Rc};

use bevy::prelude::{Entity, World};
use itertools::Itertools;

use crate::visual_object::{VisualObjectBundle, VisualObjectData};

use super::{builder::GravitySystemBuilder, dynamic_body::DynamicBody, static_body::StaticBody, system_tree::GravitySystemTree};



pub struct GravitySystemManager {
    pub system: GravitySystemTree,
    pub latest_time: u64,

    pub dynamic_bodies: Vec<Rc<RefCell<DynamicBody>>>,
    pub dynamic_body_entities: Vec<Entity>,

    pub static_bodies: Vec<StaticBody>,
    pub static_body_entities: Vec<Entity>,
}
impl GravitySystemManager {
    pub fn new(system: GravitySystemBuilder) -> Self {
        let system = system.build().unwrap();

        let mut dynamic_bodies = vec![];
        system.get_dynamic_bodies_recursive(&mut dynamic_bodies);

        let mut static_bodies = vec![];
        system.get_static_bodies_recursive(&mut static_bodies);

        Self {
            system,
            latest_time: 0,
            dynamic_bodies,
            dynamic_body_entities: vec![],
            static_bodies,
            static_body_entities: vec![],
        }
    }
    pub fn get_state_at_time(&mut self, time: u64) -> Vec<(Entity, VisualObjectData)> {
        while time > self.latest_time {
            self.latest_time += 1;
            self.system.accelerate_and_move_bodies_recursive(self.latest_time, &mut vec![]);
        }


        self.dynamic_body_entities
            .iter()
            .cloned()
            .zip(
                self.dynamic_bodies
                    .iter()
                    .map(|x| VisualObjectData::from_dynamic_body(&x.borrow(), time))
            ).chain(
                self.static_body_entities
                    .iter()
                    .cloned()
                    .zip(
                        self.static_bodies
                            .iter()
                            .map(|x| VisualObjectData::from_static_body(x, time))
                    )
            ).collect_vec()
    }

    pub fn spawn_entities(&mut self, world: &mut World) {
        let bundle_iter = self.dynamic_bodies
            .iter()
            .map(|x| VisualObjectBundle::new(VisualObjectData::from_dynamic_body(&x.borrow(), 0)));
        self.dynamic_body_entities = world.spawn_batch(bundle_iter).collect_vec();

        let bundle_iter = self.static_bodies
            .iter()
            .map(|x| VisualObjectBundle::new(VisualObjectData::from_static_body(x, 0)));
        self.static_body_entities = world.spawn_batch(bundle_iter).collect_vec();
    }
}