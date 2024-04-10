#![allow(unused)]

use bevy::prelude::*;
use bevy_math::{DVec2, dvec2, Vec2, vec2};
use bevy_mod_picking::prelude::*;

/// Systems for updating the position of massive objects each frame by reading from the future.
mod update;
pub use update::*;

/// Functionality for dragging objects to move them.
mod drag;
pub use drag::*;

mod bundle;
pub use bundle::*;

mod sim_state;
pub use sim_state::*;



pub struct VisualObjectPlugin;
impl Plugin for VisualObjectPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SimulationState::default())
            .add_systems(Update, (update_object_positions));
    }
}