use std::cmp::Ordering;
use std::cmp::Ordering::Less;

use crate::ray::Ray;
use crate::vec::V3;
use crate::material::Material;
use core::borrow::Borrow;

#[derive(Copy, Clone)]
pub struct Hit<'a> {
    dist: f64,
    p: V3,
    n: V3,
    material: &'a Box<dyn Material>,
}

//impl Eq for Hit {}

impl<'a> Hit<'a> {
    pub fn new(dist: f64, p: V3, n: V3, material: &'a Box<dyn Material>) -> Hit<'a> {
        return Hit { dist, p, n, material };
    }
    pub fn p(self) -> V3 {
        self.p
    }
    pub fn n(self) -> V3 {
        self.n
    }
    pub fn dist(self) -> f64 {
        self.dist
    }
    pub fn material(self) -> &'a Box<dyn Material> {
        &self.material
    }
}


pub trait Hittable {
    fn hit(&self, ray: &Ray, dist_min: f64, dist_max: f64) -> Option<Hit>;
}


pub struct Sphere {
    center: V3,
    radius: f64,
    material: Box<dyn Material>,
}

impl Sphere {
    pub fn new(center: V3, radius: f64, material: Box<dyn Material>) -> Sphere {
        Sphere { center, radius, material }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, dist_min: f64, dist_max: f64) -> Option<Hit> {
        let oc = ray.origin() - self.center;
        let a = ray.direction().sqr_length();
        let b = 2.0 * oc.dot(ray.direction());
        let c = oc.sqr_length() - (self.radius * self.radius) as f64;
        let discr_sqr = b * b - a * c;

        let get_hit = |ray: &Ray, dist: f64| -> Hit {
            let p = ray.point_at(dist);
            let n = (p - self.center) / self.radius;
            return Hit::new(dist, p, n, &self.material);
        };

        if discr_sqr > 0.0 {
            let tmp = (b * b - a * c).sqrt();
            let x1 = (-b - tmp) / a;
            if (dist_min..dist_max).contains(&x1) {
                return Option::Some(get_hit(ray, x1));
            }
            let x2 = (-b + tmp) / a;
            if (dist_min..dist_max).contains(&x2) {
                return Option::Some(get_hit(ray, x2));
            }
            return None;
        } else {
            None
        }
    }
}

pub struct Stage {
    objects: Vec<Box<dyn Hittable>>
}

impl Stage {
    pub fn new(objects: Vec<Box<Hittable>>) -> Stage {
        Stage { objects }
    }
}

impl Hittable for Stage {
    fn hit(&self, ray: &Ray, dist_min: f64, dist_max: f64) -> Option<Hit> {
        self.objects
            .iter()
            .map(|h| h.hit(ray, dist_min, dist_max))
            .filter(Option::is_some)
            .map(Option::unwrap)
            .min_by(|s, o| s.dist().partial_cmp(&o.dist()).unwrap())
    }
}
