use std::{cell::RefCell, rc::Rc};

use bevy::{color::palettes::css::LIGHT_GRAY, math::DVec2, prelude::{Commands, Entity, Resource, World}, utils::hashbrown::HashMap};
use itertools::Itertools;
//use crate::physics::MassiveObject;

use crate::{visual_object::{VisualObjectBundle, VisualObjectData}, G};

use super::{builder::GravitySystemBuilder, dynamic_body::DynamicBody, massive_object::MassiveObject, position_generator::PositionGenerator, system_tree::GravitySystemTree};



pub struct GravitySystemManager {
    pub system: GravitySystemTree,
    /// Smallest time returned in the updates of the last gravity calculation
    pub latest_time: u64,
    /// Latest calculated position of bodies indexed by the entities that represent them
    pub future_map: HashMap<Entity, ObjectFuture>
}
impl GravitySystemManager {
    pub fn new(system: GravitySystemBuilder) -> Self {
        Self {
            system: system.build().unwrap(),
            latest_time: 0,
            future_map: HashMap::new(),
        }
    }
    pub fn get_state_at_time(&mut self, time: u64) -> Vec<(Entity, MassiveObject)> {
        while time > self.latest_time {
            self.latest_time += 1;
            self.system.accelerate_and_move_bodies_recursive(self.latest_time, &mut vec![]);
        }
        
        self.future_map.iter().map(|(e, of)| (e.clone(), of.get_state(time).unwrap())).collect_vec()
    }

    pub fn spawn_entities(&mut self, world: &mut World) {
        let mut res = vec![];
        self.system.get_dynamic_bodies_recursive(&mut res);
        for body in res {
            let body_ref = body.borrow();
            let entity = world.spawn(VisualObjectBundle::new(VisualObjectData::new(body_ref.relative_stats.get_position_absolute(0), body_ref.relative_stats.get_velocity_absolute(0), body_ref.mu/G, body_ref.radius, LIGHT_GRAY.into()))).id();
            self.future_map.insert(entity, ObjectFuture::Dynamic { body: body.clone() });
        }

        let mut res = vec![];
        self.system.get_static_bodies_recursive(&mut res);
        for (body, position_generator) in res {
            let position = position_generator.get(0);
            let velocity = position_generator.get(1) - position;
            let entity = world.spawn(VisualObjectBundle::new(VisualObjectData::new(position, velocity, body.mass(), body.radius, LIGHT_GRAY.into()))).id();
            self.future_map.insert(entity, ObjectFuture::Static { object: MassiveObject { position, velocity, mass: body.mass() }, generator: position_generator });
        }
    }
}


pub enum ObjectFuture {
    Static { 
        object: MassiveObject,
        generator: PositionGenerator
    },
    Dynamic {
        body: Rc<RefCell<DynamicBody>>
    }
}

impl ObjectFuture {
    pub fn get_state(&self, time: u64) -> Option<MassiveObject> {
        match self {
            Self::Static { object, generator } => {
                let position = generator.get(time);
                return Some(MassiveObject { position: bevy::math::DVec2::new(position.x, position.y), ..object.clone() })
            },
            Self::Dynamic { body } => {
                let body = body.borrow();
                let object = MassiveObject {
                    position: body.relative_stats.get_position_absolute(time),
                    velocity: body.relative_stats.get_velocity_absolute(time),
                    mass: body.mu/G,
                };
                return Some(object)
            }
        }
    }
}