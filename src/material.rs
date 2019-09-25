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
        let target = 0.5 * (hit.normal() + rand_in_unit_sphere());
        Some(ray.produce(hit.point(), target, self.albedo))
    }
}


#[derive(PartialEq, Copy, Clone)]
pub struct Metal {
    albedo: V3,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: V3) -> Metal { Metal { albedo, fuzz: 0.0 } }
    pub fn new_fuzzed(albedo: V3, fuzz_factor: f64) -> Metal {
        Metal { albedo, fuzz: if fuzz_factor < 1.0 { fuzz_factor } else { 1.0 } }
    }

    fn fuzz(self, vector: V3) -> V3 {
        self.fuzz * rand_in_unit_sphere() + vector
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, &hit: &Hit) -> Option<Ray> {
        let unit_direction = ray.direction().unit();
        let reflected = unit_direction.reflect(hit.normal());
        if reflected.dot(hit.normal()) > 0.0 {
            Some(ray.produce(hit.point(), self.fuzz(reflected), self.albedo))
        } else {
            None
        }
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

