use std::fmt::Debug;

pub use aabox::*;
pub use aarect::*;
pub use constant_medium::*;
pub use instance::*;
pub use list::*;
pub use sphere::*;

use crate::aabb::AABB;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec::V3;
use crate::random::rand_in_unit_sphere;
use std::f64::consts::PI;

mod sphere;
mod aarect;
mod list;
mod aabox;
mod constant_medium;
mod instance;

#[derive(Copy, Clone)]
pub struct Hit<'a> {
    pub point: V3,
    pub normal: V3,
    pub u: f64,
    pub v: f64,
    pub material: &'a dyn Material,
    pub dist: f64,
}

impl<'a> Hit<'a> {
    pub fn new(dist: f64, p: V3, n: V3, material: &'a dyn Material, u: f64, v: f64) -> Hit<'a> {
        return Hit { dist, point: p, normal: n, material, u, v };
    }
}

#[allow(unused_variables)]
pub trait Hittable: Debug + Sync {
    fn hit(&self, ray: &Ray, dist_min: f64, dist_max: f64) -> Option<Hit>;
    fn bounding_box(&self, t_min: f32, t_max: f32) -> Option<AABB> { None }

    fn pdf_value(&self, origin: &V3, direction: &V3, hit: &Hit) -> f64 {
        1.0
    }

    fn random(&self, origin: &V3) -> V3 {
        V3::new(0.0, 1.0, 0.0)
    }

}

impl Hittable for Box<dyn Hittable>
{
    fn hit(&self, ray: &Ray, dist_min: f64, dist_max: f64) -> Option<Hit> {
        Hittable::hit(&**self, ray, dist_min, dist_max)
    }

    fn bounding_box(&self, t_min: f32, t_max: f32) -> Option<AABB> {
        Hittable::bounding_box(&**self, t_min, t_max)
    }

    fn pdf_value(&self, origin: &V3, direction: &V3, hit: &Hit) -> f64 {
        Hittable::pdf_value(&**self, origin, direction, hit)
    }

    fn random(&self, origin: &V3) -> V3 {
        Hittable::random(&**self, origin)
    }
}
impl<T:Hittable> Hittable for Box<T>
{
    fn hit(&self, ray: &Ray, dist_min: f64, dist_max: f64) -> Option<Hit> {
        Hittable::hit(&**self, ray, dist_min, dist_max)
    }

    fn bounding_box(&self, t_min: f32, t_max: f32) -> Option<AABB> {
        Hittable::bounding_box(&**self, t_min, t_max)
    }

    fn pdf_value(&self, origin: &V3, direction: &V3, hit: &Hit) -> f64 {
        Hittable::pdf_value(&**self, origin, direction, hit)
    }

    fn random(&self, origin: &V3) -> V3 {
        Hittable::random(&**self, origin)
    }
}

#[derive(Debug)]
pub struct NoHit;
impl Hittable for NoHit{
    fn hit(&self, _ray: &Ray, _dist_min: f64, _dist_max: f64) -> Option<Hit> {
        None
    }

    fn bounding_box(&self, _t_min: f32, _t_max: f32) -> Option<AABB> {
        None
    }

    fn pdf_value(&self, _origin: &V3, _direction: &V3, _hit: &Hit) -> f64 {
        1.0/(PI*4.0)
    }

    fn random(&self, _origin: &V3) -> V3 {
        rand_in_unit_sphere()
    }
}

#[cfg(test)]
mod test {
    use crate::hittable::Hittable;
    use crate::random::rand_in_unit_sphere;
    use crate::texture::Color;
    use crate::material::Lambertian;
    use crate::ray::Ray;
    use crate::vec::V3;

    pub fn test_pdf_integration<T: Hittable>(hittable: T, count: usize) {
        let normal = rand_in_unit_sphere();
        let mat = Lambertian::new(Color(V3::ones()));

        let origin = rand_in_unit_sphere();
        let integral = (0..count).into_iter()
            .map(|_| {
                let dir = rand_in_unit_sphere();
                let ray = Ray::new(origin, dir, V3::ones(), 1.0, 2);
                if let Some(hit) = hittable.hit(&ray, -99999.0, 99999.0) {
                    hittable.pdf_value(&origin, &dir, &hit)
                } else { 0.0 }
            }).sum::<f64>() / (count as f64);
        let expected = 1.0 / (2.0 * std::f64::consts::PI);
        let epsilon = 1.0 / f64::cbrt(count as f64);
        assert!(
            f64::abs(integral - expected) < epsilon,
            format!("Expected: {}, actual: {}, epsilon: {}", expected, integral, epsilon)
        );
    }
}
