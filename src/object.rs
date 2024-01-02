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
        On::<Pointer<Select>>::send_event::<ObjectSelectedEvent>(),
    ));
}

#[derive(Event)]
pub struct ObjectSelectedEvent(ListenerInput<bevy_mod_picking::prelude::Pointer<bevy_mod_picking::prelude::Select>>);
impl From<ListenerInput<bevy_mod_picking::prelude::Pointer<bevy_mod_picking::prelude::Select>>> for ObjectSelectedEvent {
    fn from(value: ListenerInput<bevy_mod_picking::prelude::Pointer<bevy_mod_picking::prelude::Select>>) -> Self {
        return Self(value)
    }
}


pub fn object_select(
    mut events: EventReader<ObjectSelectedEvent>,
    mut detail_context: ResMut<ObjectDetailUIContext>,
    object_query: Query<&MassiveObject>,
    //arrow_asset: Res<ArrowHandle>,
    mut commands: Commands
) {
    
    for event in events.read() {
        //remove the children from the previously selected entity
        if let Some(entity) = detail_context.selected {
            commands.entity(entity).clear_children();
        }

        //change selected to the new entity
        *detail_context = ObjectDetailUIContext {
            open: true,
            selected:  Some(event.0.target)
        };
        
        //draw the velocity arrow
        let mut cmds = commands.entity(event.0.target);
        let object = object_query.get(event.0.target).unwrap();
        let vel = shapes::Line(Vec2::from((0., 0.)), object.velocity);
        cmds.with_children(|builder| {
            builder.spawn((
                ShapeBundle {
                    path: GeometryBuilder::build_as(&vel),
                    spatial: SpatialBundle {
                        transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
                        ..default()
                    },
                    ..default()
                },
                Pickable::IGNORE,
                Fill::color(Color::CYAN),
                Stroke::new(Color::BLACK, 0.05),
            ));
        });
    }
}


fn object_drag(
    event: Listener<Pointer<Drag>>,
    mut trans_query: Query<&mut Transform, With<MassiveObject>>,
    projection_query: Query<&OrthographicProjection, With<MainCamera>>,
) {
    let projection = projection_query.single();
    let Ok(mut trans) = trans_query.get_mut(event.target) else {return;};
    trans.translation.x += event.delta.x * projection.scale;
    trans.translation.y -= event.delta.y * projection.scale;
}

