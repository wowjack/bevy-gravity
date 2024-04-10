
use bevy_prototype_lyon::{entity::ShapeBundle, prelude::*};

use crate::physics::MassiveObject;

use super::*;

#[derive(Bundle)]
pub struct VisualObjectBundle {
    object: MassiveObject,
    shape_bundle: ShapeBundle
}
impl Default for VisualObjectBundle {
    fn default() -> Self {
        Self {
            object: Default::default(),
            shape_bundle: ShapeBundle {
                path: GeometryBuilder::build_as(&shapes::RegularPolygon { sides: 30, center: Vec2::ZERO, feature: RegularPolygonFeature::Radius(1.)}),
                ..Default::default()
            }
        }
    }
}
impl VisualObjectBundle {
    pub fn from_object(object: MassiveObject) -> Self {
        Self { object, ..default() }
    }
}

