use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

use super::{velocity_arrow::{SpawnVelocityArrowEvent, VelocityArrow}, spawn::VisualObject};

#[derive(Resource, Default)]
pub struct SelectedObjects {
    pub selected: Vec<Entity>,
    pub focused: Option<Entity>
}

#[derive(Event, Clone)]
pub struct ObjectsSelectedEvent {
    pub entities: Vec<Entity>,
    pub deselect: bool
}


//make it so objects selected event processing doesnt anticipate targets pointing to children of massive objects
pub fn on_select(
    mut events: EventReader<ObjectsSelectedEvent>,
    arrow_query: Query<Entity, With<VelocityArrow>>,
    mut event_writer: EventWriter<SpawnVelocityArrowEvent>,
    mut commands: Commands,
    mut selected_objects: ResMut<SelectedObjects>
) {
    for event in events.read() {
        let mut event = event.clone();
        if event.entities.len() == 1 && selected_objects.selected.contains(&event.entities[0]) {
            event.deselect = false;
            selected_objects.focused = Some(event.entities[0]);
        }

        if event.deselect {
            for e in arrow_query.into_iter() {
                commands.entity(e).despawn_recursive();
            }
            selected_objects.selected = event.entities.clone();
            selected_objects.focused = None;
            event_writer.send_batch(event.entities.iter().map(|e| SpawnVelocityArrowEvent(*e)));
        } else {
            let new_entities: Vec<Entity> = event.entities.clone().into_iter().filter(|e| !selected_objects.selected.contains(e)).collect();
            selected_objects.selected.extend(new_entities.iter());
            event_writer.send_batch(new_entities.into_iter().map(|e| SpawnVelocityArrowEvent(e)));
        }
        
        if event.entities.len() == 1 {
            selected_objects.focused = Some(event.entities[0]);
        }
    }
}
