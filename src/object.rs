use bevy::{prelude::*, input::keyboard::KeyboardInput, sprite::MaterialMesh2dBundle};
use bevy_mod_picking::{PickableBundle, prelude::*, selection::{Select, Deselect}};
use bevy_prototype_lyon::prelude::*;

use crate::{ui::{ObjectDetailContext, ObjectDetailState}, MainCamera, GameState, GameResources};

const G: f32 = 0.0000000000667;
pub const VELOCITY_ARROW_SCALE: f32 = 50.;

#[derive(Component, Default)]
pub struct MassiveObject {
    pub velocity: Vec2,
    pub mass: f32,
    pub radius: f32
}

#[derive(Default)]
pub struct MassiveObjectBundle {
    pub object: MassiveObject,
    pub pickable_bundle: PickableBundle,
    pub sprite_bundle: SpriteBundle
}

#[derive(Resource)]
pub struct ObjectSpawnOptions {
    pub position: Vec2,
    pub velocity: Vec2,
    pub mass: f32,
    pub radius: f32,
}
impl Default for ObjectSpawnOptions {
    fn default() -> Self {
        Self { position: Vec2::ZERO, velocity: Vec2::ZERO, mass: 10_000., radius: 10. }
    }
}

#[derive(Event)]
pub struct SpawnObjectEvent;

pub fn spawn_object(mut commands: Commands, mut events: EventReader<SpawnObjectEvent>, game_resources: Res<GameResources>, spawn_options: Res<ObjectSpawnOptions>) {
    for _ in events.read() {
        commands.spawn((
            MassiveObject {
                velocity: spawn_options.velocity,
                mass: spawn_options.mass,
                radius: spawn_options.radius,
            },
            PickableBundle::default(),
            MaterialMesh2dBundle {
                mesh: game_resources.circle_mesh.clone().unwrap_or_default(),
                material: game_resources.circle_material.clone().unwrap(),
                transform: Transform {
                    translation: Vec3::from((spawn_options.position, 1.)),
                    scale: Vec3::splat(spawn_options.radius),
                    ..default()
                },
                ..default()
            },
            On::<Pointer<Drag>>::run(object_drag),
            On::<Pointer<Select>>::send_event::<ObjectSelectedEvent>(),
        ));
    }
}

#[derive(Event)]
pub struct ObjectSelectedEvent(pub ListenerInput<bevy_mod_picking::prelude::Pointer<bevy_mod_picking::prelude::Select>>);
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
    mut detail_context: ResMut<ObjectDetailContext>,
    perspective_query: Query<&OrthographicProjection>,
    object_query: Query<(&MassiveObject, &GlobalTransform)>,
    mut commands: Commands
) {
    let perspective = perspective_query.single();
    for event in events.read() {
        if object_query.contains(event.0.target) == false { return; }

        //remove the children from the previously selected entity
        if let Some(entity) = detail_context.selected {
            commands.entity(entity).despawn_descendants();
        }

        //change selected to the new entity
        *detail_context = ObjectDetailContext {
            open: true,
            selected:  Some(event.0.target),
        };
        
        //draw the velocity arrow
        let mut cmds = commands.entity(event.0.target);
        let Ok((object, gtrans)) = object_query.get(event.0.target) else { return; };
        let scaled_velocity = object.velocity / gtrans.to_scale_rotation_translation().0.truncate();
        let vel = shapes::Line(Vec2::from((0., 0.)), scaled_velocity);
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
                Stroke::new(Color::BLACK, 2.*perspective.scale),
                VelocityArrow
            )).id();


            let arrowtip_transform = Transform {
                translation: Vec3::from((scaled_velocity, 2.)),
                scale: Vec3::new(10.*perspective.scale, 10.*perspective.scale, 1.) / gtrans.to_scale_rotation_translation().0,
                rotation: Quat::from_rotation_z(Vec2::Y.angle_between(object.velocity)),
                ..Transform::default()
            };
            builder.spawn((
                ShapeBundle {
                    path: GeometryBuilder::build_as(&shapes::Polygon { closed: true, points: vec![
                        Vec2::new(0.,0.3), Vec2::new(-1.,-0.7), Vec2::new(-0.7, -1.), Vec2::new(0., -0.3), Vec2::new(0.7, -1.), Vec2::new(1., -0.7)
                    ]}),
                    spatial: SpatialBundle::from_transform(arrowtip_transform),
                    ..default()
                },
                PickableBundle::default(),
                On::<Pointer<Drag>>::run(arrow_drag),
                On::<Pointer<Select>>::run(|mut event: ListenerMut<Pointer<Select>>| event.stop_propagation()),
                ArrowTip(arrow_entity)
            ));
        });
    }
}

fn arrow_drag(
    event: Listener<Pointer<Drag>>,
    projection_query: Query<&OrthographicProjection, With<MainCamera>>,
    mut arrowtip_query: Query<&Parent, With<ArrowTip>>,
    mut object_query: Query<&mut MassiveObject>,
) {
    let projection = projection_query.single();
    let Ok(parent) = arrowtip_query.get_mut(event.target) else { return; };
    let Ok(mut object) = object_query.get_mut(parent.get()) else { return; };

    object.velocity.x += event.delta.x*projection.scale/VELOCITY_ARROW_SCALE;
    object.velocity.y -= event.delta.y*projection.scale/VELOCITY_ARROW_SCALE;
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
    mut arrow_query: Query<(&mut Path, &mut Stroke), With<VelocityArrow>>,
    mut arrowtip_query: Query<(&Parent, &mut ArrowTip, &mut Transform)>,
    object_query: Query<(&MassiveObject, &GlobalTransform)>,
    perspective_query: Query<&OrthographicProjection>,
) {
    let perspective = perspective_query.single();

    arrowtip_query.iter_mut().for_each(|(object_entity, tip, mut trans)| {
        let Ok((object, gtrans)) = object_query.get(object_entity.get()) else { return; };
        let velocity = object.velocity * VELOCITY_ARROW_SCALE / gtrans.to_scale_rotation_translation().0.truncate();

        trans.translation.x = velocity.x;
        trans.translation.y = velocity.y;
        trans.rotation = Quat::from_rotation_z(Vec2::Y.angle_between(velocity));
        trans.scale = Vec3::splat(10.*perspective.scale) / gtrans.to_scale_rotation_translation().0;

        let Ok((mut arrow, mut arrow_stroke)) = arrow_query.get_mut(tip.0) else { return; };
        *arrow = GeometryBuilder::build_as(&shapes::Line(Vec2::from((0., 0.)), velocity));
        *arrow_stroke = Stroke::new(Color::BLACK, 5.*perspective.scale / gtrans.to_scale_rotation_translation().0.xy().length());
    });
}




pub fn update_object_radius(
    mut object_query: Query<(&MassiveObject, &mut Transform)>,
) {
    
    for (object, mut trans) in  object_query.iter_mut() {
        trans.scale = Vec3::splat(object.radius);
    }
    
}

pub fn escape_unselect(
    keyboard_input: Res<Input<KeyCode>>,
    mut object_detail_ui: ResMut<ObjectDetailContext>,
    mut commands: Commands
) {
    if keyboard_input.pressed(KeyCode::Escape) {
        let Some(selected_entity) = object_detail_ui.selected else { return }; 
        commands.entity(selected_entity).despawn_descendants();
        object_detail_ui.open = false;
        object_detail_ui.selected = None;
    }
}

pub fn follow_object(
    detail_state: Res<ObjectDetailState>,
    detail_context: Res<ObjectDetailContext>,
    object_query: Query<&Transform, (With<MassiveObject>, Without<MainCamera>)>,
    mut camera_query: Query<&mut Transform, With<MainCamera>>,
) {
    if detail_state.follow_selected == false{ return }

    let Some(entity) = detail_context.selected else { return };
    let Ok(object_transform) = object_query.get(entity) else { return };
    let mut camera_transform = camera_query.single_mut();
    camera_transform.translation.x = object_transform.translation.x;
    camera_transform.translation.y = object_transform.translation.y;
}