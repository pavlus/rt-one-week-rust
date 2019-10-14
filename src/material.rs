use crate::vec::V3;
use crate::ray::Ray;
use crate::hittable::Hit;
use crate::texture::{Texture, Color};

use crate::random;
use std::fmt::Debug;
use std::alloc::handle_alloc_error;

pub trait Material: Debug {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<Ray>;
}


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
        let target = 0.5 * (hit.normal() + rand_in_unit_sphere());
        Some(ray.produce(hit.point(), target, self.texture.value(0.0, 0.0, hit.point()).0))
    }
}


#[derive(PartialEq, Copy, Clone, Debug)]
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


#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Dielectric {
    albedo: V3,
    ref_idx: f64,
}

impl Dielectric {
    pub fn new(ref_idx: f64) -> Dielectric { Dielectric { albedo: V3::ones(), ref_idx } }
    pub fn new_colored(albedo: V3, ref_idx: f64) -> Dielectric {
        Dielectric { albedo, ref_idx }
    }
    fn schlick(self, cosine: f64) -> f64 {
        let mut r0 = (1.0 - self.ref_idx) / (1.0 + self.ref_idx);
        r0 *= r0;
        return r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0);
    }


    fn refract(v: V3, normal: V3, ni_over_nt: f64) -> Option<V3> {
        let unit = v.unit();
        let dt = unit.dot(normal);
        let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
        return if discriminant > 0.0 {
            Some(ni_over_nt * (v - dt * normal) - discriminant.sqrt() * normal)
        } else { None };
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, &hit: &Hit) -> Option<Ray> {
        let unit_direction = ray.direction().unit();

        let cosine: f64;
        let outward_normal: V3;
        let ni_over_nt: f64;

        let vector_cosine = unit_direction.dot(hit.normal());
        if vector_cosine > 0.0 {
            outward_normal = -hit.normal();
            ni_over_nt = self.ref_idx;
            cosine = (1.0 - self.ref_idx * self.ref_idx * (1.0 - vector_cosine * vector_cosine)).sqrt();
        } else {
            outward_normal = hit.normal();
            ni_over_nt = 1.0 / self.ref_idx;
            cosine = -vector_cosine;
        }

        let refracted: Option<V3> = Dielectric::refract(unit_direction, outward_normal, ni_over_nt);
        let reflected = ray.direction().reflect(hit.normal());

        refracted
            .filter(|_| self.schlick(cosine) < random::next_std_f64())
            .map(|refracted| ray.produce(hit.point(), refracted, self.albedo))
            .or_else(|| Some(ray.produce(hit.point(), reflected, V3::ones())))
    }
}


fn rand_in_unit_sphere() -> V3 {
    loop {
        let v = V3::new(random::next_std_f64(), random::next_std_f64(), random::next_std_f64());
        if v.sqr_length() >= 1 as f64 {
            return v.unit();
        }
    }
}


