use bevy::prelude::*;
use bevy_mod_picking::prelude::*;


#[derive(Event)]
pub struct ObjectsSelectedEvent(pub Vec<Entity>);
impl From<ListenerInput<bevy_mod_picking::prelude::Pointer<bevy_mod_picking::prelude::Select>>> for ObjectsSelectedEvent {
    fn from(value: ListenerInput<bevy_mod_picking::prelude::Pointer<bevy_mod_picking::prelude::Select>>) -> Self {
        return Self(vec![value.target])
    }
}


pub fn on_select(
    mut events: EventReader<ObjectsSelectedEvent>,
    mut commands: Commands
) {
    for event in events.read() {
        for entity in event.0.iter() {
            let Some(mut commands) = commands.get_entity(*entity) else { continue; };
            commands.with_children(|builder| {
                builder.spawn(SpriteBundle::default());
            });
        }
    }
}
