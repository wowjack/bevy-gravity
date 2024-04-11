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

/// Bundle for easily creating objects.
mod bundle;
pub use bundle::*;

/// Controls the world's communication with the physics worker.
mod sim_state;
pub use sim_state::*;

/// The appearance of objects in the world
mod appearance;
pub use appearance::*;



pub struct VisualObjectPlugin;
impl Plugin for VisualObjectPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<AppearanceChangeEvent>()
            .insert_resource(SimulationState::default())
            .add_systems(Startup, init)
            .add_systems(Update, (update_object_positions, process_appearance_change_event));
    }
}



#[derive(Resource)]
pub struct CircleAssets{
    mesh: Handle<Mesh>,
    default_color: Handle<ColorMaterial>,
}

fn init(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut colors: ResMut<Assets<ColorMaterial>>,
) {
    let circle_mesh = meshes.add(RegularPolygon::new(1., 50));
    let default_color = colors.add(ColorMaterial { color: Color::BISQUE, texture: None });

    commands.insert_resource(CircleAssets { mesh: circle_mesh, default_color });
}