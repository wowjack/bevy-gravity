use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use super::*;

#[derive(Bundle)]
pub struct VisualObjectBundle {
    pub object_data: VisualObjectData,
    pub material_mesh_bundle: MaterialMesh2dBundle<ColorMaterial>,
    pub on_select: On::<Pointer<Select>>,
    pub on_drag: On::<Pointer<Drag>>,
    pub pickable_bundle: PickableBundle,
}
impl VisualObjectBundle {
    pub fn new(object_data: VisualObjectData, mesh: Mesh2dHandle, colors: &mut Assets<ColorMaterial>) -> Self {
        let material = colors.add(object_data.color);
        Self {
            object_data,
            material_mesh_bundle: MaterialMesh2dBundle { material, mesh, ..default() },
            on_select: On::<Pointer<Select>>::run(object_selected),
            on_drag: On::<Pointer<Drag>>::run(drag_object),
            pickable_bundle: PickableBundle::default()
        }
    }
}



