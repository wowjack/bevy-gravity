use bevy::math::DVec2;



pub fn polar_to_cartesian(polar: [f64;2]) -> DVec2 {
    DVec2 { x: polar[0]*polar[1].cos(), y: polar[0]*polar[1].sin() }
}



pub fn distance_point_to_line_segment(line_segment: [DVec2;2], point: DVec2) {
    let d = line_segment[1] - line_segment[0];
    
}