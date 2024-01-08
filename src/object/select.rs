use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

use super::{velocity_arrow::{SpawnVelocityArrowEvent, VelocityArrow}, spawn::VisualObject};

#[derive(Resource, Default)]
pub struct SelectedObjects {
    pub selected: Vec<Entity>,
    pub focused: Option<Entity>
}

#[derive(Event)]
pub struct ObjectsSelectedEvent(pub Vec<Entity>);


//make it so objects selected event processing doesnt anticipate targets pointing to children of massive objects
pub fn on_select(
    mut events: EventReader<ObjectsSelectedEvent>,
    arrow_query: Query<Entity, With<VelocityArrow>>,
    mut event_writer: EventWriter<SpawnVelocityArrowEvent>,
    mut commands: Commands,
    mut selected_objects: ResMut<SelectedObjects>
) {
    for event in events.read() {
        for e in arrow_query.into_iter() {
            commands.entity(e).despawn_recursive();
        }
        selected_objects.selected = event.0.clone();
        selected_objects.focused = None;
        event_writer.send_batch(event.0.iter().map(|e| SpawnVelocityArrowEvent(*e)));
    }
}
