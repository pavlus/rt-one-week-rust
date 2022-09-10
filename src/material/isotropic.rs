use super::Texture;
use super::{Hit, Material, RayCtx};
use crate::scatter::Scatter;
use crate::scatter::Scatter::Diffuse;
use crate::pdf::IsotropicPDF;

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

    #[inline]
    fn scatter_with_pdf(&self, _: RayCtx, hit: &Hit) -> Option<Scatter> {
        Some(Diffuse(
            Box::new(IsotropicPDF),
            self.albedo.value(&hit.uv, &hit.point))
        )
    }

}
