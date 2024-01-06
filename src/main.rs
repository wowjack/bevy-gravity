use bevy::{prelude::*, input::mouse::{MouseWheel, MouseScrollUnit}, sprite::{Mesh2dHandle, MaterialMesh2dBundle}};
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy_egui::EguiPlugin;
use bevy_mod_picking::prelude::*;
use bevy_prototype_lyon::prelude::ShapePlugin;
use object::*;
use path_prediction::*;
use background::*;

mod ui;
mod object;
mod path_prediction;
mod background;

#[derive(Resource)]
pub struct GameState {
    pub play: bool,
}

#[derive(Resource, Default)]
pub struct GameResources {
    pub circle_mesh: Option<Mesh2dHandle>,
    pub circle_material: Option<Handle<ColorMaterial>>
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins/*.disable::<LogPlugin>()*/,
            EguiPlugin,
            DefaultPickingPlugins.build().disable::<DebugPickingPlugin>(),
            ShapePlugin,
            FrameTimeDiagnosticsPlugin,
            //LogDiagnosticsPlugin::default(),
        ))
        .insert_resource(ClearColor(Color::rgb(0.7, 0.7, 0.7)))
        .insert_resource(ui::ObjectDetailContext::default())
        .insert_resource(ui::ObjectDetailState::default())
        .insert_resource(GameState { play: false })
        .insert_resource(GameResources::default())
        .insert_resource(ObjectSpawnOptions::default())
        .insert_resource(DraggingBackground::default())
        .add_event::<ObjectSelectedEvent>()
        .add_event::<SpawnObjectEvent>()
        .add_event::<CameraZoomed>()
        .add_event::<SelectInRectEvent>()
        .add_systems(Startup, init)
        .add_systems(Update, (
            ui::object_detail_ui,
            ui::sidebar,
            mouse_zoom,
            object_select,
            move_object,
            object_gravity,
            update_arrow,
            spawn_object,
            path_prediction,
            update_object_radius,
            escape_unselect,
            follow_object,
            scale_background,
            rect_select
        ))
        .run()
}


#[derive(Event)]
pub struct CameraZoomed(f32);

fn mouse_zoom(
    mut query: Query<(&mut OrthographicProjection, &mut Transform)>,
    mut scroll_events: EventReader<MouseWheel>,
    primary_window: Query<&Window>,
    mut zoom_eventwriter: EventWriter<CameraZoomed>,
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
        zoom_eventwriter.send(CameraZoomed(proj.scale));
    }
}


/// Used to help identify our main camera
#[derive(Component)]
pub struct MainCamera;

fn init(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut game_resources: ResMut<GameResources>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    commands.spawn((
        Camera2dBundle::default(),
        MainCamera
    )).with_children(|builder| {
        builder.spawn(BackgroundBundle::new(&mut materials, &mut meshes));
    });

    game_resources.circle_mesh = Some(meshes.add(shape::Circle {radius: 0.5, vertices: 100}.into()).into());
    game_resources.circle_material = Some(materials.add(ColorMaterial::from(Color::PURPLE)));
}










