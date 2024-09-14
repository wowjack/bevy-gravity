use bevy::{math::DVec2, prelude::*};
use bevy_egui::{egui::{panel, DragValue, RichText, SidePanel, Slider, Button}, EguiContexts};
use rand::Rng;
use crate::{gravity_system_tree::system_manager::{self, GravitySystemManager}, path_calculator::PathCalculator, visual_object::{CircleMesh, DrawOptions, FollowObjectResource, SelectedObjects, SimulationState, VisualObjectBundle, VisualObjectData}};



pub const SIDE_PANEL_WIDTH: f32 = 350.;


pub struct ObjectSpawnOptions {
    position: DVec2,
    velocity: DVec2,
    mass: f64,
    radius: f64,
    rgb: [f32; 3]
}
impl Default for ObjectSpawnOptions {
    fn default() -> Self {
        Self { position: Default::default(), velocity: Default::default(), mass: 1., radius: 1., rgb: [1., 1., 1.] }
    }
}

pub fn side_panel(
    mut contexts: EguiContexts,
    mut sim_state: ResMut<SimulationState>,
    mut draw_options: ResMut<DrawOptions>,
    mut spawn_options: Local<ObjectSpawnOptions>,
    selected_objects: Res<SelectedObjects>,
    mut follow_object_resource: ResMut<FollowObjectResource>,
    system_manager: Res<GravitySystemManager>,
    mut commands: Commands,
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

            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.checkbox(&mut sim_state.running, "Run:");
                    ui.add(bevy_egui::egui::Slider::new(&mut sim_state.run_speed, 0.01..=50_000.0).logarithmic(true))
                });
                ui.label(format!("tick: {}", unsafe { sim_state.current_time.to_int_unchecked::<u64>() }))
            });
            

            ui.collapsing("Draw Options", |ui| {
                ui.checkbox(&mut draw_options.draw_velocity_arrow, "Show Velocity");
                ui.checkbox(&mut draw_options.draw_future_path, "Show Path");
                ui.checkbox(&mut follow_object_resource.follow_object, "Follow Focused Object");
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
                        Slider::new(&mut spawn_options.radius, 1.0f64..=10_000.)
                            .logarithmic(true)
                    );
                });
                ui.horizontal(|ui| {
                    ui.label("Color");
                    bevy_egui::egui::color_picker::color_edit_button_rgb(ui, &mut spawn_options.rgb);
                });
                if ui.button("Spawn").clicked() {
                    //let object_data = VisualObjectData::new(
                    //    spawn_options.position,
                    //    spawn_options.velocity,
                    //    spawn_options.mass,
                    //    spawn_options.radius,
                    //    Color::linear_rgb(spawn_options.rgb[0], spawn_options.rgb[1], spawn_options.rgb[2]),
                    //);
                    //let bundle = VisualObjectBundle::new(object_data.clone());
                    //let entity = commands.spawn(bundle).id();
                    //let event = ChangeEvent { entity, change: crate::physics::Change::CreateObject(MassiveObject::from(object_data))};
                    //change_event_writer.send(event);
                }
            });

            ui.separator();

            ui.collapsing("Focused Object Editor", |ui| {
                let Some((e, mut data)) = selected_objects.focused.clone() else { 
                    ui.label("No Object Focused");
                    return;
                };
                ui.label(format!("{}", data.name));
                ui.horizontal(|ui| {
                    ui.label("Position");
                    let x_pos_changed = ui.add(DragValue::new(&mut data.position.x).prefix("X: ")).changed();
                    let y_pos_changed = ui.add(DragValue::new(&mut data.position.y).prefix("Y: ")).changed();
                    if x_pos_changed || y_pos_changed {
                        //let event = ChangeEvent::new(e, Change::SetPosition(data.position));
                        //change_event_writer.send(event);
                    }
                });
                ui.horizontal(|ui| {
                    ui.label("Velocity");
                    let x_vel_changed = ui.add(DragValue::new(&mut data.velocity.x).prefix("X: ")).changed();
                    let y_vel_changed = ui.add(DragValue::new(&mut data.velocity.y).prefix("Y: ")).changed();
                    if x_vel_changed || y_vel_changed {
                        //let event = ChangeEvent::new(e, Change::SetVelocity(data.velocity));
                        //change_event_writer.send(event);
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
                });
                ui.horizontal(|ui| {
                    ui.label("Radius");
                    ui.style_mut().spacing.slider_width = 225.;
                    let radius_slider = ui.add(
                        Slider::new(&mut data.radius, 1.0..=10_000.)
                            .logarithmic(true)
                    );
                });

                let mut rgb: [f32; 3] = data.color.to_linear().to_f32_array_no_alpha();
                ui.horizontal(|ui| {
                    ui.label("Color");
                    let color_changed = bevy_egui::egui::color_picker::color_edit_button_rgb(ui, &mut rgb).changed();
                });

                if ui.button("add path calculator").clicked() {
                    if let Some(mut ec) = commands.get_entity(e) {
                        let path_calc = PathCalculator::new(&system_manager, e);
                        ec.insert(path_calc);
                    }
                }
            });
        });
}



