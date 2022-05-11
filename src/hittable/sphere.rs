use std::borrow::Borrow;
use std::fmt::Debug;

use super::{AABB, Hit, Hittable, Material, RayCtx, V3};
use crate::random::rand_in_unit_sphere;
use crate::types::{P3, Time, Distance, Timespan, Angle, Scale, P2, Probability};
use crate::consts::{FRAC_PI_2, PI, TAU};
use nalgebra::Unit;

#[derive(Debug)]
pub struct Sphere<M> {
    pub center: P3,
    pub radius: Distance,
    pub material: M,
}
impl<M: Clone + Debug> Clone for Sphere<M>{
    fn clone(&self) -> Self {
        Sphere{
            material: self.material.clone(),
            ..*self
        }
    }
}

impl<M:Material> Sphere<M> {
    pub fn new(center: P3, radius: Distance, material: M) -> Sphere<M> {
        Sphere { center, radius, material }
    }
    #[inline]
    fn center(&self, _: Time) -> &P3 {
        &self.center
    }

    fn aabb(&self, _: Timespan) -> AABB {
        let radius_vec = V3::from_element(self.radius);
        AABB::new((&self.center.coords - radius_vec).into(), (&self.center.coords + radius_vec).into())
    }

}
impl<M> Borrow<M> for Sphere<M>{
    fn borrow(&self) -> &M {
        &self.material
    }
}
#[derive(Debug)]
pub struct MovingSphere {
    center_t0: P3,
    center_t1: P3,
    time0: Time,
    duration: Time,
    radius: Distance,
    material: Box<dyn Material>,
}
impl MovingSphere {
    pub fn new(center_t0: P3, center_t1: P3, timespan: Timespan, radius: Distance, material: Box<dyn Material>) -> MovingSphere {
        MovingSphere {
            center_t0,
            center_t1,
            time0: timespan.start,
            duration: timespan.end - timespan.start,
            radius,
            material,
        }
    }
    #[inline]
    fn center(&self, time: Time) -> P3 {
        let scale = ((time - self.time0) / self.duration) as Scale;
        (&self.center_t0 + scale * (&self.center_t1 - &self.center_t0)).into()
    }
    #[inline]
    fn radius(&self) -> Distance { self.radius }
    fn aabb(&self, t: Time) -> AABB {
        let r3 = V3::from_element(self.radius());
        AABB::new((self.center(t) - r3).into(), (self.center(t) + r3).into())
    }
}

impl<M: Material> Hittable for Sphere<M> {
    fn hit(&self, ray_ctx: &RayCtx, dist_min: Distance, dist_max: Distance) -> Option<Hit> {
        let center =self.center(ray_ctx.time);
        let radius = self.radius;
        let oc = ray_ctx.ray.origin - center;
        let a = ray_ctx.ray.direction.norm_squared();
        let b = oc.dot(&ray_ctx.ray.direction);
        let c = oc.norm_squared() - (radius * radius);
        let discr_sqr = b * b - a * c;

        let get_hit = |ray_ctx: &RayCtx, dist: Distance| -> Hit {
            let p = ray_ctx.ray.point_at(dist);
            let n = Unit::new_unchecked((p - center) / radius);
            let uv = uv(n);
            return Hit::new(dist, p, n, &self.material, uv);
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

    fn bounding_box(&self, t_min: Time, t_max: Time) -> Option<AABB> {
        Some(self.aabb(t_min..t_max))
    }

    fn pdf_value(&self, origin: &P3, _direction: &Unit<V3>, _hit: &Hit) -> Probability {
        let sqr_r = self.radius * self.radius;
        let direction = &self.center - origin;
        let cos_theta_max = Distance::sqrt(1.0 - sqr_r / direction.norm_squared());
        let solid_angle = TAU * (1.0 - cos_theta_max);

        (1.0/solid_angle) as Probability
    }

    fn random(&self, origin: &P3) -> Unit<V3> {
        let norm = (origin - &self.center).normalize();
        Unit::new_normalize(self.radius * rand_in_unit_hemisphere(&norm).coords + (&self.center - origin))
    }
}

impl Hittable for MovingSphere {
    fn hit(&self, ray_ctx: &RayCtx, dist_min: Distance, dist_max: Distance) -> Option<Hit> {
        let center = self.center(ray_ctx.time);
        let oc = &ray_ctx.ray.origin - center;
        let a = ray_ctx.ray.direction.norm_squared();
        let b = oc.dot(&ray_ctx.ray.direction);
        let c = oc.norm_squared() - (self.radius * self.radius);
        let discr_sqr = b * b - a * c;

        let get_hit = |ray_ctx: &RayCtx, dist: Distance| -> Hit {
            let p = ray_ctx.ray.point_at(dist);
            let n = Unit::new_unchecked((p - center) / self.radius);
            let uv = uv(n);
            return Hit::new(dist, p, n, self.material.borrow(), uv);
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

    fn bounding_box(&self, t_min: Time, t_max: Time) -> Option<AABB> {
        Some(self.aabb(t_min) + self.aabb(t_max))
    }

}

fn uv(unit_point: Unit<V3>) -> P2 {
    let phi = Angle::atan2(unit_point.z, unit_point.x);
    let theta = unit_point.y.asin();

    let u = 1.0 - (phi + PI) / TAU;
    let v = (theta + FRAC_PI_2) / PI;
    P2::new(u, v)
}

#[cfg(test)]
mod test {
    use crate::random::{rand_in_unit_sphere, next_std_f64};
    use crate::hittable::Sphere;
    use crate::material::Lambertian;
    use crate::types::Color;
    use crate::hittable::test::test_pdf_integration;

    #[test]
    fn test_pdf() {
        for _ in 0..100 {
            let count = 10_000;

            let center = 6.0 * rand_in_unit_sphere();
            let radius = 1.0 + next_std_f64();
            let sphere = Sphere::new(center, radius, Lambertian::<Color>::new(Color::from_element(1.0)));

            test_pdf_integration(sphere, count);
        }
    }
}

