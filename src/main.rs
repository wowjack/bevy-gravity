use bevy::{prelude::*, input::mouse::{MouseWheel, MouseScrollUnit}};
use bevy_egui::EguiPlugin;
use bevy_mod_picking::prelude::*;
use bevy_prototype_lyon::prelude::ShapePlugin;
use object::*;
use ui::sidebar;

mod ui;
mod object;

#[derive(Resource)]
pub struct GameState {
    play: bool
}

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
        .insert_resource(GameState { play: false })
        .add_event::<ObjectSelectedEvent>()
        .add_event::<SpawnObjectEvent>()
        .add_systems(Startup, init)
        .add_systems(Update, (ui::ui_example_system, ui::sidebar, mouse_zoom, object_select, move_object, object_gravity, update_arrow, path_prediction, spawn_object))
        .run()
}


fn mouse_zoom(
    mut query: Query<(&mut OrthographicProjection, &mut Transform)>,
    mut scroll_events: EventReader<MouseWheel>,
    primary_window: Query<&Window>,
) {
    let pixels_per_line = 100.; // Maybe make configurable?
    let scroll = scroll_events
        .read()
        .map(|ev| match ev.unit {
            MouseScrollUnit::Pixel => ev.y,
            MouseScrollUnit::Line => ev.y * pixels_per_line,
        })
        .sum::<f32>();

    if scroll == 0. {
        return;
    }

    let window = primary_window.single();
    let window_size = Vec2::new(window.width(), window.height());
    let mouse_normalized_screen_pos = window
        .cursor_position()
        .map(|cursor_pos| (cursor_pos / window_size) * 2. - Vec2::ONE)
        .map(|p| Vec2::new(p.x, -p.y));

    for (mut proj, mut pos) in &mut query {
        let old_scale = proj.scale;
        proj.scale = proj.scale * (1. + -scroll * 0.001);

        // Move the camera position to normalize the projection window
        if let Some(mouse_normalized_screen_pos) = mouse_normalized_screen_pos {
            let proj_size = proj.area.max / old_scale;
            let mouse_world_pos = pos.translation.truncate()
                + mouse_normalized_screen_pos * proj_size * old_scale;
            pos.translation = (mouse_world_pos
                - mouse_normalized_screen_pos * proj_size * proj.scale)
                .extend(pos.translation.z);
        }
    }
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









