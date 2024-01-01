use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_mod_picking::{PickableBundle, prelude::*, selection::{Select, Deselect}};
use bevy_prototype_lyon::prelude::*;

use crate::{ui::ObjectDetailUIContext, ArrowHandle, MainCamera};

#[derive(Event)]
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
    sprite_bundle: SpriteBundle
}

pub fn spawn_object(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn((
        MassiveObject {
            velocity: Vec2::new(0.5, 0.5)
        },
        PickableBundle::default(),
        MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Circle::default())).into(),
            transform: Transform::default().with_translation(Vec3::from([20., 20., 0.])).with_scale(Vec3::splat(40.)),
            material: materials.add(ColorMaterial::from(Color::PURPLE)),
            ..default()
        },
        On::<Pointer<Drag>>::run(object_drag),
        On::<Pointer<Select>>::run(|event: Listener<Pointer<Select>>, mut detail_context: ResMut<ObjectDetailUIContext>| {
            *detail_context = ObjectDetailUIContext {
                open: true,
                selected:  Some(event.target)
            };
        }),
    ));
}

fn object_drag(
    event: Listener<Pointer<Drag>>,
    mut trans_query: Query<&mut Transform, With<MassiveObject>>,
    projection_query: Query<&OrthographicProjection, With<MainCamera>>,
) {
    let projection = projection_query.single();
    let mut trans = trans_query.get_mut(event.target).unwrap();
    trans.translation.x += event.delta.x * projection.scale;
    trans.translation.y -= event.delta.y * projection.scale;
}
/*
pub fn object_selected(
    mut events: EventReader<PointerEvent<Select>>,
    mut detail_context: ResMut<ObjectDetailUIContext>,
    arrow_asset: Res<ArrowHandle>,
    mut commands: Commands
) {
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
*/
