use std::fmt::Debug;

pub use lambertian::*;
pub use metal::*;

use crate::hittable::Hit;
use crate::random;
use crate::ray::Ray;
use crate::texture::{Color, Texture};
use crate::vec::V3;

pub mod lambertian;
pub mod metal;

pub trait Material: Debug + Sync + Send {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<Ray> { None }
    fn emmit(&self, hit: &Hit) -> Color { Color(V3::zeros()) }
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
        let unit_direction = ray.direction.unit();

        let cosine: f64;
        let outward_normal: V3;
        let ni_over_nt: f64;

        let vector_cosine = unit_direction.dot(hit.normal);
        if vector_cosine > 0.0 {
            outward_normal = -hit.normal;
            ni_over_nt = self.ref_idx;
            cosine = (1.0 - self.ref_idx * self.ref_idx * (1.0 - vector_cosine * vector_cosine)).sqrt();
        } else {
            outward_normal = hit.normal;
            ni_over_nt = 1.0 / self.ref_idx;
            cosine = -vector_cosine;
        }

        let refracted: Option<V3> = Dielectric::refract(unit_direction, outward_normal, ni_over_nt);
        let reflected = ray.direction.reflect(hit.normal);

        refracted
            .filter(|_| self.schlick(cosine) < random::next_std_f64())
            .map(|refracted| ray.produce(hit.point, refracted, self.albedo))
            .or_else(|| Some(ray.produce(hit.point, reflected, V3::ones())))
    }
}


#[derive(Debug)]
pub struct DiffuseLight {
    texture: Box<dyn Texture>,
    intensity_scale: f64,
}

impl DiffuseLight {
    pub fn new(texture: Box<dyn Texture>, scale: f64) -> DiffuseLight {
        DiffuseLight { texture, intensity_scale: scale }
    }
}

impl Material for DiffuseLight {
    fn emmit(&self, hit: &Hit) -> Color {
        Color(self.intensity_scale * self.texture.value(hit.u, hit.v, hit.point).0)
    }
}
