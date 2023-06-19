use bevy_egui::{egui::{self, Frame, epaint::Shadow, Color32, Stroke, Margin, Align2, Rounding}, EguiContexts};
use bevy::prelude::*;

use crate::object::MassiveObject;


#[derive(bevy::prelude::Resource, Default)]
pub struct ObjectDetailUIContext {
    pub open: bool,
    pub selected: Option<bevy::prelude::Entity>,
}
pub fn ui_example_system(
    mut contexts: EguiContexts,
    mut resource: bevy::prelude::ResMut<ObjectDetailUIContext>,
    query: Query<(&GlobalTransform, &Transform, &MassiveObject)>,
    camera_query: Query<(&GlobalTransform, &Camera)>,
) {
    let (camera_transform, camera) = camera_query.get_single().unwrap();
    let window_frame = Frame {
        inner_margin: Margin::same(5.),
        rounding: Rounding::same(4.),
        shadow: Shadow::NONE,
        fill: Color32::DARK_GRAY,
        stroke: Stroke::new(1., Color32::BLACK),
        ..Frame::default()
    };
    let selected_entity = match resource.selected {
        Some(e) => query.get(e).ok(),
        None => None
    };
    match selected_entity {
        Some((t, tr, object)) => {
            let ui_window_pos = camera.world_to_viewport(camera_transform, t.translation()).unwrap().to_array();
            egui::Window::new(format!("{:?}", resource.selected))
                .open(&mut resource.open)
                .resizable(false)
                .frame(window_frame)
                .anchor(Align2::LEFT_BOTTOM, [ui_window_pos[0]+tr.scale.x/2. + 10.0, ui_window_pos[1]*-1.])
                .show(contexts.ctx_mut(), |ui| {
                    ui.label(format!("Velocity: {}, {}", object.velocity.x, object.velocity.y));
                });
        },
        None => ()
    }
}