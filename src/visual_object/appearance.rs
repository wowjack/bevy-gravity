use bevy::sprite::MaterialMesh2dBundle;

use super::*;

#[derive(Bundle)]
pub struct AppearanceBundle {
    pub appearance: Appearance,
    //pub shape_bundle: MaterialMesh2dBundle<ColorMaterial>,
}
impl Default for AppearanceBundle {
    fn default() -> Self {
        Self {
            appearance: Appearance::new(1.),
            //shape_bundle: ShapeBundle {
            //    path: GeometryBuilder::build_as(&shapes::RegularPolygon { sides: 50, center: Vec2::ZERO, feature: RegularPolygonFeature::Radius(1.)}),
            //    ..Default::default()
            //},
            //fill: Fill::color(Color::BISQUE)
        }
    }
}
impl AppearanceBundle {
    pub fn new(radius: f32, color: Color) -> Self {
        Self {
            appearance: Appearance::new(radius),
            //shape_bundle: ShapeBundle {
            //    path: GeometryBuilder::build_as(&shapes::RegularPolygon { sides: 50, center: Vec2::ZERO, feature: RegularPolygonFeature::Radius(radius)}),
            //    ..Default::default()
            //},
            //fill: Fill::color(color)
        }
    }
}


#[derive(Component)]
pub struct Appearance {
    pub radius: f32,
}
impl Appearance {
    pub fn new(radius: f32) -> Self {
        Self { radius }
    }
}

#[derive(Event)]
pub struct AppearanceChangeEvent {
    entity: Entity,
    appearance: Appearance,
}


pub fn process_appearance_change_event(er: EventReader<AppearanceChangeEvent>, ) {
    if er.is_empty() { return }
}