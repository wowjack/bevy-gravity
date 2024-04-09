
use bevy_prototype_lyon::entity::ShapeBundle;

use super::*;

/*
Since the physics future is closely tied to the position of the objects, the bundle will include a spatial bundle.
Should the object positions be updated every frame?
Call method on the physics future to get relevant object positions
*/

#[derive(Bundle)]
pub struct MassiveObjectBundle {
    shape_bundle: ShapeBundle,
    object: MassiveObject,
}

