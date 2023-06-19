use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_mod_picking::{PickableBundle, prelude::{RaycastPickTarget, OnPointer, ListenedEvent, Bubble, Drag, PointerEvent}, selection::{Select, Deselect}};
use bevy_prototype_lyon::prelude::*;

use crate::{ui::ObjectDetailUIContext, ArrowHandle};

pub struct ObjectDragEvent {
    entity: Entity,
    position: Vec2
}

#[derive(Component, Default)]
pub struct MassiveObject {
    pub velocity: Vec2,
}

#[derive(Default)]
pub struct MassiveObjectBundle {
    object: MassiveObject,
    pickable_bundle: PickableBundle,
    pick_target: RaycastPickTarget,
    sprite_bundle: SpriteBundle
}

pub fn spawn_object(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn((
        MassiveObject {
            velocity: Vec2::new(0.5, 0.5)
        },
        PickableBundle::default(),
        RaycastPickTarget::default(),
        MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Circle::default())).into(),
            transform: Transform::default().with_translation(Vec3::from([20., 20., 0.])).with_scale(Vec3::splat(40.)),
            material: materials.add(ColorMaterial::from(Color::PURPLE)),
            ..default()
        },
        OnPointer::<Drag>::run_callback(|In(event): In<ListenedEvent<Drag>>, mut events: EventWriter<ObjectDragEvent>| {
            events.send(ObjectDragEvent {
                entity: event.target,
                position: event.pointer_location.position
            });
            return Bubble::Up;
        }),
    ));
}

pub fn handle_object_drag(mut events: EventReader<ObjectDragEvent>, mut objects: Query<&mut Transform, With<MassiveObject>>, camera: Query<(&Camera, &GlobalTransform)>) {
    let camera = camera.get_single().unwrap();
    for e in events.iter() {
        let pos = camera.0.viewport_to_world_2d(camera.1, e.position).unwrap();
        let mut transform = objects.get_mut(e.entity).unwrap();
        transform.translation = Vec3::from((pos, 0.));
    }
}



pub fn object_selected(mut events: EventReader<PointerEvent<Select>>, mut detail_context: ResMut<ObjectDetailUIContext>, arrow_asset: Res<ArrowHandle>, mut commands: Commands) {
    for e in events.iter() {
        //open the ui window
        *detail_context = ObjectDetailUIContext {
            open: true,
            selected:  Some(e.target)
        };

        //draw the velocity arrow
        if let Some(mut cmds) = commands.get_entity(e.target) {
            let vel = shapes::Line(Vec2::from((0., 0.)), Vec2::from((1., 1.)));
            cmds.with_children(|builder| {
                builder.spawn((
                    ShapeBundle {
                        path: GeometryBuilder::build_as(&vel),
                        ..default()
                    },
                    Fill::color(Color::CYAN),
                    Stroke::new(Color::BLACK, 0.05),
                ));
            });
        }
    }
}
