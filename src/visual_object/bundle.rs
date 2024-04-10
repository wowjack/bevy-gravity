
use bevy_prototype_lyon::{entity::ShapeBundle, prelude::*};

use crate::physics::MassiveObject;

use super::*;

#[derive(Bundle, Default)]
pub struct VisualObjectBundle {
    object: MassiveObject,
    appearance: AppearanceBundle
}
impl VisualObjectBundle {
    pub fn new(object: MassiveObject, radius: f32, color: Color) -> Self {
        Self { object, appearance: AppearanceBundle::new(radius, color) }
    }
    pub fn from_object(object: MassiveObject) -> Self {
        Self { object, ..default() }
    }
}

