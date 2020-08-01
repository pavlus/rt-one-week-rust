use crate::random;

use super::{Color, Texture};
use super::{Hit, Material, Ray, V3};
use core::f64::consts::PI;
use crate::onb::ONB;
use crate::scatter::Scatter;
use crate::pdf::CosinePDF;

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
        let target = 0.5 * (hit.normal + random::rand_in_unit_sphere());
        Some(ray.produce(hit.point, target, self.texture.value(hit.u, hit.v, hit.point).0))
    }

    fn scatter_with_pdf(&self, _: &Ray, hit: &Hit) -> Option<Scatter> {
        let albedo = self.texture.value(hit.u, hit.v, hit.point);
        Some(Scatter::Diffuse(Box::new(CosinePDF::from_w(&hit.normal)), albedo))
    }

    fn scattering_pdf(&self, hit: &Hit, scattered: &Ray) -> f64 {
        let cosine = hit.normal.dot(scattered.direction.unit());
        if cosine < 0.0 { 0.0 } else { cosine / PI }
    }
}

