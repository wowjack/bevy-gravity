use bevy::sprite::MaterialMesh2dBundle;

use crate::{physics::MassiveObject, CircleAssets};

use super::*;

#[derive(Bundle)]
pub struct VisualObjectBundle {
    pub object: MassiveObject,
    pub appearance: Appearance,
    pub material_mesh_bundle: MaterialMesh2dBundle<ColorMaterial>,
    pub on_select: On::<Pointer<Select>>,
    pub on_deselect: On::<Pointer<Deselect>>,
    pub pickable_bundle: PickableBundle,
}
impl VisualObjectBundle {
    pub fn new(object: MassiveObject, radius: f32, circle_assets: &CircleAssets) -> Self {
        Self { 
            object,
            appearance: Appearance::new(radius),
            ..Self::default(circle_assets)
        }
    }
    pub fn from_object(object: MassiveObject, circle_assets: &CircleAssets) -> Self {
        Self { object, ..Self::default(circle_assets) }
    }
    pub fn with_radius(mut self, radius: f32) -> Self {
        self.appearance.radius = radius;
        return self
    }

    pub fn default(circle_assets: &CircleAssets) -> Self {
        Self {
            object: MassiveObject::default(),
            appearance: Appearance::new(1.),
            material_mesh_bundle: MaterialMesh2dBundle {
                mesh: circle_assets.mesh.clone().into(),
                material: circle_assets.default_color.clone(),
                ..default()
            },
            on_select: On::<Pointer<Select>>::target_insert((VelocityArrow, FuturePath)),
            on_deselect: On::<Pointer<Deselect>>::target_remove::<(VelocityArrow, FuturePath)>(),
            pickable_bundle: PickableBundle::default()
        }
    }
}


