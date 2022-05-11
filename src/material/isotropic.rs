use crate::random::rand_in_unit_sphere;

use super::Texture;
use super::{Hit, Material, RayCtx};
use crate::scatter::Scatter;
use crate::scatter::Scatter::Diffuse;
use crate::pdf::IsotropicPDF;
use nalgebra::Unit;

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
    fn scatter(&self, ray_ctx: &RayCtx, hit: &Hit) -> Option<RayCtx> {
        Some(
            ray_ctx.produce(
                hit.point,
                Unit::new_unchecked(rand_in_unit_sphere().coords),
                self.albedo.value(&hit.uv, &hit.point))
        )
    }

    #[inline]
    fn scatter_with_pdf(&self, _: &RayCtx, hit: &Hit) -> Option<Scatter> {
        Some(Diffuse(
            Box::new(IsotropicPDF::from_w(hit.normal)),
            self.albedo.value(&hit.uv, &hit.point))
        )
    }

}
