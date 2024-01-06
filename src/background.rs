use std::cmp::min;

use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_mod_picking::{prelude::*, events::{Pointer, DragStart}};

use crate::{CameraZoomed, MainCamera};


#[derive(Resource, Default)]
pub struct DraggingBackground {
    pub start: Vec2,
    pub change: Vec2,
    pub rect: Option<Entity>,
}

#[derive(Component)]
pub struct Background;

#[derive(Bundle)]
pub struct BackgroundBundle {
    pub material_mesh_bundle: MaterialMesh2dBundle<ColorMaterial>,
    pub drag_start: On::<Pointer<DragStart>>,
    pub dragging: On::<Pointer<Drag>>,
    pub drag_end: On::<Pointer<DragEnd>>,
    pub background: Background
}
impl BackgroundBundle {
    pub fn new(materials: &mut ResMut<Assets<ColorMaterial>>, meshes: &mut ResMut<Assets<Mesh>>) -> Self {
        Self { 
            material_mesh_bundle: MaterialMesh2dBundle{
                mesh: meshes.add(shape::Box::new(1.5, 1.5, 1.5).into()).into(),
                material: materials.add(Color::GRAY.into()).into(),
                transform: Transform::from_scale(Vec3::new(5000., 5000., 1.)).with_translation(Vec3::new(0.,0.,-1000.)),
                visibility: Visibility::Visible,
                ..default()
            },
            drag_start: On::<Pointer<DragStart>>::run(drag_start),
            dragging: On::<Pointer<Drag>>::run(drag),
            drag_end: On::<Pointer<DragEnd>>::run(drag_end),
            background: Background,
        }
    }
}

#[derive(Component)]
pub struct DragRectangle;

fn drag_start(event: Listener<Pointer<DragStart>>, mut data: ResMut<DraggingBackground>, mut commands: Commands) {
    let Some(start_pos) = event.event.hit.position else { return };
    data.start = start_pos.truncate();

    let rect_entity = commands.spawn((
        SpriteBundle {
            sprite: Sprite { color: Color::rgba(1., 1., 1., 0.2), ..default() },
            transform: Transform::from_translation(Vec3::new(data.start.x, data.start.y, -5.)).with_scale(Vec3::new(0., 0., 0.)),
            ..default()
        },
        DragRectangle
    )).id();

    data.rect = Some(rect_entity);
}

fn drag(event: Listener<Pointer<Drag>>, mut data: ResMut<DraggingBackground>, mut rect_query: Query<&mut Transform, With<DragRectangle>>, projection_query: Query<&OrthographicProjection, With<MainCamera>>) {
    let projection = projection_query.single();
    data.change +=  event.delta * projection.scale;
    if data.rect.is_none() {return}
    let Ok(mut rect) = rect_query.get_mut(data.rect.unwrap()) else { return };
    
    rect.translation = Vec3::new(data.start.x + data.change.x/2., data.start.y - data.change.y/2., -5.);
    rect.scale = Vec3::from((data.change, 1.));
}

fn drag_end(_event: Listener<Pointer<DragEnd>>, mut data: ResMut<DraggingBackground>, mut commands: Commands, mut select_event_writer: EventWriter<SelectInRectEvent>) {
    let Some(entity) = data.rect else { return };
    commands.entity(entity).despawn_recursive();
    data.rect = None;

    let end = Vec2::new(data.start.x + data.change.x, data.start.y - data.change.y);

    select_event_writer.send(SelectInRectEvent { 
        min: Vec2::new(if data.start.x < end.x {data.start.x} else {end.x}, if data.start.y < end.y {data.start.y} else {end.y}),
        max: Vec2::new(if data.start.x > end.x {data.start.x} else {end.x}, if data.start.y > end.y {data.start.y} else {end.y}) 
    });
}

#[derive(Event)]
pub struct SelectInRectEvent {
    min: Vec2,
    max: Vec2,
}



pub fn scale_background(
    mut events: EventReader<CameraZoomed>,
    mut background_query: Query<&mut Transform, With<Background>>,
    projection_query: Query<&OrthographicProjection, With<MainCamera>>
) {
    if events.is_empty() {return}

    let mut background = background_query.single_mut();
    let projection = projection_query.single();

    for _event in events.read() {
        background.scale = Vec3::new(projection.area.width(), projection.area.height(), 1.);
    }
}