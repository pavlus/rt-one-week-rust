use std::borrow::Borrow;
use std::f64::consts;

use super::{AABB, Hit, Hittable, Material, Ray, V3};

#[derive(Debug)]
pub struct Sphere {
    center: V3,
    radius: f64,
    material: Box<dyn Material>,
}

impl Sphere {
    pub fn new(center: V3, radius: f64, material: Box<dyn Material>) -> Sphere {
        Sphere { center, radius, material }
    }
    fn center(&self, time: f32) -> V3 {
        self.center
    }
    fn radius(&self) -> f64 { self.radius }
    fn material(&self) -> &Box<dyn Material> { &self.material }

    fn aabb(&self, t0: f32, t1: f32) -> AABB {
        AABB::new(self.center - self.radius, self.center + self.radius)
    }

    fn uv(unit_point: V3) -> (f64, f64) {
        let phi = f64::atan2(unit_point.z, unit_point.x);
        let theta = unit_point.y.asin();

        let u = 1.0 - (phi + consts::PI) / (2.0 * consts::PI);
        let v = (theta + consts::FRAC_PI_2) / consts::PI;
        (u, v)
    }
}

#[derive(Debug)]
pub struct MovingSphere {
    center_t0: V3,
    center_t1: V3,
    time0: f32,
    duration: f32,
    radius: f64,
    material: Box<dyn Material>,
}

impl MovingSphere {
    pub fn new(center_t0: V3, center_t1: V3, time0: f32, time1: f32, radius: f64, material: Box<dyn Material>) -> MovingSphere {
        MovingSphere {
            center_t0,
            center_t1,
            time0,
            duration: time1 - time0,
            radius,
            material,
        }
    }
    fn center(&self, time: f32) -> V3 {
        let scale = (time - self.time0) / self.duration;
        self.center_t0 + scale * (self.center_t1 - self.center_t0)
    }
    fn radius(&self) -> f64 { self.radius }
    fn material<'a>(&'a self) -> &'a Box<dyn Material> { &self.material }
    fn aabb(&self, t: f32) -> AABB {
        AABB::new(self.center(t) - self.radius(), self.center(t) + self.radius())
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, dist_min: f64, dist_max: f64) -> Option<Hit> {
        let oc = ray.origin - self.center(ray.time);
        let a = ray.direction.sqr_length();
        let b = oc.dot(ray.direction);
        let c = oc.sqr_length() - (self.radius() * self.radius()) as f64;
        let discr_sqr = b * b - a * c;

        let get_hit = |ray: &Ray, dist: f64| -> Hit {
            let p = ray.point_at(dist);
            let n = (p - self.center(ray.time)) / self.radius();
            let (u, v) = Sphere::uv(n);
            return Hit::new(dist, p, n, self.material().borrow(), u, v);
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

    fn bounding_box(&self, t_min: f32, t_max: f32) -> Option<AABB> {
        Some(self.aabb(t_min, t_max))
    }
}

impl Hittable for MovingSphere {
    fn hit(&self, ray: &Ray, dist_min: f64, dist_max: f64) -> Option<Hit> {
//        if !self.bounding_box(ray.time(), ray.time())
//            .unwrap().hit(ray, dist_min, dist_max) { return None; }

        let oc = ray.origin - self.center(ray.time);
        let a = ray.direction.sqr_length();
        let b = oc.dot(ray.direction);
        let c = oc.sqr_length() - (self.radius * self.radius) as f64;
        let discr_sqr = b * b - a * c;

        let get_hit = |ray: &Ray, dist: f64| -> Hit {
            let p = ray.point_at(dist);
            let n = (p - self.center(ray.time)) / self.radius;
            let (u, v) = Sphere::uv(n);
            return Hit::new(dist, p, n, self.material.borrow(), u, v);
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

    fn bounding_box(&self, t_min: f32, t_max: f32) -> Option<AABB> {
        Some(self.aabb(t_min) + self.aabb(t_max))
    }
}
