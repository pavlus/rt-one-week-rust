use std::borrow::Borrow;
use std::f64::consts;

use super::{AABB, Hit, Hittable, Material, Ray, V3};
use crate::onb::ONB;

#[derive(Debug)]
pub struct Sphere<M> {
    pub center: V3,
    pub radius: f64,
    pub material: M,
}
impl<M: Clone> Clone for Sphere<M>{
    fn clone(&self) -> Self {
        Sphere{
            material: self.material.clone(),
            ..*self
        }
    }
}

impl<M:Material> Sphere<M> {
    pub fn new(center: V3, radius: f64, material: M) -> Sphere<M> {
        Sphere { center, radius, material }
    }
    #[inline]
    fn center(&self, _: f32) -> V3 {
        self.center
    }
    #[inline]
    fn radius(&self) -> f64 { self.radius }

    fn aabb(&self, _: f32, _: f32) -> AABB {
        AABB::new(self.center - self.radius, self.center + self.radius)
    }

}
impl<M> Borrow<M> for Sphere<M>{
    fn borrow(&self) -> &M {
        &self.material
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
    #[inline]
    fn center(&self, time: f32) -> V3 {
        let scale = (time - self.time0) / self.duration;
        self.center_t0 + scale * (self.center_t1 - self.center_t0)
    }
    #[inline]
    fn radius(&self) -> f64 { self.radius }
    fn aabb(&self, t: f32) -> AABB {
        AABB::new(self.center(t) - self.radius(), self.center(t) + self.radius())
    }
}

impl<M: Material> Hittable for Sphere<M> {
    fn hit(&self, ray: &Ray, dist_min: f64, dist_max: f64) -> Option<Hit> {
        let center =self.center(ray.time);
        let radius = self.radius;
        let oc = ray.origin - center;
        let a = ray.direction.sqr_length();
        let b = oc.dot(ray.direction);
        let c = oc.sqr_length() - (radius * radius) as f64;
        let discr_sqr = b * b - a * c;

        let get_hit = |ray: &Ray, dist: f64| -> Hit {
            let p = ray.point_at(dist);
            let n = (p - center) / radius;
            let (u, v) = uv(n);
            return Hit::new(dist, p, n, &self.material, u, v);
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

    fn pdf_value(&self, origin: &V3, _: &V3, _: &Hit) -> f64 {
        let sqr_r = self.radius * self.radius;
        // todo: am i sure that hit distance doesn't mean anything?
        let direction = self.center - *origin;
        let cos_theta_max = f64::sqrt(1.0 - sqr_r / direction.sqr_length());
        let solid_angle = 2.0 * consts::PI * (1.0 - cos_theta_max);

        1.0/solid_angle
    }

    fn random(&self, origin: &V3) -> V3 {
        let direction = self.center - *origin;
        let onb = ONB::from_w(&direction);
        onb.local(random_to_sphere(self.radius(), direction.sqr_length()))
    }
}
#[inline]
fn random_to_sphere(radius: f64, sqr_dist: f64) -> V3 {
    let r1 = crate::random::next_std_f64();
    let r2 = crate::random::next_std_f64();
    let z = 1.0 + r2 * (f64::sqrt(1.0 - radius * radius / sqr_dist) - 1.0);

    let phi = 2.0 * consts::PI * r1;
    let sin_theta = f64::sqrt(1.0 - z * z);

    let (sin_phi,cos_phi) = f64::sin_cos(phi);
    let x = cos_phi * sin_theta;
    let y = sin_phi * sin_theta;

    V3::new(x, y, z)
}

impl Hittable for MovingSphere {
    fn hit(&self, ray: &Ray, dist_min: f64, dist_max: f64) -> Option<Hit> {
        let center = self.center(ray.time);
        let oc = ray.origin - center;
        let a = ray.direction.sqr_length();
        let b = oc.dot(ray.direction);
        let c = oc.sqr_length() - (self.radius * self.radius) as f64;
        let discr_sqr = b * b - a * c;

        let get_hit = |ray: &Ray, dist: f64| -> Hit {
            let p = ray.point_at(dist);
            let n = (p - center) / self.radius;
            let (u, v) = uv(n);
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

fn uv(unit_point: V3) -> (f64, f64) {
    let phi = f64::atan2(unit_point.z, unit_point.x);
    let theta = unit_point.y.asin();

    let u = 1.0 - (phi + consts::PI) / (2.0 * consts::PI);
    let v = (theta + consts::FRAC_PI_2) / consts::PI;
    (u, v)
}
