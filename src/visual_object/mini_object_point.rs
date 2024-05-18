use super::*;
use bevy_vector_shapes::{painter::ShapePainter, shapes::DiscPainter};

//Draw a little point on top of objects that are too small to see


pub fn draw_mini_object_point(
    mut painter: ShapePainter,
    object_query: Query<&Transform, With<VisualObjectData>>,
) {
    for transform in object_query.iter().filter(|t| t.scale.x < 2.) {
        painter.set_translation(transform.translation);
        painter.circle(2.);
    }
}