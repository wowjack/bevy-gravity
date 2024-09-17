use bevy::{ecs::system, prelude::{Commands, Entity, Query, Resource, Visibility}};

use crate::{pseudo_camera::camera::CameraState, visual_object::VisualObjectData};

use super::{builder::GravitySystemBuilder, system_tree::{BodyStore, DiscreteGravitySystemTime, GravitySystemTime, GravitySystemTree}};



#[derive(Resource, Clone)]
pub struct GravitySystemManager {
    system_tree: GravitySystemTree,

    /// Dynamic and static bodies along with their entities if they have them
    pub body_store: BodyStore,

    /// The time associated with the current position of bodies
    current_time: DiscreteGravitySystemTime
}
impl GravitySystemManager {
    pub fn new(builder: GravitySystemBuilder) -> Self {
        let (system_tree, body_store) = builder.build().unwrap();
        Self { system_tree, body_store, current_time: 0 }
    }
    /// If the new time is greater than the current time, then update dynamic bodies. \
    /// Update visual objects in the query to the new time. \
    /// BE CERTAIN THAT new_time ISNT NEGATIVE OR ELSE UB OCCURS
    pub fn update_visual_objects(
        &mut self,
        new_time: GravitySystemTime,
        object_query: &mut Query<(&mut VisualObjectData, &mut Visibility)>,
        camera: &CameraState,
    ) {
        // to_int rounds down, so add 1
        let new_discrete_time = unsafe { new_time.to_int_unchecked::<DiscreteGravitySystemTime>() + 1 };

        // update dynamic bodies until current_time = new_discrete_time
        while self.current_time < new_discrete_time {
            self.current_time += 1;
            self.body_store.update_dynamic_bodies(&mut self.system_tree, self.current_time);
        }

        // Set the position of all static bodies
        self.body_store.update_static_bodies(&self.system_tree, new_time);

        // Set visual objects using the query
        let interpolation_factor = new_time - (new_discrete_time as f64 - 1.);
        self.system_tree.update_visual_objects(&self.body_store, object_query, camera, true, interpolation_factor)
        //self.body_store.update_visual_objects(object_query, interpolation_factor);
    }

    pub fn step(&mut self) {
        self.current_time += 1;
        self.body_store.update_dynamic_bodies(&mut self.system_tree, self.current_time);
    }

    /// Populate the body store with entities
    pub fn spawn_bodies(&mut self, commands: &mut Commands) {
        self.body_store.spawn_visual_objects(commands);
    }

    pub fn get_current_time(&self) -> DiscreteGravitySystemTime {
        self.current_time
    }

    /// Copy the system and retain one dynamic body
    pub fn retain_clone(&self, entity: Entity) -> Option<Self> {
        let Some((body_store, idx)) = self.body_store.retain_clone(entity) else { return None };
        let system_tree = self.system_tree.retain_clone(idx);
        if system_tree.total_child_dynamic_bodies == 0 { return None };
        Some(Self {
            system_tree,
            body_store,
            current_time: self.current_time
        })
    }
}