use std::collections::VecDeque;

use bevy::prelude::*;
use bevy_egui::EguiContexts;
use egui::{DragValue, Slider, Layout};

use crate::object::{spawn::SpawnObjectEvent, physics_future::{PhysicsFuture, UpdatePhysics}, select::SelectedObjects};

pub const SIDE_PANEL_WIDTH: f32 = 250.;
pub const BOTTOM_PANEL_HEIGHT: f32 = 100.;


pub fn bottom_panel(
    mut contexts: EguiContexts,
    physics_future: Res<PhysicsFuture>,
    selected_objects: Res<SelectedObjects>
) {
    egui::TopBottomPanel::new(egui::panel::TopBottomSide::Bottom, "bottom_panel")
        .exact_height(BOTTOM_PANEL_HEIGHT)
        .resizable(false)
        .show(contexts.ctx_mut(), |ui| {
            ui.horizontal(|ui| {
                ui.allocate_ui_with_layout([100., 100.].into(), Layout::default(), |ui| {
                    ui.vertical_centered(|ui| {
                        ui.add_space(5.);
                        ui.strong("Buffer Size");
                        ui.add_space(5.);
                        ui.strong(format!("{:?}", physics_future.future.lock().unwrap().values().next().unwrap_or(&VecDeque::new()).len()));
                    });
                });
                ui.allocate_ui((100., 100.).into(), |ui| {
                    ui.vertical_centered(|ui| {
                        ui.add_space(5.);
                        egui::ScrollArea::new([false, true]).max_height(100.).show(ui, |ui| {
                            for e in &selected_objects.selected {
                                ui.heading(format!("{:?}", e));
                            }
                        });
                    });
                });
            });
        });
}



pub fn side_panel(
    mut contexts: EguiContexts,
    mut spawn_options: Local<SpawnObjectEvent>,
    mut spawn_event_writer: EventWriter<SpawnObjectEvent>,
    mut update_physics: ResMut<UpdatePhysics>
) {
    egui::SidePanel::new(egui::panel::Side::Right, "sidepanel")
        .exact_width(SIDE_PANEL_WIDTH)
        .resizable(false)
        .show_animated(contexts.ctx_mut(), true, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(10.);
                ui.style_mut().spacing.icon_width = 40.;
                ui.style_mut().spacing.icon_width_inner = 20.;
                ui.add(egui::Checkbox::new(&mut update_physics.0, "")).on_hover_text("Run");
            });
            ui.vertical_centered(|ui| {
                ui.add_space(10.);
                ui.heading("Spawn Object");
                ui.columns(3, |ui| {
                    ui[0].label("Position");
                    ui[1].add(DragValue::new(&mut spawn_options.position.x));
                    ui[2].add(DragValue::new(&mut spawn_options.position.y));
                });
                ui.columns(3, |ui| {
                    ui[0].label("Velocity");
                    ui[1].add(DragValue::new(&mut spawn_options.velocity.x));
                    ui[2].add(DragValue::new(&mut spawn_options.velocity.y));
                });
                ui.horizontal(|ui| {
                    ui.label("Mass");
                    ui.add(Slider::new(&mut spawn_options.mass, (0.0001)..=(1_000_000_000_000_000_000.)).logarithmic(true));
                });
                ui.horizontal(|ui| {
                    ui.label("Radius");
                    ui.add(Slider::new(&mut spawn_options.radius, (1.)..=(10_000.)).logarithmic(true));
                });
                if ui.button("Spawn").clicked() {
                    spawn_event_writer.send(*spawn_options);
                }
            });
        });
}