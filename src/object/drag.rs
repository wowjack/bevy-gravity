use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

use crate::MainCamera;

use super::{object::MassiveObject, physics::physics_future::PhysicsStateChangeEvent};

#[derive(Event)]
pub struct ObjectDraggedEvent {
    pub target: Entity,
    pub delta: Vec2
}
impl From<ListenerInput<Pointer<Drag>>> for ObjectDraggedEvent {
    fn from(value: ListenerInput<Pointer<Drag>>) -> Self {
        return Self { target: value.target, delta: value.delta }
    }
}

pub fn drag_object(
    mut events: EventReader<ObjectDraggedEvent>,
    dragged_query: Query<&Parent, Without<MassiveObject>>,
    mut object_query: Query<&mut Transform, With<MassiveObject>>,
    mut event_writer: EventWriter<PhysicsStateChangeEvent>,
    camera_query: Query<&OrthographicProjection, With<MainCamera>>
) {
    if events.is_empty() { return }

    let projection = camera_query.single();

    for event in events.read() {
        let Ok(object_entity) = dragged_query.get(event.target) else { continue };
        let Ok(mut object_transform) = object_query.get_mut(object_entity.get()) else { continue };
        object_transform.translation.x += event.delta.x*projection.scale;
        object_transform.translation.y -= event.delta.y*projection.scale;
        event_writer.send(PhysicsStateChangeEvent);
    }
}