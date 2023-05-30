use bevy::{prelude::*, core_pipeline::clear_color::ClearColorConfig};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(init)
        .run()
}

fn init(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        camera_2d: Camera2d {clear_color: ClearColorConfig::Custom(Color::BEIGE)},
        ..Camera2dBundle::default()
    });
}


