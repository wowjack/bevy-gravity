use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_mod_picking::prelude::*;

use super::{object_bundle::MassiveObjectBundle, object::MassiveObject, ObjectResources, physics_future::PhysicsStateChangeEvent, select::ObjectsSelectedEvent, drag::ObjectDraggedEvent};


#[derive(Event, Copy, Clone)]
pub struct SpawnObjectEvent {
    pub position: Vec2,
    pub velocity: Vec2,
    pub mass: f64,
    pub radius: f32,
}
impl Default for SpawnObjectEvent {
    fn default() -> Self {
        Self { position: Vec2::ZERO, velocity: Vec2::ZERO, mass: 1., radius: 1. }
    }
}

pub fn spawn_objects(
    mut events: EventReader<SpawnObjectEvent>,
    mut commands: Commands,
    resources: Res<ObjectResources>,
    mut physics_event_writer: EventWriter<PhysicsStateChangeEvent>,
) {
    if events.is_empty() {return }

    for event in events.read() {
        commands.spawn((
            MassiveObjectBundle {
                spatial: SpatialBundle::from_transform(Transform::from_translation(Vec3::from((event.position, 0.)))),
                object: MassiveObject { velocity: event.velocity, mass: event.mass },
                ..default()
            },
        )).with_children(|builder| {
            builder.spawn((
                MaterialMesh2dBundle {
                    material: resources.circle_material.clone().unwrap(),
                    mesh: resources.circle_mesh.clone().unwrap().into(),
                    transform: Transform::from_scale(Vec3::new(event.radius, event.radius, 1.)),
                    ..default()
                },
                PickableBundle::default(),
                On::<Pointer<Select>>::run(|mut event: ListenerMut<Pointer<Select>>, object_query: Query<&Parent, With<VisualObject>>, mut event_writer: EventWriter<ObjectsSelectedEvent>| {
                    event.stop_propagation();
                    let Ok(parent) = object_query.get(event.target) else { return };
                    event_writer.send(ObjectsSelectedEvent{ entities: vec![parent.get()], deselect: true });
                }),
                VisualObject
            ));
        });
    }
    physics_event_writer.send(PhysicsStateChangeEvent);
}


#[derive(Component)]
pub struct VisualObject;