use bevy::prelude::*;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy_egui::EguiPlugin;
use bevy_mod_picking::prelude::*;
use bevy_prototype_lyon::prelude::ShapePlugin;
use object::MassiveObjectPlugin;
use zoom::mouse_zoom;

mod zoom;
mod object;
mod ui;


fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins/*.disable::<LogPlugin>()*/,
            EguiPlugin,
            DefaultPickingPlugins.build().disable::<DebugPickingPlugin>(),
            ShapePlugin,
            FrameTimeDiagnosticsPlugin,
            //LogDiagnosticsPlugin::default(),
            MassiveObjectPlugin
        ))
        .insert_resource(ClearColor(Color::rgb(0.7, 0.7, 0.7)))
        .add_systems(Startup, init)
        .add_systems(Update, (
            mouse_zoom,
            ui::ui
        ))
        .run()
}


#[derive(Component)]
pub struct MainCamera;

fn init(
    mut commands: Commands,
) {
    commands.spawn((
        Camera2dBundle::default(),
        MainCamera
    ));
}














