use crate::vec::V3;
use crate::ray::Ray;
use crate::hittable::Hit;

use rand::prelude::thread_rng;
use rand::Rng;

pub trait Material {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<Ray>;
}

#[derive(PartialEq, Copy, Clone)]
pub struct Lambertian {
    albedo: V3
}

impl Lambertian {
    pub fn new(albedo: V3) -> Lambertian { Lambertian { albedo } }
}

impl Material for Lambertian {
    fn scatter(&self, ray: &Ray, &hit: &Hit) -> Option<Ray> {
        let target = hit.p() + hit.n() + rand_in_unit_sphere();
        ray.produce(ray.origin(), target - ray.direction(), self.albedo).validate()
    }
}

fn rand_in_unit_sphere() -> V3 {
    let mut rand = rand::thread_rng();
    loop {
        let v = V3::new(rand.gen(), rand.gen(), rand.gen());
        if v.sqr_length() >= 1 as f64 {
            return v.unit();
        }
    }
}

