use bevy::sprite::Mesh2dHandle;

use super::*;

/// Changes to an object's radius or color does not change the physics so it gets handled here
#[derive(Event)]
pub struct VisualChangeEvent {
    pub target: Entity,
    pub change: VisualChange
}


pub enum VisualChange {
    SetRadius(f32),
    SetColor(Color)
}



pub fn process_visual_change_event(
    mut er: EventReader<VisualChangeEvent>,
    mut object_query: Query<(&mut VisualObjectData, &mut Handle<ColorMaterial>)>,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
) {
    for event in er.read() {
        let Ok((mut data, mut color)) = object_query.get_mut(event.target) else { continue };
        match event.change {
            VisualChange::SetRadius(new_radius) => {
                data.radius = new_radius;
            },
            VisualChange::SetColor(new_color) => {
                //println!("Changing color to {:?}", new_color);
                data.color = new_color;
                *color = color_materials.add(ColorMaterial::from(new_color));
            }
        }
    }
}