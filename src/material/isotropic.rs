use crate::random::rand_in_unit_sphere;

use super::{Color, Texture};
use super::{Hit, Material, Ray, V3};

#[derive(Debug)]
pub struct Isotropic {
    albedo: Box<dyn Texture>
}

impl Isotropic {
    pub fn new(albedo: Box<dyn Texture>) -> Isotropic {
        Isotropic { albedo }
    }
}

impl Material for Isotropic {
    /// Isotropic material has a unit-sphere BSDF,
    /// this means that amount of reflected light
    /// is equal to the amount of transmitted light
    /// probability of reflection in any direction is the same
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<Ray> {
        Some(
            ray.produce(
                hit.point,
                rand_in_unit_sphere(),
                self.albedo.value(hit.u, hit.v, hit.point).0)
        )
    }
}
