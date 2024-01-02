use bevy_egui::{egui::{self, Frame, epaint::Shadow, Color32, Stroke, Margin, Align2, Rounding}, EguiContexts};
use bevy::prelude::*;

use crate::{object::{MassiveObject, spawn_object}, MainCamera};


#[derive(bevy::prelude::Resource, Default)]
pub struct ObjectDetailUIContext {
    pub open: bool,
    pub selected: Option<bevy::prelude::Entity>,
}
pub fn ui_example_system(
    mut contexts: EguiContexts,
    mut resource: bevy::prelude::ResMut<ObjectDetailUIContext>,
    mut query: Query<(&GlobalTransform, &Transform, &mut MassiveObject)>,
    camera_query: Query<(&GlobalTransform, &Camera, &OrthographicProjection), With<MainCamera>>,
    window_query: Query<&Window>,
) {
    let window = window_query.get_single().unwrap();
    let (camera_transform, camera, projection) = camera_query.get_single().unwrap();
    let window_frame = Frame {
        inner_margin: Margin::same(5.),
        rounding: Rounding::same(4.),
        shadow: Shadow::NONE,
        fill: Color32::DARK_GRAY,
        stroke: Stroke::new(1., Color32::BLACK),
        ..Frame::default()
    };
    let selected_entity = match resource.selected {
        Some(e) => query.get_mut(e).ok(),
        None => None
    };
    match selected_entity {
        Some((t, tr, mut object)) => {
            let mut ui_window_pos = camera.world_to_viewport(camera_transform, t.translation()).unwrap().to_array();
            ui_window_pos[0] += (1./projection.scale) * tr.scale.x/2. + 10.; // move window to the right of the object
            ui_window_pos[1] -= window.height();

            egui::Window::new(format!("{:?}", resource.selected))
                .open(&mut resource.open)
                .resizable(false)
                .frame(window_frame)
                .anchor(Align2::LEFT_BOTTOM, ui_window_pos)
                .show(contexts.ctx_mut(), |ui| {
                    ui.label(format!("Velocity: {}, {}", object.velocity.x, object.velocity.y));
                    ui.add(egui::DragValue::new(&mut object.velocity.x));
                    ui.add(egui::DragValue::new(&mut object.velocity.y));
                });
        },
        None => ()
    }
}





pub fn sidebar(
    mut contexts: EguiContexts,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    egui::SidePanel::new(egui::panel::Side::Right, "sidebar")
        .min_width(150.)
        .resizable(false)
        .show(contexts.ctx_mut(), |ui| {
            ui.label("sidepanela");
            if ui.button("Spawn").clicked() {
                spawn_object(commands, meshes, materials);
            }
        });
}