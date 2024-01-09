use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

use super::{object::MassiveObject, drag::ObjectDraggedEvent};

#[derive(Bundle)]
pub struct MassiveObjectBundle {
    pub spatial: SpatialBundle,
    pub object: MassiveObject,
    pub on_drag: On::<Pointer<Drag>>,
}

impl Default for MassiveObjectBundle {
    fn default() -> Self {
        Self {
            on_drag: On::<Pointer<Drag>>::send_event::<ObjectDraggedEvent>(),
            spatial: default(),
            object: default()
        }
    }
}