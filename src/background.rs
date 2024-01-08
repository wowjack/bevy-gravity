use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_mod_picking::{prelude::*, events::{Pointer, DragStart}};

use crate::{MainCamera, object::{object::MassiveObject, select::ObjectsSelectedEvent, spawn::VisualObject}, zoom::ProjectionScaleChange};


#[derive(Resource, Default)]
pub struct DraggingBackground {
    pub start: Vec2,
    pub change: Vec2,
    pub rect: Option<Entity>,
    pub drag_type: DragType
}
#[derive(Default)]
pub enum DragType {
    #[default]
    Select,
    Move
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
                mesh: meshes.add(shape::Quad::new(Vec2::new(1., 1.)).into()).into(),
                material: materials.add(Color::rgba(1.,1.,1.,0.).into()).into(),
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

fn drag_start(event: Listener<Pointer<DragStart>>, mut data: ResMut<DraggingBackground>, mut commands: Commands, keyboard: Res<Input<KeyCode>>) {
    let Some(start_pos) = event.event.hit.position else { return };
    data.start = start_pos.truncate();
    data.change = Vec2::ZERO;

    data.drag_type = if keyboard.pressed(KeyCode::Space) { DragType::Move } else { default() };
    if let DragType::Move = data.drag_type { return }

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

fn drag(event: Listener<Pointer<Drag>>, mut data: ResMut<DraggingBackground>, mut rect_query: Query<&mut Transform, (With<DragRectangle>, Without<MainCamera>)>, mut projection_query: Query<(&OrthographicProjection, &mut Transform), With<MainCamera>>) {
    let (projection, mut camera_trans) = projection_query.single_mut();
    match data.drag_type {
        DragType::Select => {
            data.change +=  event.delta * projection.scale;
            if data.rect.is_none() {return}
            let Ok(mut rect) = rect_query.get_mut(data.rect.unwrap()) else { return };

            rect.translation = Vec3::new(data.start.x + data.change.x/2., data.start.y - data.change.y/2., -5.);
            rect.scale = Vec3::from((data.change, 1.));
        },
        DragType::Move => {
            camera_trans.translation.x -= event.delta.x * projection.scale;
            camera_trans.translation.y += event.delta.y * projection.scale;
        }
    }
    
}

fn drag_end(_event: Listener<Pointer<DragEnd>>, mut data: ResMut<DraggingBackground>, mut commands: Commands, mut select_event_writer: EventWriter<SelectInRectEvent>) {
    if let DragType::Move = data.drag_type { return }

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
    pub min: Vec2,
    pub max: Vec2,
}

pub fn rect_select(mut events: EventReader<SelectInRectEvent>, mut object_query: Query<(Entity, &GlobalTransform), With<MassiveObject>>, mut event_writer: EventWriter<ObjectsSelectedEvent>) {
    for event in events.read() {
        let entities: Vec<Entity> = object_query
            .iter_mut()
            .filter(|(_, t)| t.translation().x > event.min.x && t.translation().y > event.min.y && t.translation().x < event.max.x && t.translation().y < event.max.y) 
            .map(|(e, _)| e)
            .collect();
        if entities.len() < 1 { continue }
        event_writer.send(ObjectsSelectedEvent(entities));
    }
}



pub fn scale_background(
    mut events: EventReader<ProjectionScaleChange>,
    mut background_query: Query<&mut Transform, With<Background>>,
    projection_query: Query<&OrthographicProjection, With<MainCamera>>
) {
    if events.is_empty() {return}

    let mut background = background_query.single_mut();
    let projection = projection_query.single();

    for _event in events.read() {
        background.scale = Vec3::new(projection.area.width()*2., projection.area.height()*2., 1.);
    }
}