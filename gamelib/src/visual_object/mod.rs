
use bevy::{math::DVec2, prelude::*};
use bevy_mod_picking::prelude::*;

/// Systems for updating the position of massive objects each frame by reading from the future.
mod update;
pub use update::*;
/// Bundle for easily creating objects.
mod bundle;
pub use bundle::*;
/// Controls the world's communication with the physics worker.
mod sim_state;
pub use sim_state::*;
mod velocity_arrow;
pub use velocity_arrow::*;
mod future_path;
pub use future_path::*;
mod mini_object_point;
pub use mini_object_point::*;
mod draw_options;
pub use draw_options::*;
mod select_object;
pub use select_object::*;
mod spawn;
pub use spawn::*;
mod follow_object;
pub use follow_object::*;

use crate::{gravity_system_tree::{dynamic_body::DynamicBody, static_body::StaticBody}, path_calculator::draw_path, G};

pub const CIRCLE_VERTICES: usize = 100;

/// Info about how and where to draw visual objects
#[derive(Default, Component, Clone)]
pub struct VisualObjectData {
    pub position: DVec2,
    pub velocity: DVec2,
    pub mass: f64,
    pub radius: f64,
    pub color: Color,
    pub name: String,
}
impl VisualObjectData {
    pub fn from_dynamic_body(dynamic_body: &DynamicBody) -> Self {
        Self {
            position: dynamic_body.get_interpolated_absolute_position(0.),
            velocity: dynamic_body.get_interpolated_relative_velocity(0.),
            mass: dynamic_body.get_mass(),
            radius: dynamic_body.get_radius(),
            color: dynamic_body.get_color(),
            name: dynamic_body.get_name(),
        }
    }

    pub fn from_static_body(static_body: &StaticBody) -> Self {
        Self {
            position: static_body.get_absolute_position(),
            velocity: static_body.get_relative_velocity(),
            mass: static_body.get_mass(),
            radius: static_body.get_radius(),
            color: static_body.get_color(),
            name: static_body.get_name()
        }
    }
}

/// All circles must have their own unchanging mesh or else bevy_mod_picking doesn't work correctly.
/// So all circles share a mesh and just use their scale to change radius
#[derive(Resource)]
pub struct CircleMesh(pub Handle<Mesh>);

pub struct VisualObjectPlugin;
impl Plugin for VisualObjectPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SelectInRectEvent>()
            .init_gizmo_group::<FuturePathLineConfig>()
            .insert_resource(SimulationState::default())
            .insert_resource(DrawOptions::default())
            .insert_resource(SelectedObjects::default())
            .insert_resource(FollowObjectResource::default())
            .add_systems(Startup, (init, spawn_background_rect, set_future_path_gizmo_config))
            .add_systems(PreUpdate, (update_object_data, update_object_positions.after(update_object_data)))
            .add_systems(Update, (
                draw_path,
                add_material_mesh,
                move_pseudo_camera,
                draw_selection_rect,
                rect_select,
                draw_selected_object_halo,
                update_focused_object_data,
                draw_velocity_arrows,
                draw_mini_object_point,
            ));
    }
}

fn init(
    mut config_store: ResMut<GizmoConfigStore>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut commands: Commands
) {
    config_store.config_mut::<DefaultGizmoConfigGroup>().0.line_width = 2.;

    commands.insert_resource(
        CircleMesh(meshes.add(bevy::math::prelude::RegularPolygon::new(1., CIRCLE_VERTICES)))
    );
}