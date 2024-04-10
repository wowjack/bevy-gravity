use super::*;

/*
Any time you want to modify a physics object, you must do so by sending an event.
The event processor will hand the changes to the physics worker.
Will it be fine if changes are only visually reflected once the worker gets them?
*/

#[derive(Event, Clone)]
pub struct ChangeEvent {
    pub entity: Entity,
    pub change: Change
}

#[derive(Clone)]
pub enum Change {
    CreateObject(MassiveObject),
    DeleteObject,
    SetPosition(DVec2),
    SetVelocity(DVec2),
    SetMass(f64),
}


/// Read change events and notify the physics worker of the change
pub fn process_change_event(mut events: EventReader<ChangeEvent>, future: Res<PhysicsFuture>) {
    if events.is_empty() { return }

    future.send_changes(events.read().map(|e| e.clone()).collect_vec());
}