use bevy::prelude::*;
use bevy_egui::{egui::{panel, DragValue, RichText, SidePanel, Slider}, EguiContexts};
use bevy_math::DVec2;
use crate::{physics::{Change, ChangeEvent, MassiveObject}, visual_object::{CircleMesh, DrawOptions, SelectedObjects, SimulationState, VisualChange, VisualChangeEvent, VisualObjectBundle, VisualObjectData}};



pub const SIDE_PANEL_WIDTH: f32 = 350.;


pub struct ObjectSpawnOptions {
    position: DVec2,
    velocity: DVec2,
    mass: f64,
    radius: f32,
    rgb: [f32; 3]
}
impl Default for ObjectSpawnOptions {
    fn default() -> Self {
        Self { position: Default::default(), velocity: Default::default(), mass: 1., radius: 1., rgb: Color::DARK_GREEN.rgb_linear_to_vec3().into() }
    }
}

pub fn side_panel(
    mut contexts: EguiContexts,
    mut change_event_writer: EventWriter<ChangeEvent>,
    mut commands: Commands,
    mut sim_state: ResMut<SimulationState>,
    mut draw_options: ResMut<DrawOptions>,
    mut spawn_options: Local<ObjectSpawnOptions>,
    circle_mesh: Res<CircleMesh>,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
    selected_objects: Res<SelectedObjects>,
    mut visual_change_event_writer: EventWriter<VisualChangeEvent>,
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

            ui.horizontal(|ui| {
                ui.checkbox(&mut sim_state.running, "Run:");
                ui.add(bevy_egui::egui::Slider::new(&mut sim_state.run_speed, 1..=50_000).logarithmic(true))
            });

            ui.collapsing("Draw Options", |ui| {
                ui.checkbox(&mut draw_options.draw_velocity_arrow, "Show Velocity");
                ui.checkbox(&mut draw_options.draw_future_path, "Show Path")
            });

            ui.separator();

            ui.collapsing("Spawn Object", |ui| {
                ui.horizontal(|ui| {
                    ui.label("Position");
                    ui.add(DragValue::new(&mut spawn_options.position.x).prefix("X: "));
                    ui.add(DragValue::new(&mut spawn_options.position.y).prefix("Y: "));
                });
                ui.horizontal(|ui| {
                    ui.label("Velocity");
                    ui.add(DragValue::new(&mut spawn_options.velocity.x).prefix("X: "));
                    ui.add(DragValue::new(&mut spawn_options.velocity.y).prefix("Y: "));
                });
                ui.horizontal(|ui| {
                    ui.label("Mass");
                    ui.style_mut().spacing.slider_width = 225.;
                    ui.add(
                        Slider::new(&mut spawn_options.mass, 1.0..=1e30)
                            .logarithmic(true)
                            .custom_formatter(|num, _| format!("{:1.1e}", num))
                    );
                });
                ui.horizontal(|ui| {
                    ui.label("Radius");
                    ui.style_mut().spacing.slider_width = 225.;
                    ui.add(
                        Slider::new(&mut spawn_options.radius, 1.0..=10_000.)
                            .logarithmic(true)
                    );
                });
                ui.horizontal(|ui| {
                    ui.label("Color");
                    bevy_egui::egui::color_picker::color_edit_button_rgb(ui, &mut spawn_options.rgb);
                });
                if ui.button("Spawn").clicked() {
                    let object_data = VisualObjectData::new(
                        spawn_options.position,
                        spawn_options.velocity,
                        spawn_options.mass,
                        spawn_options.radius,
                        Color::rgb_linear_from_array(spawn_options.rgb)
                    );
                    let bundle = VisualObjectBundle::new(object_data.clone(), circle_mesh.0.clone().into(), &mut color_materials);
                    let entity = commands.spawn(bundle).id();
                    let event = ChangeEvent { entity, change: crate::physics::Change::CreateObject(MassiveObject::from(object_data))};
                    change_event_writer.send(event);
                }
            });

            ui.separator();

            ui.collapsing("Focused Object Editor", |ui| {
                let Some((e, mut data)) = selected_objects.focused .clone() else { 
                    ui.label("No Object Focused");
                    return;
                };
                ui.label(format!("{:?}", e));
                ui.horizontal(|ui| {
                    ui.label("Position");
                    let x_pos_changed = ui.add(DragValue::new(&mut data.position.x).prefix("X: ")).changed();
                    let y_pos_changed = ui.add(DragValue::new(&mut data.position.y).prefix("Y: ")).changed();
                    if x_pos_changed || y_pos_changed {
                        let event = ChangeEvent::new(e, Change::SetPosition(data.position));
                        change_event_writer.send(event);
                    }
                });
                ui.horizontal(|ui| {
                    ui.label("Velocity");
                    let x_vel_changed = ui.add(DragValue::new(&mut data.velocity.x).prefix("X: ")).changed();
                    let y_vel_changed = ui.add(DragValue::new(&mut data.velocity.y).prefix("Y: ")).changed();
                    if x_vel_changed || y_vel_changed {
                        let event = ChangeEvent::new(e, Change::SetVelocity(data.velocity));
                        change_event_writer.send(event);
                    }
                });
                ui.horizontal(|ui| {
                    ui.label("Mass");
                    ui.style_mut().spacing.slider_width = 225.;
                    let mass_slider = ui.add(
                        Slider::new(&mut data.mass, 1.0..=1e30)
                            .logarithmic(true)
                            .custom_formatter(|num, _| format!("{:1.1e}", num))
                    );
                    if mass_slider.changed() {
                        let event = ChangeEvent::new(e, Change::SetMass(data.mass));
                        change_event_writer.send(event);
                    }
                });
                ui.horizontal(|ui| {
                    ui.label("Radius");
                    ui.style_mut().spacing.slider_width = 225.;
                    let radius_slider = ui.add(
                        Slider::new(&mut data.radius, 1.0..=10_000.)
                            .logarithmic(true)
                    );
                    if radius_slider.changed() {
                        visual_change_event_writer.send(VisualChangeEvent { target: e, change: VisualChange::SetRadius(data.radius) });
                    }
                });

                let mut rgb: [f32; 3] = data.color.rgb_linear_to_vec3().into();
                ui.horizontal(|ui| {
                    ui.label("Color");
                    let color_changed = bevy_egui::egui::color_picker::color_edit_button_rgb(ui, &mut rgb).changed();
                    if color_changed {
                        visual_change_event_writer.send(VisualChangeEvent { target: e, change: VisualChange::SetColor(Color::rgb_linear_from_array(rgb)) });
                    }
                });
            });

            //check if pointer is within the ui
            //println!("{}", ui.rect_contains_pointer(Rect::everything_right_of(window.width() - SIDE_PANEL_WIDTH)));
        });
}