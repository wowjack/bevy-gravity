use bevy::prelude::*;
use bevy_egui::{egui::{panel, RichText, SidePanel}, EguiContexts};



pub const SIDE_PANEL_WIDTH: f32 = 350.;



pub fn side_panel(
    mut contexts: EguiContexts,
) {
    SidePanel::new(panel::Side::Right, "sidepanel")
        .exact_width(SIDE_PANEL_WIDTH)
        .resizable(false)
        .show_animated(contexts.ctx_mut(), true, |ui| {
            ui.vertical_centered_justified(|ui| {
                ui.add_space(20.);
                ui.heading(RichText::new("Bevy Gravity").strong().size(30.));
                ui.add_space(20.);
            });
        });
}