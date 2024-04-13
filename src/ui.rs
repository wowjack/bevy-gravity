use bevy::prelude::*;
use bevy_egui::{egui::{panel, DragValue, Rect, RichText, SidePanel, Slider}, EguiContexts};
use bevy_math::DVec2;

use crate::{physics::{ChangeEvent, MassiveObject}, pseudo_camera::CameraState, visual_object::{DrawOptions, SimulationState, VisualObjectBundle}, CircleAssets};



pub const SIDE_PANEL_WIDTH: f32 = 350.;


#[derive(Default)]
pub struct ObjectSpawnOptions {
    position: DVec2,
    velocity: DVec2,
    mass: f64,
    radius: f32,
    rgb: [f32; 3]
}

pub fn side_panel(
    mut contexts: EguiContexts,
    mut change_event_writer: EventWriter<ChangeEvent>,
    mut commands: Commands,
    circle_assets: Res<CircleAssets>,
    mut sim_state: ResMut<SimulationState>,
    window_query: Query<&Window>,
    camera_query: Query<(&CameraState, &Camera, &GlobalTransform)>,
    mut draw_options: ResMut<DrawOptions>,
    mut spawn_options: Local<ObjectSpawnOptions>,
) {
    let window = window_query.single();
    let (camera_state, camera, camera_gtrans) = camera_query.single();

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
                    let o = MassiveObject { position: spawn_options.position, velocity: spawn_options.velocity, mass: spawn_options.mass };
                    let entity = commands.spawn(VisualObjectBundle::new(o.clone(), spawn_options.radius, circle_assets.as_ref())).id();
                    let event = ChangeEvent { entity, change: crate::physics::Change::CreateObject(o)};
                    change_event_writer.send(event);
                }
            });

            //check if pointer is within the ui
            //println!("{}", ui.rect_contains_pointer(Rect::everything_right_of(window.width() - SIDE_PANEL_WIDTH)));
        });
}