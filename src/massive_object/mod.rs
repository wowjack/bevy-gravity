#![allow(unused)]

use bevy::prelude::*;
use bevy_math::{DVec2, dvec2, Vec2, vec2};
use bevy_mod_picking::prelude::*;

/// MassiveObject struct for storing data.
mod object;
pub use object::*;

/// Systems for updating the position of massive objects each frame by reading from the future.
mod update;
pub use update::*;

/// Everything relating to the background physics process.
mod physics;
pub use physics::*;

/// Functionality for dragging objects to move them.
mod drag;
pub use drag::*;




mod bundle;
pub use bundle::*;

mod plugin;
pub use plugin::*;