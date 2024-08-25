use bevy::sprite::MaterialMesh2dBundle;
use itertools::Itertools;
use crate::pseudo_camera::camera::CameraState;

use super::*;

#[derive(Resource, Default)]
pub struct SelectedObjects {
    pub selected: Vec<Entity>,
    pub focused: Option<(Entity, VisualObjectData)>,
}


#[derive(Event)]
pub struct SelectInRectEvent {
    pub min: Vec2,
    pub max: Vec2,
}


#[derive(Component, Default)]
pub struct BackgroundRect {
    pub drag_start: Option<Vec2>,
    pub drag_end: Option<Vec2>,
}

pub fn rect_select(
    mut er: EventReader<SelectInRectEvent>,
    object_query: Query<(Entity, &Transform), With<VisualObjectData>>,
    mut selected_objects_resource: ResMut<SelectedObjects>,
) {
    for e in er.read() {
        let selected_objects =  object_query.iter()
            .filter_map(|(entity, Transform { translation, ..})| 
                if translation.x <= e.max.x && translation.x >= e.min.x && translation.y <= e.max.y && translation.y >= e.min.y {
                    Some(entity)
                } else {
                    None
                }
            ).collect_vec();
        if selected_objects.is_empty() { continue }
        selected_objects_resource.selected = selected_objects;
        selected_objects_resource.focused = None;
        if selected_objects_resource.selected.len() == 1 {
            selected_objects_resource.focused = Some((selected_objects_resource.selected.first().unwrap().clone(), VisualObjectData::default()))
        }
    }
}

pub fn spawn_background_rect(
    mut commands: Commands,
    mut colors: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    commands.spawn((
        BackgroundRect::default(),
        MaterialMesh2dBundle {
            material: colors.add(ColorMaterial { color: Color::rgb_u8(3, 0, 7), texture: None }),
            mesh: meshes.add(bevy::math::primitives::Rectangle::new(10_000., 10_000.)).into(),
            transform: Transform::from_translation(Vec3::Z*-1000.),
            ..default()
        },
        On::<Pointer<DragStart>>::target_component_mut(|listener, background: &mut BackgroundRect| background.drag_start = listener.hit.position.map(|v| v.truncate())),
        On::<Pointer<Drag>>::run(|listener: Listener<Pointer<Drag>>, mut background_query: Query<&mut BackgroundRect>| {
            let Ok(mut background_rect) = background_query.get_mut(listener.target) else { return };
            let Some(start_pos) = background_rect.drag_start else { return };
            let end_pos = Vec2::new(start_pos.x + listener.distance.x, start_pos.y - listener.distance.y);
            background_rect.drag_end = Some(end_pos);
        }),
        On::<Pointer<DragEnd>>::run(|listener: Listener<Pointer<DragEnd>>, mut background_query: Query<&mut BackgroundRect>, mut ew: EventWriter<SelectInRectEvent>| {
            let Ok(mut background) = background_query.get_mut(listener.target) else { return };
            let Some(start_pos) = background.drag_start.take() else { return };
            background.drag_end = None;
            let end_pos = Vec2::new(start_pos.x + listener.distance.x, start_pos.y - listener.distance.y);
            ew.send(SelectInRectEvent { min: Vec2::new(start_pos.x.min(end_pos.x), start_pos.y.min(end_pos.y)), max: Vec2::new(start_pos.x.max(end_pos.x), start_pos.y.max(end_pos.y)) });
        }),
    ));
}


pub fn draw_selection_rect(background_query: Query<&BackgroundRect>, mut gizmos: Gizmos) {
    let background = background_query.single();
    let Some(start_pos) = background.drag_start else { return };
    let Some(end_pos) = background.drag_end else { return };
    gizmos.rect_2d(
        (start_pos + end_pos) / 2.,
        0.,
        (start_pos - end_pos).abs(),
        Color::linear_rgb(0.75, 0.75, 0.75),
    );
}


pub fn object_selected(
    listener: Listener<Pointer<Select>>,
    mut selected_objects: ResMut<SelectedObjects>,
) {
    if selected_objects.selected.contains(&listener.target) == false {
        selected_objects.selected = vec![listener.target];
    }
    selected_objects.focused = Some((listener.target, VisualObjectData::default()));
}



pub fn draw_selected_object_halo(
    selected_objects: Res<SelectedObjects>,
    object_query: Query<(&Transform, &VisualObjectData)>,
    camera_query: Query<&CameraState>,
    mut gizmos: Gizmos
) {
    if selected_objects.selected.is_empty() { return }
    let camera = camera_query.single();
    for e in &selected_objects.selected {
        let Ok((trans, object)) = object_query.get(e.clone()) else { continue };
        gizmos.circle_2d(trans.translation.truncate(), object.radius as f32*camera.get_scale(), Color::WHITE).resolution(CIRCLE_VERTICES);
    }
}


/// Update the data for the focused object in the SelectedObjects resource
pub fn update_focused_object_data(
    object_query: Query<&VisualObjectData>,
    mut selected_objects: ResMut<SelectedObjects>,
) {
    let Some((focused_entity, _)) = selected_objects.focused.take() else { return };
    let Ok(data) = object_query.get(focused_entity) else { return };
    selected_objects.focused = Some((focused_entity, data.clone()));
}