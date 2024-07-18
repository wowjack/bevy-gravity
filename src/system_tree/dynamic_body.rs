// Bodies that do not affect gravity but are affected by it

use particular::{math::DVec2, PointMass};

#[derive(Clone, Debug, PartialEq)]
pub struct DynamicBody {
    pub position: DVec2,
    pub velocity: DVec2,
    pub mass: f64, // maybe gravitational parameter
}
impl DynamicBody {
    pub fn as_point_mass(&self) -> PointMass<DVec2, f64> {
        PointMass { position: self.position, mass: self.mass }
    }
}