use bevy::{prelude::*, core_pipeline::clear_color::ClearColorConfig, log::LogPlugin, diagnostic::DiagnosticsPlugin};
use bevy_egui::EguiPlugin;
use bevy_mod_picking::prelude::*;
use bevy_prototype_lyon::prelude::ShapePlugin;
use object::MassiveObject;
use ui::ObjectDetailUIContext;

mod ui;
mod object;
#[derive(Resource)]
pub struct ArrowHandle(Option<Handle<Image>>);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)//.disable::<LogPlugin>())
        .add_plugin(EguiPlugin)
        .add_plugins(DefaultPickingPlugins.build().disable::<DebugPickingPlugin>())
        .add_plugin(ShapePlugin)
        .insert_resource(ui::ObjectDetailUIContext::default())
        .insert_resource(ArrowHandle(None))
        .add_event::<object::ObjectDragEvent>()
        .add_startup_system(init)
        .add_startup_system(object::spawn_object)
        .add_system(ui::ui_example_system)
        .add_system(object::handle_object_drag)
        .add_system(object::object_selected)
        //.add_system(move_body)
        .run()
}

fn init(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut arrow_handle: ResMut<ArrowHandle>
) {
    commands.spawn((
        Camera2dBundle {
            camera_2d: Camera2d {clear_color: ClearColorConfig::Custom(Color::BEIGE)},
            ..Camera2dBundle::default()
        }, 
        RaycastPickCamera::default(),
    ));
    
    let arrow: Handle<Image> = asset_server.load("arrow.png");
    *arrow_handle = ArrowHandle(Some(arrow));
}









