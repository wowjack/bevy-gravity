use bevy_egui::{egui::{self, Frame, epaint::Shadow, Color32, Stroke, Margin, Rounding, Rect, Pos2, Vec2}, EguiContexts};
use bevy::{prelude::*, render::view::window};

use crate::{object::{MassiveObject, SpawnObjectEvent, ObjectSpawnOptions}, MainCamera, GameState};


#[derive(bevy::prelude::Resource, Default, Clone)]
pub struct ObjectDetailContext {
    pub open: bool,
    pub selected: Vec<Entity>,
}
#[derive(Resource, Default)]
pub struct ObjectDetailState {
    pub follow_selected: bool
}

#[derive(Event)]
pub struct WindowSizeEvent(bevy_egui::egui::Rect);

pub fn object_detail_ui(
    mut contexts: EguiContexts,
    mut detail_context: ResMut<ObjectDetailContext>,
    mut detail_state: ResMut<ObjectDetailState>,
    mut query: Query<(&GlobalTransform, &mut Transform, &mut MassiveObject)>,
    mut window_size_event_writer: EventWriter<WindowSizeEvent>,
) {
    if detail_context.selected.is_empty() { return }

    let window_frame = Frame {
        inner_margin: Margin::same(5.),
        rounding: Rounding::same(4.),
        shadow: Shadow::NONE,
        fill: Color32::DARK_GRAY,
        stroke: Stroke::new(1., Color32::BLACK),
        ..Frame::default()
    };
    for entity in detail_context.selected.clone() {
        let Ok((_gt, mut tr, mut object)) = query.get_mut(entity) else { return };
        egui::Window::new(format!("{:?}", detail_context.selected))
            .open(&mut detail_context.open)
            .frame(window_frame)
            .default_size(Vec2::new(1., 1.))
            .resizable(false)
            .title_bar(false)
            .show(contexts.ctx_mut(), |ui| {
                window_size_event_writer.send(WindowSizeEvent(ui.clip_rect()));
                
                ui.horizontal(|ui| {
                    ui.columns(2, |ui| {
                        ui[0].centered_and_justified(|ui| {ui.add(egui::DragValue::new(&mut tr.translation.x).max_decimals(8).prefix("x: ").speed(0.01));});
                        ui[1].centered_and_justified(|ui| {ui.add(egui::DragValue::new(&mut tr.translation.y).max_decimals(8).prefix("y: ").speed(0.01));})
                    });
                });
                ui.horizontal(|ui| {
                    ui.columns(2, |ui| {
                        ui[0].centered_and_justified(|ui| {ui.add(egui::DragValue::new(&mut object.velocity.x).max_decimals(8).prefix("x: ").speed(0.01));});
                        ui[1].centered_and_justified(|ui| {ui.add(egui::DragValue::new(&mut object.velocity.y).max_decimals(8).prefix("y: ").speed(0.01));});
                    });
                });
                ui.add(egui::DragValue::new(&mut object.mass).prefix("Mass: ").speed(10000000.).prefix("Mass: "));
                ui.add(egui::Slider::new(&mut object.radius, (1.)..=(1_000.)).logarithmic(true).prefix("Radius: "));
                ui.checkbox(&mut detail_state.follow_selected, "Follow Object");
            });
    }        
}

#[derive(Component)]
pub struct WindowBlockingRectangle;
#[derive(Component)]
pub struct SidebarBlockingRectangle;

pub fn track_window(mut events: EventReader<WindowSizeEvent>, mut rect_query: Query<&mut Transform, With<WindowBlockingRectangle>>, camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>) {
    if events.is_empty() { return }

    let mut trans = rect_query.single_mut();
    let (camera, gtrans) = camera_query.single();

    for event in events.read() {
        let rect_min = bevy::prelude::Vec2::new(event.0.min.x, event.0.min.y) - 7.;
        let rect_max = bevy::prelude::Vec2::new(event.0.max.x, event.0.max.y) + 7.;
        let min = camera.viewport_to_world_2d(gtrans, rect_min).unwrap();
        let max = camera.viewport_to_world_2d(gtrans, rect_max).unwrap();
        let x_size = (max.x - min.x).abs();
        let y_size = (min.y - max.y).abs();
        let mid = (min + max) / 2.;
        trans.translation = mid.extend(2.);
        trans.scale = Vec3::new(x_size, y_size, 1.);
    }
}



pub fn sidebar(
    mut contexts: EguiContexts,
    mut gamestate: ResMut<GameState>,
    mut spawn_options: ResMut<ObjectSpawnOptions>,
    mut spawn_event_writer: EventWriter<SpawnObjectEvent>,
) {
    egui::SidePanel::new(egui::panel::Side::Right, "sidebar")
        .exact_width(200.)
        .resizable(false)
        .show(contexts.ctx_mut(), |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("Spawn");
                ui.columns(3, |ui| {
                    ui[0].label("Position: ");
                    ui[1].vertical_centered(|ui| {ui.add(egui::DragValue::new(&mut spawn_options.position.x).speed(1.).prefix("x: "));});
                    ui[2].vertical_centered(|ui| {ui.add(egui::DragValue::new(&mut spawn_options.position.y).speed(1.).prefix("y: "));});
                });
                ui.columns(3, |ui| {
                    ui[0].label("Velocity: ");
                    ui[1].vertical_centered(|ui| {ui.add(egui::DragValue::new(&mut spawn_options.velocity.x).speed(0.01).prefix("x: "));});
                    ui[2].vertical_centered(|ui| {ui.add(egui::DragValue::new(&mut spawn_options.velocity.y).speed(0.01).prefix("y: "));});
                });
                ui.columns(2, |ui| {
                    ui[0].label("Mass: ");
                    ui[0].add(egui::DragValue::new(&mut spawn_options.mass).speed(100_000.));
                    ui[1].label("Radius: ");
                    ui[1].add(egui::DragValue::new(&mut spawn_options.radius).speed(1.));
                });
                if ui.button("Spawn").clicked() {
                    spawn_event_writer.send(SpawnObjectEvent);
                }
            });
            
            ui.checkbox(&mut gamestate.play, "Run: ");

        });
}