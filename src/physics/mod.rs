#![allow(unused)]
use super::*;

mod path_prediction;
pub use path_prediction::*;

/// Custom data structure for storing the future path of objects
mod physics_future;
pub use physics_future::*;

mod worker;
pub use worker::*;

/// Systems and events for modifying massive objects
mod change_event;
pub use change_event::*;

/// MassiveObject struct for storing data.
mod object;
pub use object::*;


pub struct PhysicsPlugin;
impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ChangeEvent>()
            .insert_resource(PhysicsFuture::default())
            .add_systems(Update, (process_change_event));
    }
}