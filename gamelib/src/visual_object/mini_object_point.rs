use super::*;
use bevy_vector_shapes::{painter::ShapePainter, shapes::DiscPainter};

//Draw a little point on top of objects that are too small to see


pub fn draw_mini_object_point(
    mut painter: ShapePainter,
    object_query: Query<(&Transform, &VisualObjectData)>,
) {
    for (transform, VisualObjectData { color, .. }) in object_query.iter().filter(|(t, _)| t.scale.x < 2.) {
        painter.set_translation(transform.translation);
        painter.set_color(*color);
        painter.circle(2.);
    }
}