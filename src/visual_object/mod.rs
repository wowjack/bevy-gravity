
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
mod visual_change_event;
pub use visual_change_event::*;
mod spawn;
pub use spawn::*;
mod reference_frame;
pub use reference_frame::*;
mod follow_object;
pub use follow_object::*;

pub const CIRCLE_VERTICES: usize = 100;

/// Info about how and where to draw visual objects
#[derive(Default, Component, Clone)]
pub struct VisualObjectData {
    pub position: DVec2,
    pub velocity: DVec2,
    pub mass: f64,
    pub radius: f32,
    pub color: Color,
}
impl VisualObjectData {
    pub fn new(position: DVec2, velocity: DVec2, mass: f64, radius: f32, color: Color) -> Self {
        Self { position, velocity, mass, radius, color }
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
            .add_event::<VisualChangeEvent>()
            .insert_resource(SimulationState::default())
            .insert_resource(DrawOptions::default())
            .insert_resource(SelectedObjects::default())
            .insert_resource(ReferenceFrameResource::default())
            .insert_resource(FollowObjectResource::default())
            .add_systems(Startup, (init, spawn_background_rect))
            .add_systems(PreUpdate, update_object_data)
            .add_systems(Update, (
                move_pseudo_camera,
                draw_selection_rect,
                rect_select,
                draw_selected_object_halo,
                draw_ref_object_halo.after(draw_selected_object_halo),
                update_focused_object_data,
                process_visual_change_event,
                update_object_positions,
                draw_future_paths.after(update_object_positions),
                draw_velocity_arrows.after(update_object_positions),
                draw_mini_object_point.after(update_object_positions),
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