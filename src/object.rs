use bevy::{prelude::*, sprite::MaterialMesh2dBundle, transform};
use bevy_mod_picking::{PickableBundle, prelude::*, selection::{Select, Deselect}};
use bevy_prototype_lyon::prelude::*;

use crate::{ui::ObjectDetailUIContext, ArrowHandle, MainCamera};


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

#[derive(Component)]
struct VelocityArrow;
#[derive(Component)]
struct ArrowTip(Entity);

pub fn object_select(
    mut events: EventReader<ObjectSelectedEvent>,
    mut detail_context: ResMut<ObjectDetailUIContext>,
    perspective_query: Query<&OrthographicProjection>,
    object_query: Query<&MassiveObject>,
    //arrow_asset: Res<ArrowHandle>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands
) {
    let perspective = perspective_query.single();
    for event in events.read() {
        if object_query.contains(event.0.target) == false { return; }

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
        let Ok(object) = object_query.get(event.0.target) else { return; };
        let vel = shapes::Line(Vec2::from((0., 0.)), object.velocity);
        cmds.with_children(|builder| {
            let arrow_entity = builder.spawn((
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
                VelocityArrow
            )).id();


            let arrowtip_transform = Transform {
                translation: Vec3::from((object.velocity, 2.)),
                scale: Vec3::new(0.2*perspective.scale, 0.2*perspective.scale, 1.),
                rotation: Quat::from_rotation_z(Vec2::Y.angle_between(object.velocity)),
                ..Transform::default()
            };
            builder.spawn((
                MaterialMesh2dBundle {
                    mesh: bevy::sprite::Mesh2dHandle(meshes.add(Mesh::from(shape::RegularPolygon::new(1.5, 3)))),
                    transform: arrowtip_transform,
                    material: materials.add(ColorMaterial::from(Color::BLACK)),
                    ..default()
                },
                PickableBundle::default(),
                On::<Pointer<Drag>>::run(arrow_drag),
                ArrowTip(arrow_entity)
            ));
        });
    }
}

fn arrow_drag(
    event: Listener<Pointer<Drag>>,
    projection_query: Query<&OrthographicProjection, With<MainCamera>>,
    mut arrowtip_query: Query<(&Parent, &mut Transform, &ArrowTip), Without<VelocityArrow>>,
    mut object_query: Query<(&mut MassiveObject, &GlobalTransform)>,
    mut arrow_query: Query<&mut Path, (With<VelocityArrow>, Without<MassiveObject>)>
) {
    let projection = projection_query.single();
    let Ok((parent, mut arrowtip_trans, ArrowTip(arrow_entity))) = arrowtip_query.get_mut(event.target) else { return; };
    let Ok((mut object, object_gtrans)) = object_query.get_mut(parent.get()) else { return; };
    let Ok(mut arrow) = arrow_query.get_mut(*arrow_entity) else { return; };

    let object_scale = object_gtrans.to_scale_rotation_translation().0;
    arrowtip_trans.translation.x += event.delta.x*projection.scale / object_scale.x;
    arrowtip_trans.translation.y -= event.delta.y*projection.scale / object_scale.y;
    arrowtip_trans.rotation = Quat::from_rotation_z(Vec2::Y.angle_between(object.velocity));

    let velocity = arrowtip_trans.translation.truncate();
    object.velocity = velocity;
    *arrow = GeometryBuilder::build_as(&shapes::Line(Vec2::from((0., 0.)), velocity));
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

