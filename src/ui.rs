use bevy::prelude::*;
use bevy_egui::EguiContexts;
use egui::{DragValue, Frame, Rounding, Color32, Stroke, epaint::Shadow, Margin};


pub fn ui(mut contexts: EguiContexts, mut val: Local<f64>, mut open: Local<bool>) {
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
            if ui.add(DragValue::new(&mut (*val))).changed() {
                println!("Changed");
            }
        });
}