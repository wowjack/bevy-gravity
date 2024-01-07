use bevy::prelude::*;

use super::object::MassiveObject;

#[derive(Bundle)]
pub struct MassiveObjectBundle {
    pub spatial: SpatialBundle,
    pub object: MassiveObject
}