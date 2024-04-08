use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use bevy_prototype_lyon::{entity::{ShapeBundle, Path}, geometry::GeometryBuilder, shapes, draw::{Fill, Stroke}};

use crate::MainCamera;

use super::{object::MassiveObject, physics::physics_future::PhysicsStateChangeEvent};

const VELOCITY_SCALE: f32 = 100.;

#[derive(Event)]
pub struct SpawnVelocityArrowEvent(pub Entity);

#[derive(Component)]
pub struct VelocityArrow(pub Entity);

#[derive(Component)]
pub struct ArrowShaft(pub Entity);

#[derive(Component)]
pub struct ArrowTip(pub Entity);


pub fn spawn_velocity_arrow(
    mut commands: Commands,
    mut events: EventReader<SpawnVelocityArrowEvent>,
    projection_query: Query<&OrthographicProjection, With<MainCamera>>,
    object_query: Query<&MassiveObject>,
) {
    return;
    if events.is_empty() { return }

    let projection = projection_query.single();

    for event in events.read() {
        let Ok(obj) = object_query.get(event.0) else { continue };
        commands.entity(event.0)
                .with_children(|builder| {
                    builder.spawn((SpatialBundle::default(), VelocityArrow(event.0)))
                           .with_children(|builder| {
                                builder.spawn((
                                    ShapeBundle {
                                        path: GeometryBuilder::build_as(&shapes::Line(Vec2::new(0.,0.), obj.velocity * VELOCITY_SCALE)),
                                        spatial: SpatialBundle::from_transform(Transform::from_translation(Vec3::new(0.,0.,1.))),
                                        ..default()
                                    },
                                    Fill::color(Color::BLACK),
                                    Stroke::new(Color::BLACK, 2.*projection.scale),
                                    Pickable::IGNORE,
                                    ArrowShaft(event.0),
                                ));
                                builder.spawn((
                                    ShapeBundle {
                                        path: GeometryBuilder::build_as(&shapes::Polygon { 
                                            closed: true,
                                            points: vec![(0.,0.1).into(), (-0.5,-0.9).into(), (0.,-0.6).into(), (0.5,-0.9).into()]
                                        }),
                                        spatial: SpatialBundle {
                                            transform: Transform {
                                                translation: (obj.velocity * VELOCITY_SCALE).extend(1.),
                                                scale: Vec3::new(1.,1.,1.) * 15. * projection.scale,
                                                rotation: Quat::from_rotation_z(Vec2::Y.angle_between(obj.velocity))
                                            },
                                            ..default()
                                        },
                                        ..default()
                                    },
                                    Fill::color(Color::BLACK),
                                    PickableBundle::default(),
                                    On::<Pointer<Drag>>::run(drag_arrowtip),
                                    On::<Pointer<Select>>::run(|mut e: ListenerMut<Pointer<Select>>| e.stop_propagation()),
                                    ArrowTip(event.0)
                                ));
                           });
                });
    }
}


pub fn update_velocity_arrow(
    object_query: Query<&MassiveObject>,
    mut arrowtip_query: Query<(&mut Transform, &ArrowTip)>,
    mut arrowshaft_query: Query<(&mut Path, &mut Stroke, &ArrowShaft)>,
    projection_query: Query<&OrthographicProjection, With<MainCamera>>,
) {
    return;
    if arrowtip_query.is_empty() { return }

    let projection = projection_query.single();

    for (mut trans, arrowtip) in arrowtip_query.iter_mut() {
        let Ok(object) = object_query.get(arrowtip.0) else { continue };
        trans.translation = (object.velocity * VELOCITY_SCALE).extend(1.);
        trans.scale = Vec3::new(15.,15.,15.) * projection.scale;
        trans.rotation = Quat::from_rotation_z(Vec2::Y.angle_between(object.velocity));
    }

    for (mut path, mut stroke, arrowshaft) in arrowshaft_query.iter_mut() {
        let Ok(object) = object_query.get(arrowshaft.0) else { continue };
        *path = GeometryBuilder::build_as(&shapes::Line(Vec2::ZERO, object.velocity*VELOCITY_SCALE));
        *stroke = Stroke::new(Color::BLACK, 2.*projection.scale);
    }

}

pub fn drag_arrowtip(
    event: Listener<Pointer<Drag>>,
    arrow_query: Query<&ArrowTip>,
    mut object_query: Query<&mut MassiveObject>,
    projection_query: Query<&OrthographicProjection, With<MainCamera>>,
    mut event_writer: EventWriter<PhysicsStateChangeEvent>
) {
    let Ok(ArrowTip(e)) = arrow_query.get(event.target) else { return };
    let Ok(mut object) = object_query.get_mut(*e) else { return };
    let projection = projection_query.single();
    let scaled_delta = event.delta * (projection.scale / VELOCITY_SCALE);
    object.velocity.x += scaled_delta.x;
    object.velocity.y -= scaled_delta.y;
    event_writer.send(PhysicsStateChangeEvent);
}