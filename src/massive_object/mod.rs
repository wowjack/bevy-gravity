#![allow(unused)]

use bevy::prelude::*;
use bevy_math::{DVec2, dvec2, Vec2, vec2};
use bevy_mod_picking::prelude::*;

/// MassiveObject struct for storing data.
mod object;
pub use object::*;

/// Custom data structure for storing the future path of objects
mod physics_future;
pub use physics_future::*;

/// Background thread that does physics
mod physics_worker;
pub use physics_worker::*;

/// Function for getting the future path of objects
/// Probably make a new directory for this
mod path_prediction;
pub use path_prediction::*;

/// Systems and events for modifying massive objects
mod modify_object;
pub use modify_object::*;

/// Systems for updating the position of massive objects each frame by reading from the future.
mod update;
pub use update::*;


mod bundle;
pub use bundle::*;

mod plugin;
pub use plugin::*;