use bevy::math::DVec2;



pub fn polar_to_cartesian(polar: [f64;2]) -> DVec2 {
    DVec2 { x: polar[0]*polar[1].cos(), y: polar[0]*polar[1].sin() }
}



pub fn distance_point_to_line_segment(line_segment: [DVec2;2], point: DVec2) {
    todo!()
    
}


pub fn get_orbital_speed(center_mass: f64, radius: f64) -> f64 {
    (center_mass/radius.powi(3)).sqrt()
}
pub fn get_orbital_radius(center_mass: f64, speed: f64) -> f64 {
    (center_mass*speed.powi(-2)).powf(1./3.)
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn orbital_calc_inverse() {
        let center_mass = 1000.;
        let radius = 1000.;
        let orbital_speed = get_orbital_speed(center_mass, radius);
        let new_radius = get_orbital_radius(center_mass, orbital_speed);
        assert!((new_radius-radius).abs() < 0.0000001);
    }
}



