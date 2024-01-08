use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

use super::{velocity_arrow::{SpawnVelocityArrowEvent, VelocityArrow}, spawn::VisualObject};


#[derive(Event)]
pub struct ObjectsSelectedEvent(pub Vec<Entity>);
impl From<ListenerInput<Pointer<Select>>> for ObjectsSelectedEvent {
    fn from(value: ListenerInput<Pointer<Select>>) -> Self {
        return Self(vec![value.target])
    }
}


pub fn on_select(
    mut events: EventReader<ObjectsSelectedEvent>,
    object_query: Query<&Parent, With<VisualObject>>,
    arrow_query: Query<Entity, With<VelocityArrow>>,
    mut event_writer: EventWriter<SpawnVelocityArrowEvent>,
    mut commands: Commands
) {
    for event in events.read() {
        for e in arrow_query.into_iter() {
            commands.entity(e).despawn_recursive();
        }
        event_writer.send_batch(event.0.iter().filter_map(|e| object_query.get(*e).ok()).map(|e| SpawnVelocityArrowEvent(e.get())));
    }
}
