use bevy::{prelude::*, input::mouse::{MouseWheel, self}, math::vec2};
use bevy_egui::EguiPlugin;
use bevy_mod_picking::prelude::*;
use bevy_prototype_lyon::prelude::ShapePlugin;
use object::{MassiveObject, spawn_object};
use ui::ObjectDetailUIContext;

mod ui;
mod object;

#[derive(Resource)]
pub struct ArrowHandle(Option<Handle<Image>>);

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins/*.disable::<LogPlugin>()*/,
            EguiPlugin,
            DefaultPickingPlugins.build().disable::<DebugPickingPlugin>(),
            ShapePlugin
        ))
        .insert_resource(ClearColor(Color::rgb(0.7, 0.7, 0.7)))
        .insert_resource(ui::ObjectDetailUIContext::default())
        .insert_resource(ArrowHandle(None))
        .add_event::<object::ObjectDragEvent>()
        .add_systems(Startup, (init, object::spawn_object))
        .add_systems(Update, (ui::ui_example_system, mouse_zoom))
        .run()
}


fn mouse_zoom(mut events: EventReader<MouseWheel>, mut projection_query: Query<(&mut Transform, &mut OrthographicProjection), With<MainCamera>>, window_query: Query<&Window>) {
    let delta_zoom: f32 = events.read().map(|e| e.y).sum();
    if delta_zoom == 0. {return;}

    let (mut camera_pos, mut projection) = projection_query.single_mut();
    

    let window = window_query.single();
    let window_size = Vec2::new(window.width(), window.height());
    let Some(mouse_pos) = window.cursor_position() else { return };
    let mut mouse_normalized_screen_pos = (mouse_pos / window_size) * 2. - Vec2::ONE;
    mouse_normalized_screen_pos.y = -mouse_normalized_screen_pos.y;
    let mouse_world_pos = camera_pos.translation.truncate() + mouse_normalized_screen_pos * Vec2::new(projection.area.max.x, projection.area.max.y) * projection.scale;

    projection.scale -= 0.05 * delta_zoom * projection.scale;

    camera_pos.translation = (mouse_world_pos - mouse_normalized_screen_pos * Vec2::new(projection.area.max.x, projection.area.max.y) * projection.scale).extend(camera_pos.translation.z);
}


/// Used to help identify our main camera
#[derive(Component)]
pub struct MainCamera;

fn init(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut arrow_handle: ResMut<ArrowHandle>
) {
    commands.spawn((
        Camera2dBundle::default(),
        MainCamera
    ));
    
    let arrow: Handle<Image> = asset_server.load("arrow.png");
    *arrow_handle = ArrowHandle(Some(arrow));
}









