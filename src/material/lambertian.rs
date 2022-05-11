use super::{Color, Texture};
use super::{Hit, Material, RayCtx};
use crate::scatter::Scatter;
use crate::pdf::{CosinePDF, PDF};

#[derive(Debug)]
pub struct Lambertian<T> {
    texture: T,
}

impl <T: Texture> Lambertian<T> {
    pub fn new(albedo: Color) -> Lambertian<Color> { Lambertian { texture: albedo } }
    pub fn texture(texture: T) -> Lambertian<T> { Lambertian { texture } }
}

impl <T: Texture> Material for Lambertian<T> {
    fn scatter(&self, ray_ctx: &RayCtx, &hit: &Hit) -> Option<RayCtx> {
        let albedo = self.texture.value(&hit.uv, &hit.point);
        let target = CosinePDF::from_w(hit.normal).generate();
        Some(ray_ctx.produce(hit.point, target, albedo))
    }

    fn scatter_with_pdf(&self, _: &RayCtx, hit: &Hit) -> Option<Scatter> {
        let albedo = self.texture.value(&hit.uv, &hit.point);
        Some(Scatter::Diffuse(Box::new(CosinePDF::from_w(hit.normal)), albedo))
    }
}
