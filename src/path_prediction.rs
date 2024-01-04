use bevy::prelude::*;
use bevy_prototype_lyon::{entity::ShapeBundle, geometry::GeometryBuilder, shapes, draw::{Fill, Stroke}};

use crate::object::{ObjectSelectedEvent, MassiveObject};



#[derive(Component, Default)]
pub struct PathPrediction;


pub fn path_prediction(
    mut events: EventReader<ObjectSelectedEvent>,
    object_query: Query<(&MassiveObject, &Transform), With<MassiveObject>>,
    mut commands: Commands
) {
    return;
    for event in events.read() {
        commands.entity(event.0.target).with_children(|builder| {
            builder.spawn((SpatialBundle::default(), PathPrediction))
                   .with_children(|builder| {
                        builder.spawn((
                            ShapeBundle {
                                path: GeometryBuilder::build_as(&shapes::Line(Vec2::from((0., 0.)), Vec2::from((0., 3.)))),
                                ..default()
                            },
                            Fill::color(Color::CYAN),
                            Stroke::new(Color::BLACK, 0.05),
                        ));
                   });  
        });
    }
}

