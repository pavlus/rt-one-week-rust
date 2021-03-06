use crate::random::rand_in_unit_sphere;

use super::Texture;
use super::{Hit, Material, Ray};
use crate::scatter::Scatter;
use crate::scatter::Scatter::Diffuse;
use crate::pdf::IsotropicPDF;
use core::f64::consts;
use crate::vec::V3;

#[derive(Debug)]
pub struct Isotropic<T> {
    albedo: T
}

impl<T: Texture> Isotropic<T> {
    pub fn new(albedo: T) -> Isotropic<T> {
        Isotropic { albedo }
    }
}

impl<T: Clone> Clone for Isotropic<T> {
    fn clone(&self) -> Self {
        Isotropic { albedo: self.albedo.clone() }
    }
}

impl<T: Texture> Material for Isotropic<T> {
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

    #[inline]
    fn scatter_with_pdf(&self, _: &Ray, hit: &Hit) -> Option<Scatter> {
        Some(Diffuse(
            Box::new(IsotropicPDF::from_w(&hit.normal.unit())),
            self.albedo.value(hit.u, hit.v, hit.point))
        )
    }

    //todo: check that integrates to the same value as others
    #[inline]
    fn scattering_pdf(&self, _hit: &Hit, _direction: &V3) -> f64 {
        // 1/ (4*pi), where 4*pi is the solid angle of full sphere
        0.25 * consts::FRAC_1_PI
    }


}
