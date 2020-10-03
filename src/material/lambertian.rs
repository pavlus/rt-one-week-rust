
use super::{Color, Texture};
use super::{Hit, Material, Ray, V3};
use crate::scatter::Scatter;
use crate::pdf::{CosinePDF, PDF};

#[derive(Debug)]
pub struct Lambertian {
    texture: Box<dyn Texture>
}

impl Lambertian {
    pub fn new(albedo: Color) -> Lambertian { Lambertian { texture: Box::new(albedo) } }
    #[deprecated]
    pub fn color(albedo: V3) -> Lambertian { Lambertian { texture: Box::new(Color(albedo)) } }
    pub fn texture(texture: Box<dyn Texture>) -> Lambertian { Lambertian { texture } }
}

impl Material for Lambertian {
    fn scatter(&self, ray: &Ray, &hit: &Hit) -> Option<Ray> {
        let albedo = self.texture.value(hit.u, hit.v, hit.point);
        let target = CosinePDF::from_w(&hit.normal).generate();
        Some(ray.produce(hit.point, target, albedo.0))
    }

    fn scatter_with_pdf(&self, _: &Ray, hit: &Hit) -> Option<Scatter> {
        let albedo = self.texture.value(hit.u, hit.v, hit.point);
        Some(Scatter::Diffuse(Box::new(CosinePDF::from_w(&hit.normal)), albedo))
    }

    fn scattering_pdf(&self, hit: &Hit, direction: &V3) -> f64 {
        CosinePDF::from_w(&hit.normal).value(direction, hit)
    }
}
