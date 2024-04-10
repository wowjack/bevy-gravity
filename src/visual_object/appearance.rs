use bevy_prototype_lyon::{draw::Fill, entity::ShapeBundle, geometry::GeometryBuilder, shapes::{self, RegularPolygonFeature}};

use super::*;

#[derive(Bundle)]
pub struct AppearanceBundle {
    pub appearance: Appearance,
    pub shape_bundle: ShapeBundle,
    pub fill: Fill
}
impl Default for AppearanceBundle {
    fn default() -> Self {
        Self {
            appearance: Appearance::new(1., Color::BISQUE),
            shape_bundle: ShapeBundle {
                path: GeometryBuilder::build_as(&shapes::RegularPolygon { sides: 50, center: Vec2::ZERO, feature: RegularPolygonFeature::Radius(1.)}),
                ..Default::default()
            },
            fill: Fill::color(Color::BISQUE)
        }
    }
}
impl AppearanceBundle {
    pub fn new(radius: f32, color: Color) -> Self {
        Self {
            appearance: Appearance::new(radius, color),
            shape_bundle: ShapeBundle {
                path: GeometryBuilder::build_as(&shapes::RegularPolygon { sides: 50, center: Vec2::ZERO, feature: RegularPolygonFeature::Radius(radius)}),
                ..Default::default()
            },
            fill: Fill::color(color)
        }
    }
}


#[derive(Component)]
pub struct Appearance {
    pub radius: f32,
    pub color: Color,
}
impl Appearance {
    pub fn new(radius: f32, color: Color) -> Self {
        Self { radius, color }
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