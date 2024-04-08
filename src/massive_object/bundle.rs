
use super::*;

/*
Since the physics future is closely tied to the position of the objects, the bundle will include spatial bundle
*/

#[derive(Bundle)]
pub struct MassiveObjectBundle {
    spatial: SpatialBundle,
    object: MassiveObject
}

