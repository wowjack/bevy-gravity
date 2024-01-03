use bevy_egui::{egui::{self, Frame, epaint::Shadow, Color32, Stroke, Margin, Align2, Rounding}, EguiContexts};
use bevy::prelude::*;

use crate::{object::{MassiveObject, spawn_object, SpawnObjectEvent}, MainCamera, GameState};


#[derive(bevy::prelude::Resource, Default)]
pub struct ObjectDetailUIContext {
    pub open: bool,
    pub selected: Option<bevy::prelude::Entity>,
}
pub fn ui_example_system(
    mut contexts: EguiContexts,
    mut resource: bevy::prelude::ResMut<ObjectDetailUIContext>,
    mut query: Query<(&GlobalTransform, &Transform, &mut MassiveObject)>,
    camera_query: Query<(&GlobalTransform, &Camera, &OrthographicProjection), With<MainCamera>>,
    window_query: Query<&Window>,
) {
    let window = window_query.get_single().unwrap();
    let (camera_transform, camera, projection) = camera_query.get_single().unwrap();
    let window_frame = Frame {
        inner_margin: Margin::same(5.),
        rounding: Rounding::same(4.),
        shadow: Shadow::NONE,
        fill: Color32::DARK_GRAY,
        stroke: Stroke::new(1., Color32::BLACK),
        ..Frame::default()
    };
    let selected_entity = match resource.selected {
        Some(e) => query.get_mut(e).ok(),
        None => None
    };
    match selected_entity {
        Some((t, tr, mut object)) => {
            let Some(mut ui_window_pos) = camera.world_to_viewport(camera_transform, t.translation()) else { return; };
            ui_window_pos.x += (1./projection.scale) * tr.scale.x/2. + 10.; // move window to the right of the object
            ui_window_pos.y -= window.height();

            egui::Window::new(format!("{:?}", resource.selected))
                .open(&mut resource.open)
                .resizable(false)
                .frame(window_frame)
                .anchor(Align2::LEFT_BOTTOM, ui_window_pos.to_array())
                .fixed_size((200., 400.))
                .show(contexts.ctx_mut(), |ui| {
                    ui.horizontal(|ui| {
                        ui.add(egui::DragValue::new(&mut object.velocity.x).max_decimals(8).prefix("x:").speed(0.01));
                        ui.add(egui::DragValue::new(&mut object.velocity.y).max_decimals(8).prefix("y:").speed(0.01));
                    });
                    ui.add(egui::DragValue::new(&mut object.mass).prefix("Mass: ").speed(100000.));
                });
        },
        None => ()
    }
}





pub fn sidebar(
    mut contexts: EguiContexts,
    mut gamestate: ResMut<GameState>,
    mut spawn_event_writer: EventWriter<SpawnObjectEvent>
) {
    egui::SidePanel::new(egui::panel::Side::Right, "sidebar")
        .min_width(150.)
        .resizable(false)
        .show(contexts.ctx_mut(), |ui| {
            ui.label("");
            if ui.button("Spawn").clicked() {
                spawn_event_writer.send(SpawnObjectEvent);
            }
            ui.checkbox(&mut gamestate.play, "Run: ");

        });
}