use bevy::{prelude::*, sprite::{Mesh2dHandle, MaterialMesh2dBundle}};
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy_egui::EguiPlugin;
use bevy_mod_picking::prelude::*;
use bevy_prototype_lyon::prelude::ShapePlugin;
use object::{MassiveObjectPlugin, spawn::SpawnObjectEvent};
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
        .add_systems(PostStartup, post_start)
        .add_systems(Update, (
            update,
            mouse_zoom,
            ui::ui
        ))
        .run()
}


fn update() {

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


fn post_start(mut event_writer: EventWriter<SpawnObjectEvent>) {
    event_writer.send_batch((0..100).map(|i| SpawnObjectEvent { position: Vec2::new(i as f32*10., 0.), mass: 100_000_000_000., ..default()}));
    //event_writer.send(SpawnObjectEvent { velocity: Vec2::new(0.05, 0.05), ..default()});
}














