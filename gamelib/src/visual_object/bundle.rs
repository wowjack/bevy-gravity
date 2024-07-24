use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use super::*;

#[derive(Bundle)]
pub struct VisualObjectBundle {
    pub object_data: VisualObjectData,
    pub requires_bundle: RequiresMaterialMesh,
    pub on_select: On::<Pointer<Select>>,
    pub pickable_bundle: PickableBundle,
}
impl VisualObjectBundle {
    pub fn new(object_data: VisualObjectData) -> Self {
        Self {
            object_data,
            requires_bundle: RequiresMaterialMesh,
            on_select: On::<Pointer<Select>>::run(object_selected),
            pickable_bundle: PickableBundle::default()
        }
    }
}

#[derive(Component)]
pub struct RequiresMaterialMesh;

pub fn add_material_mesh(
    query: Query<(Entity, &VisualObjectData), With<RequiresMaterialMesh>>,
    circle_mesh: Res<CircleMesh>,
    mut commands: Commands,
    mut colors: ResMut<Assets<ColorMaterial>>
) {
    if query.is_empty() { return }
    for (entity, VisualObjectData { color, .. }) in query.iter() {
        let material = colors.add(*color);
        commands.entity(entity)
            .remove::<RequiresMaterialMesh>()
            .insert(MaterialMesh2dBundle { material, mesh: circle_mesh.0.clone().into(), ..default() });
    }
}

