#![allow(unused)]

use bevy::prelude::*;
use bevy_math::{DVec2, dvec2, Vec2, vec2};
use bevy_mod_picking::prelude::*;

/// Systems for updating the position of massive objects each frame by reading from the future.
mod update;
mod Velocity_arrow;
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

mod velocity_arrow;
pub use velocity_arrow::*;



pub struct VisualObjectPlugin;
impl Plugin for VisualObjectPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<AppearanceChangeEvent>()
            .insert_resource(SimulationState::default())
            .add_systems(Startup, init)
            .add_systems(Update, (
                update_object_positions,
                draw_velocity_arrows.after(update_object_positions)
            ));
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
    mut config_store: ResMut<GizmoConfigStore>
) {
    config_store.config_mut::<DefaultGizmoConfigGroup>().0.line_width = 5.;

    let circle_mesh = meshes.add(RegularPolygon::new(1., 50));
    let default_color = colors.add(ColorMaterial { color: Color::BISQUE, texture: None });

    commands.insert_resource(CircleAssets { mesh: circle_mesh, default_color });
}