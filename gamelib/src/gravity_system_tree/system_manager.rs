use bevy::{prelude::{Entity, Resource}, utils::hashbrown::HashMap};
use itertools::Itertools;
//use crate::physics::MassiveObject;

use super::{massive_object::MassiveObject, builder::GravitySystemBuilder, dynamic_body::DynamicBody, position_generator::PositionGenerator, static_body::{StaticBody, StaticPosition}, SystemTree};



#[derive(Resource)]
pub struct GravitySystemManager {
    pub system: SystemTree,
    /// Smallest time returned in the updates of the last gravity calculation
    smallest_time: u64,
    /// Latest calculated position of bodies indexed by the entities that represent them
    pub future_map: HashMap<Entity, ObjectFuture>
}
impl GravitySystemManager {
    // Takes in a system builder to ensure the time hasn't advanced at all
    pub fn new(system: GravitySystemBuilder, entities: &[Entity]) -> Self {
        let mut system = system.build().unwrap();
        system.distribute_entities(entities);
        // Get dynamic body positions at time 0 to initially store in the map
        let dynamic_positions = system.get_dynamic_body_positions();
        let static_positions = system.get_static_body_positions();
        let mut future_map = HashMap::new();
        // populate the future map with time 0 
        for body in dynamic_positions {
            let Some(entity) = body.get_entity() else { continue };
            let o: MassiveObject = body.into();
            future_map.insert(entity, ObjectFuture::Dynamic { prev: o.clone(), prev_time: 0, next: o, next_time: 0 });
        }
        for (body, generator) in static_positions {
            let Some(entity) = body.entity else { continue };
            let position = generator.get(0);
            let velocity = generator.get(1) - position;

            let object = MassiveObject {
                position: bevy::math::DVec2::new(position.x, position.y),
                velocity: bevy::math::DVec2::new(velocity.x, velocity.y),
                mass: body.mass,
            };
            future_map.insert(entity, ObjectFuture::Static { object, generator });
        }
        Self {
            system: system,
            smallest_time: 0,
            future_map,
        }
    }

    pub fn get_state_at_time(&mut self, time: u64) -> Vec<(Entity, MassiveObject)> {
        while time > self.smallest_time {
            let changes = self.system.calculate_gravity();
            self.process_changes_vec(changes);
        }

        self.future_map
            .iter()
            .map(|(entity, of)| (entity.clone(), of.get_state(time).unwrap()))
            .collect_vec()
    }

    /// Update positions in map and self.smallest_time
    fn process_changes_vec(&mut self, changes: Vec<(u64, DynamicBody)>) {
        self.smallest_time = u64::MAX;
        for (new_time, new_body) in changes {
            if new_time < self.smallest_time {
                self.smallest_time = new_time
            }
            let Some(entity) = new_body.get_entity() else { continue };
            match self.future_map.get_mut(&entity) {
                Some(ObjectFuture::Static { .. }) => panic!("How did this happen"),
                Some(ObjectFuture::Dynamic { prev, prev_time, next, next_time }) => {
                    //assert_eq!(*next_time, new_time);// A change should only appear if we have caught up the the last reported time
                    *prev = next.clone(); *prev_time = *next_time;
                    *next = new_body.into(); *next_time = new_time;
                },
                None => {
                    panic!("How did a new entity appear?")
                }
            }
        }
    }
}



pub enum ObjectFuture {
    Static { 
        object: MassiveObject,
        generator: PositionGenerator
    },
    Dynamic {
        prev: MassiveObject,
        prev_time: u64,
        next: MassiveObject,
        next_time: u64,
    }
}

impl ObjectFuture {
    pub fn get_state(&self, time: u64) -> Option<MassiveObject> {
        match self {
            Self::Static { object, generator } => {
                let position = generator.get(time);
                return Some(MassiveObject { position: bevy::math::DVec2::new(position.x, position.y), ..object.clone() })
            },
            Self::Dynamic { prev, prev_time, next, next_time } => {
                if time > *next_time { panic!("no position available. requested: {time} available: {next_time}") }
                if time < *prev_time { panic!("no position available. requested: {time} initial: {prev_time}") }
                if time == *next_time {
                    return Some(next.clone());
                }
                if time == *prev_time {
                    panic!("How return previous time?");
                }
                let total_time_diff = next_time - prev_time;
                let request_time_diff = time - prev_time;
                let scalar = request_time_diff as f64 / total_time_diff as f64;
                let position = prev.position + scalar*(next.position - prev.position);
                return Some(MassiveObject { position, velocity: prev.velocity, mass: prev.mass });
            }
        }
    } 
}