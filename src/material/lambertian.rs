use crate::random;

use super::{Color, Texture};
use super::{Hit, Material, Ray, V3};

#[derive(Debug)]
pub struct Lambertian {
    texture: Box<dyn Texture>
}

impl Lambertian {
    #[deprecated]
    pub fn new(albedo: V3) -> Lambertian { Lambertian { texture: Box::new(Color(albedo)) } }
    pub fn color(albedo: Color) -> Lambertian { Lambertian { texture: Box::new(albedo) } }
    pub fn texture(texture: Box<dyn Texture>) -> Lambertian { Lambertian { texture } }
}

impl Material for Lambertian {
    fn scatter(&self, ray: &Ray, &hit: &Hit) -> Option<Ray> {
        let target = 0.5 * (hit.normal + random::rand_in_unit_sphere());
        Some(ray.produce(hit.point, target, self.texture.value(hit.u, hit.v, hit.point).0))
    }
}