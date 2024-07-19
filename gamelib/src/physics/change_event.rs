use bevy::math::DVec2;

use self::visual_object::{SimulationState, VisualObjectData};

use super::*;

/*
Any time you want to modify a physics object, you must do so by sending an event.
The event processor will hand the changes to the physics worker.
Will it be fine if changes are only visually reflected once the worker gets them?
*/

#[derive(Event, Clone, Debug)]
pub struct ChangeEvent {
    pub entity: Entity,
    pub change: Change
}
impl ChangeEvent {
    pub fn new(entity: Entity, change: Change) -> Self {
        Self { entity, change }
    }
}

#[derive(Clone, Debug)]
pub enum Change {
    CreateObject(MassiveObject),
    DeleteObject,
    SetPosition(DVec2),
    SetVelocity(DVec2),
    SetMass(f64),
}


/// Read change events and notify the physics worker of the change
pub fn process_change_event(
    mut events: EventReader<ChangeEvent>,
    future: Res<PhysicsFuture>,
    mut sim_state: ResMut<SimulationState>,
    mut object_query: Query<&mut VisualObjectData> // The physics worker does not report mass back to the main thread so mass changes must be reflected in the world here
) {
    if events.is_empty() { return }



    future.send_changes(events.read().map(|e| {
        if let Change::SetMass(new_mass) = e.change {
            object_query.get_mut(e.entity).unwrap().mass = new_mass;
        }
        e.clone()
    }).collect_vec());
    sim_state.current_time = 0;
}