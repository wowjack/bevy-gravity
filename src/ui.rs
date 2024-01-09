use std::collections::VecDeque;

use bevy::prelude::*;
use bevy_egui::EguiContexts;
use egui::{DragValue, Slider, Layout};

use crate::object::{spawn::{SpawnObjectEvent, VisualObject}, physics_future::{PhysicsFuture, UpdatePhysics}, select::SelectedObjects, object::{MassiveObject, EditObjectData, EditObjectEvent}};

pub const SIDE_PANEL_WIDTH: f32 = 300.;
pub const BOTTOM_PANEL_HEIGHT: f32 = 150.;

#[derive(Resource, Default)]
pub struct ToDraw {
    pub velocity_arrow: bool,
    pub future_path: bool,
    pub prediction_buffer_len: usize,
    pub prediction_line_segment_size: f32,
    pub outline: bool
}

pub fn bottom_panel(
    mut contexts: EguiContexts,
    physics_future: Res<PhysicsFuture>,
    mut selected_objects: ResMut<SelectedObjects>,
    object_query: Query<(&Children, &MassiveObject, &Transform), Without<VisualObject>>,
    visual_query: Query<&Transform, (With<VisualObject>, Without<MassiveObject>)>,
    mut edit_object: Local<EditObjectData>,
    mut edit_object_event_writer: EventWriter<EditObjectEvent>,
    mut to_draw: ResMut<ToDraw>,
) {
    egui::TopBottomPanel::new(egui::panel::TopBottomSide::Bottom, "bottom_panel")
        .exact_height(BOTTOM_PANEL_HEIGHT)
        .resizable(false)
        .show(contexts.ctx_mut(), |ui| {
            ui.allocate_ui_with_layout((500., BOTTOM_PANEL_HEIGHT).into(), Layout::left_to_right(egui::Align::Center), |ui| {
                ui.vertical(|ui| {
                    ui.add_space(5.);
                    ui.strong("Buffer Size");
                    ui.add_space(5.);
                    ui.strong(format!("{:?}", physics_future.future.lock().unwrap().values().next().unwrap_or(&VecDeque::new()).len()));
                });
                ui.vertical(|ui| {
                    ui.add_space(5.);
                    egui::ScrollArea::new([false, true]).show(ui, |ui| {
                        for e in selected_objects.selected.clone() {
                            ui.style_mut().spacing.button_padding = (20., 5.).into();
                            if ui.button(format!("{:?}", e)).clicked() {
                                selected_objects.focused = Some(e);
                            }
                        }
                    });
                });
                ui.vertical(|ui| {
                    let Some(focused) = selected_objects.focused else { return };
                    
                    ui.heading(format!("{:?}", focused));
                    let Ok((children, object, trans)) = object_query.get(focused) else { return };
                    let Ok(visual_trans) = visual_query.get(*children.iter().filter(|e| visual_query.contains(**e)).next().unwrap()) else { return };
                    edit_object.mass = object.mass as f32;
                    edit_object.position = trans.translation.truncate();
                    edit_object.velocity = object.velocity;
                    edit_object.radius = visual_trans.scale.x;
                    
                    let mut changed = false;
                    ui.horizontal(|ui| {
                        ui.strong("Position");
                        changed = ui.add(DragValue::new(&mut edit_object.position.x).prefix("X:")).changed() || changed;
                        changed = ui.add(DragValue::new(&mut edit_object.position.y).prefix("Y:")).changed() || changed;
                    });
                    ui.horizontal(|ui| {
                        ui.strong("Velocity");
                        changed = ui.add(DragValue::new(&mut edit_object.velocity.x).speed(0.1).prefix("X:")).changed() || changed;
                        changed = ui.add(DragValue::new(&mut edit_object.velocity.y).speed(0.1).prefix("Y:")).changed() || changed;
                    });
                    ui.horizontal(|ui| {
                        ui.strong("Mass");
                        changed = ui.add(Slider::new(&mut edit_object.mass, (1.)..=(1_000_000_000_000_000_000_000_000.)).logarithmic(true)).changed() || changed;
                    });
                    ui.horizontal(|ui| {
                        ui.strong("Radius");
                        changed = ui.add(Slider::new(&mut edit_object.radius, (1.)..=(1_000_000.)).logarithmic(true)).changed() || changed;
                    });

                    if changed {
                        edit_object_event_writer.send(EditObjectEvent { entity: focused, data: edit_object.clone() })
                    } 
                });
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        ui.checkbox(&mut to_draw.future_path, "Future Path");
                        ui.add(Slider::new(&mut to_draw.prediction_buffer_len, 1..=1_000_000).prefix("Length: ").logarithmic(true));
                    });
                    ui.horizontal(|ui| {
                        ui.add(Slider::new(&mut to_draw.prediction_line_segment_size, (1.)..=(100000.)).prefix("Segment Length: ").logarithmic(true))
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
                ui.add(egui::Checkbox::new(&mut update_physics.update, "")).on_hover_text("Run");
                ui.add(Slider::new(&mut update_physics.step, 1..=100_000).logarithmic(true).prefix("Speed: "));
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
                    ui.add(Slider::new(&mut spawn_options.mass, (1.)..=(1_000_000_000_000_000_000_000_000.)).logarithmic(true));
                });
                ui.horizontal(|ui| {
                    ui.label("Radius");
                    ui.add(Slider::new(&mut spawn_options.radius, (1.)..=(1_000_000.)).logarithmic(true));
                });
                if ui.button("Spawn").clicked() {
                    spawn_event_writer.send(*spawn_options);
                }
            });
        });
}