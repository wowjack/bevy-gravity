use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use super::{object_bundle::MassiveObjectBundle, object::MassiveObject, ObjectResources, physics_future::PhysicsStateChange};


#[derive(Event, Default)]
pub struct SpawnObjectEvent {
    pub position: Vec2,
    pub velocity: Vec2,
    pub mass: f64,
    pub radius: f32,
}

pub fn spawn_objects(
    mut events: EventReader<SpawnObjectEvent>,
    mut commands: Commands,
    resources: Res<ObjectResources>,
    mut physics_event_writer: EventWriter<PhysicsStateChange>,
) {
    if events.is_empty() {return }

    for event in events.read() {
        commands.spawn(MassiveObjectBundle {
            spatial: SpatialBundle::from_transform(Transform::from_translation(Vec3::from((event.position, 0.)))),
            object: MassiveObject { velocity: event.velocity, mass: event.mass },  
        }).with_children(|builder| {
            builder.spawn(MaterialMesh2dBundle {
                material: resources.circle_material.clone().unwrap(),
                mesh: resources.circle_mesh.clone().unwrap().into(),
                ..default()
            });
        });
    }
    physics_event_writer.send(PhysicsStateChange);
}