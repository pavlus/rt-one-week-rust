use std::borrow::Borrow;
use std::f64::consts;

use super::{AABB, Hit, Hittable, Material, RayCtx, V3};
use crate::random::rand_in_unit_hemisphere;

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
    fn hit(&self, ray_ctx: &RayCtx, dist_min: f64, dist_max: f64) -> Option<Hit> {
        let center =self.center(ray_ctx.time);
        let radius = self.radius;
        let oc = ray_ctx.ray.origin - center;
        let a = ray_ctx.ray.direction.sqr_length();
        let b = oc.dot(ray_ctx.ray.direction);
        let c = oc.sqr_length() - (radius * radius) as f64;
        let discr_sqr = b * b - a * c;

        let get_hit = |ray_ctx: &RayCtx, dist: f64| -> Hit {
            let p = ray_ctx.ray.point_at(dist);
            let n = (p - center) / radius;
            let (u, v) = uv(n);
            return Hit::new(dist, p, n, &self.material, u, v);
        };

        if discr_sqr > 0.0 {
            let tmp = (b * b - a * c).sqrt();
            let x1 = (-b - tmp) / a;
            if (dist_min..dist_max).contains(&x1) {
                return Option::Some(get_hit(ray_ctx, x1));
            }
            let x2 = (-b + tmp) / a;
            if (dist_min..dist_max).contains(&x2) {
                return Option::Some(get_hit(ray_ctx, x2));
            }
            return None;
        } else {
            None
        }
    }

    fn bounding_box(&self, t_min: f32, t_max: f32) -> Option<AABB> {
        Some(self.aabb(t_min, t_max))
    }

    fn pdf_value(&self, origin: &V3, _direction: &V3, _hit: &Hit) -> f64 {
        let sqr_r = self.radius * self.radius;
        let direction = self.center - *origin;
        let cos_theta_max = f64::sqrt(1.0 - sqr_r / direction.sqr_length());
        let solid_angle = 2.0 * consts::PI * (1.0 - cos_theta_max);

        1.0/solid_angle
    }

    fn random(&self, origin: &V3) -> V3 {
        let norm = (*origin - self.center).unit();
        self.radius * rand_in_unit_hemisphere(&norm) + self.center - *origin
    }
}

impl Hittable for MovingSphere {
    fn hit(&self, ray_ctx: &RayCtx, dist_min: f64, dist_max: f64) -> Option<Hit> {
        let center = self.center(ray_ctx.time);
        let oc = ray_ctx.ray.origin - center;
        let a = ray_ctx.ray.direction.sqr_length();
        let b = oc.dot(ray_ctx.ray.direction);
        let c = oc.sqr_length() - (self.radius * self.radius) as f64;
        let discr_sqr = b * b - a * c;

        let get_hit = |ray_ctx: &RayCtx, dist: f64| -> Hit {
            let p = ray_ctx.ray.point_at(dist);
            let n = (p - center) / self.radius;
            let (u, v) = uv(n);
            return Hit::new(dist, p, n, self.material.borrow(), u, v);
        };

        if discr_sqr > 0.0 {
            let tmp = (b * b - a * c).sqrt();
            let x1 = (-b - tmp) / a;
            if (dist_min..dist_max).contains(&x1) {
                return Option::Some(get_hit(ray_ctx, x1));
            }
            let x2 = (-b + tmp) / a;
            if (dist_min..dist_max).contains(&x2) {
                return Option::Some(get_hit(ray_ctx, x2));
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

#[cfg(test)]
mod test {
    use crate::random::{rand_in_unit_sphere, next_std_f64, rand_in_unit_hemisphere};
    use crate::hittable::{Sphere, Hit, Hittable};
    use crate::material::{Lambertian, Material};
    use crate::vec::V3;
    use crate::texture::Color;
    use crate::ray::RayCtx;
    use crate::hittable::test::test_pdf_integration;

    #[test]
    fn test_pdf() {
        for _ in 0..100 {
            let count = 10_000;

            let center = 6.0 * rand_in_unit_sphere();
            let radius = 1.0 + next_std_f64();
            let sphere = Sphere::new(center, radius, Lambertian::new(Color(V3::ones())));

            test_pdf_integration(sphere, count);
        }
    }
}

