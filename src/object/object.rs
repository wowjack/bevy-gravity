use bevy::prelude::*;

use super::{spawn::VisualObject, physics_future::PhysicsStateChangeEvent};

/*
Spatial bundle at the top level
Mesh and arrow and such as children
*/


#[derive(Component)]
pub struct MassiveObject {
    pub velocity: Vec2,
    pub mass: f64,
}

impl Default for MassiveObject {
    fn default() -> Self {
        Self { velocity: Vec2::ZERO, mass: 1. }
    }
}



#[derive(Event)]
pub struct EditObjectEvent {
    pub entity: Entity,
    pub data: EditObjectData
}

#[derive(Default, Clone)]
pub struct EditObjectData {
    pub position: Vec2,
    pub velocity: Vec2,
    pub mass: f32,
    pub radius: f32
}

pub fn edit_object(
    mut events: EventReader<EditObjectEvent>,
    mut event_writer: EventWriter<PhysicsStateChangeEvent>,
    mut object_query: Query<(&Children, &mut MassiveObject, &mut Transform), Without<VisualObject>>,
    mut visual_query: Query<&mut Transform, (With<VisualObject>, Without<MassiveObject>)>,
) {
    for event in events.read() {
        let Ok((children, mut object, mut trans)) = object_query.get_mut(event.entity) else { return };
        let Ok(mut visual_trans) = visual_query.get_mut(*children.iter().filter(|e| visual_query.contains(**e)).next().unwrap()) else { return };
        object.mass = event.data.mass as f64;
        object.velocity = event.data.velocity;
        trans.translation = event.data.position.extend(0.);
        visual_trans.scale = Vec3::splat(event.data.radius);
        event_writer.send(PhysicsStateChangeEvent);
    }
}