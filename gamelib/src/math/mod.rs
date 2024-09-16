use bevy::math::DVec2;

use crate::G;


pub fn polar_to_cartesian(polar: [f64;2]) -> DVec2 {
    DVec2 { x: polar[0]*polar[1].cos(), y: polar[0]*polar[1].sin() }
}


pub fn get_orbital_speed(center_mass: f64, radius: f64) -> f64 {
    ((center_mass*G)/radius.powi(3)).sqrt()
}
pub fn get_orbital_radius(center_mu: f64, speed: f64) -> f64 {
    (center_mu*speed.powi(-2)).powf(1./3.)
}
pub fn get_center_mu(radius: f64, speed: f64) -> f64 {
    speed.powi(2) * radius.powi(3)
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn orbital_calc_inverse() {
        let center_mu = 1000.;
        let radius = 1000.;
        let orbital_speed = get_orbital_speed(center_mu, radius);
        let new_radius = get_orbital_radius(center_mu, orbital_speed);
        assert!((new_radius-radius).abs() < 0.0000001);


        let radius = 1000.;
        let speed = 0.05;
        let center_mu = get_center_mu(radius, speed);
        let new_speed = get_orbital_speed(center_mu, radius);
        println!("{speed}, {new_speed}");
        assert!((new_speed-speed).abs() < 0.0000001);
    }
}


/// Get the radius of the hill sphere 
pub fn get_suggested_system_radius(central_mass: f64, orbital_mass: f64, orbital_radius: f64) -> f64 {
    0.75 * orbital_radius * (orbital_mass / (3.*central_mass)).powf(1./3.)
}





