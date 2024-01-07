use bevy::prelude::*;
use bevy_egui::EguiContexts;
use egui::{Frame, Rounding, Color32, Stroke, epaint::Shadow, Margin};

use crate::object::spawn::SpawnObjectEvent;


pub fn ui(
    mut contexts: EguiContexts,
    mut spawn_event_writer: EventWriter<SpawnObjectEvent>,
    mut open: Local<bool>
) {
    let window_frame = Frame {
        inner_margin: Margin::same(5.),
        rounding: Rounding::same(4.),
        shadow: Shadow::NONE,
        fill: Color32::DARK_GRAY,
        stroke: Stroke::new(1., Color32::BLACK),
        ..Frame::default()
    };

    *open = true;
    egui::Window::new("Window")
        .open(&mut (*open))
        .frame(window_frame)
        .show(contexts.ctx_mut(), |ui| {
            if ui.button("spawn").clicked() {
                spawn_event_writer.send(SpawnObjectEvent {velocity: Vec2::new(0.005, 0.005), ..default()});
            }
        });
}