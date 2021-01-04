
use super::{Color, Texture};
use super::{Hit, Material, RayCtx, V3};
use crate::scatter::Scatter;
use crate::pdf::{CosinePDF, PDF};
use nalgebra::Unit;

pub struct Lambertian {
    texture: Box<dyn Texture>
}

impl Lambertian {
    pub fn new(albedo: Color) -> Lambertian { Lambertian { texture: Box::new(albedo) } }
    #[deprecated]
    pub fn color(albedo: Color) -> Lambertian { Lambertian { texture: Box::new(albedo) } }
    pub fn texture(texture: Box<dyn Texture>) -> Lambertian { Lambertian { texture } }
}

impl Material for Lambertian {
    fn scatter(&self, ray_ctx: &RayCtx, &hit: &Hit) -> Option<RayCtx> {
        let albedo = self.texture.value(hit.u, hit.v, &hit.point);
        let target = CosinePDF::from_w(&hit.normal).generate();
        Some(ray_ctx.produce(hit.point, target, albedo))
    }

    fn scatter_with_pdf(&self, _: &RayCtx, hit: &Hit) -> Option<Scatter> {
        let albedo = self.texture.value(hit.u, hit.v, &hit.point);
        Some(Scatter::Diffuse(Box::new(CosinePDF::from_w(&hit.normal)), albedo))
    }

}
