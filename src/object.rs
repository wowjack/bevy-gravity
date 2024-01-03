use bevy::{prelude::*, sprite::MaterialMesh2dBundle, transform};
use bevy_mod_picking::{PickableBundle, prelude::*, selection::{Select, Deselect}};
use bevy_prototype_lyon::prelude::*;

use crate::{ui::ObjectDetailUIContext, ArrowHandle, MainCamera, GameState};

const G: f32 = 0.0000000000667;

#[derive(Component, Default)]
pub struct MassiveObject {
    pub velocity: Vec2,
    pub mass: f32
}

#[derive(Default)]
pub struct MassiveObjectBundle {
    object: MassiveObject,
    pickable_bundle: PickableBundle,
    sprite_bundle: SpriteBundle
}

#[derive(Event)]
pub struct SpawnObjectEvent;

pub fn spawn_object(mut commands: Commands, mut events: EventReader<SpawnObjectEvent>, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>) {
    for event in events.read() {
        commands.spawn((
            MassiveObject {
                velocity: Vec2::new(0.5, 0.5),
                mass: 10000000000.
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
}

#[derive(Event)]
pub struct ObjectSelectedEvent(ListenerInput<bevy_mod_picking::prelude::Pointer<bevy_mod_picking::prelude::Select>>);
impl From<ListenerInput<bevy_mod_picking::prelude::Pointer<bevy_mod_picking::prelude::Select>>> for ObjectSelectedEvent {
    fn from(value: ListenerInput<bevy_mod_picking::prelude::Pointer<bevy_mod_picking::prelude::Select>>) -> Self {
        return Self(value)
    }
}

#[derive(Component)]
pub struct VelocityArrow;
#[derive(Component)]
pub struct ArrowTip(Entity);

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
                    mesh: bevy::sprite::Mesh2dHandle(meshes.add(Mesh::from(shape::RegularPolygon::new(1.3, 3)))),
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
    mut arrowtip_query: Query<&Parent, With<ArrowTip>>,
    mut object_query: Query<(&mut MassiveObject, &GlobalTransform)>,
) {
    let projection = projection_query.single();
    let Ok(parent) = arrowtip_query.get_mut(event.target) else { return; };
    let Ok((mut object, object_gtrans)) = object_query.get_mut(parent.get()) else { return; };

    let object_scale = object_gtrans.to_scale_rotation_translation().0;
    object.velocity.x += event.delta.x*projection.scale / object_scale.x;
    object.velocity.y -= event.delta.y*projection.scale / object_scale.y;
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



pub fn move_object(
    mut object_query: Query<(&mut Transform, &MassiveObject)>,
    state: Res<GameState>
) {
    if state.play == false { return; }

    for (mut trans, object) in object_query.iter_mut() {
        trans.translation += object.velocity.extend(0.);
    }
}

pub fn object_gravity(
    state: Res<GameState>,
    time: Res<Time>,
    mut object_query: Query<(&GlobalTransform, &mut MassiveObject)>,
) {
    if state.play == false { return; }

    let delta_time = time.delta().as_millis() as f32 / 1000.;

    let mut v: Vec<_> = object_query.iter_mut().collect();
    for i in 0..v.len() {
        let (_, c2) = v.split_at_mut(i);
        let ((trans, object), c2) = c2.split_first_mut().unwrap(); //safe

        c2.iter_mut().for_each(|(other_trans, other_obj)| {
            let force = G * object.mass * other_obj.mass / trans.translation().distance_squared(other_trans.translation());
            let angle = (trans.translation() - other_trans.translation()).truncate().angle_between(Vec2::X);

            let accel = force/object.mass; //a = f/m
            object.velocity += Vec2::new(angle.cos()*accel*-1., angle.sin()*accel) * delta_time;

            let other_accel = force/other_obj.mass; //a = f/m
            other_obj.velocity += Vec2::new(angle.cos()*other_accel, angle.sin()*other_accel*-1.) * delta_time;
        });
    }
}

pub fn update_arrow(
    mut arrow_query: Query<&mut Path, With<VelocityArrow>>,
    mut arrowtip_query: Query<(&Parent, &mut ArrowTip, &mut Transform)>,
    object_query: Query<&MassiveObject>,
) {
    arrowtip_query.iter_mut().for_each(|(object_entity, tip, mut trans)| {
        let Ok(object) = object_query.get(object_entity.get()) else { return; };
        let velocity = object.velocity;

        trans.translation.x = velocity.x;
        trans.translation.y = velocity.y;
        trans.rotation = Quat::from_rotation_z(Vec2::Y.angle_between(object.velocity));

        let Ok(mut arrow) = arrow_query.get_mut(tip.0) else { return; };
        *arrow = GeometryBuilder::build_as(&shapes::Line(Vec2::from((0., 0.)), velocity));
    });
}

pub fn path_prediction(
    mut events: EventReader<ObjectSelectedEvent>,
) {
    for _event in events.read() {
        //println!("Object selected");
    }
}