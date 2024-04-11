use bevy::prelude::*;
use bevy_egui::{egui::{panel, RichText, SidePanel}, EguiContexts};
use bevy_math::DVec2;

use crate::{physics::{ChangeEvent, MassiveObject}, visual_object::VisualObjectBundle, CircleAssets};



pub const SIDE_PANEL_WIDTH: f32 = 350.;


#[derive(Resource, Default)]
pub struct ObjectSpawnOptions(MassiveObject);

pub fn side_panel(
    mut contexts: EguiContexts,
    mut change_event_writer: EventWriter<ChangeEvent>,
    mut commands: Commands,
    circle_assets: Res<CircleAssets>,
) {
    SidePanel::new(panel::Side::Right, "sidepanel")
        .exact_width(SIDE_PANEL_WIDTH)
        .resizable(false)
        .show(contexts.ctx_mut(), |ui| {
            ui.vertical_centered_justified(|ui| {
                ui.add_space(20.);
                ui.heading(RichText::new("Bevy Gravity").strong().size(30.));
                ui.add_space(20.);
            });

            if ui.button("spawn").clicked() {
                let entity = commands.spawn(VisualObjectBundle::default(circle_assets.as_ref())).id();
                change_event_writer.send(ChangeEvent { entity, change: crate::physics::Change::CreateObject(
                    MassiveObject { position: DVec2::ZERO, velocity: DVec2::ZERO, mass: 1. }
                ) });
            }
        });
}