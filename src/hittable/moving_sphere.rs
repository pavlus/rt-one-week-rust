use std::fmt::Debug;

use nalgebra::Unit;

use crate::hittable::Bounded;
use crate::types::{Geometry, P3, Scale, Time, Timespan};

use super::{AABB, Hit, Hittable, Material, RayCtx, V3};

#[derive(Debug)]
pub struct MovingSphere<M> {
    center_t0: P3,
    center_t1: P3,
    time0: Time,
    duration: Time,
    radius: Geometry,
    material: M,
}

impl<M> MovingSphere<M> {
    pub fn new(center_t0: P3, center_t1: P3, timespan: Timespan, radius: Geometry, material: M) -> Self {
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
    fn radius(&self) -> Geometry { self.radius }
    fn aabb(&self, t: Time) -> AABB {
        let r3 = V3::from_element(self.radius());
        AABB::new((self.center(t) - r3).into(), (self.center(t) + r3).into())
    }
}

impl<M: Material> Hittable for MovingSphere<M> {
    fn hit(&self, ray_ctx: &RayCtx, dist_min: Geometry, dist_max: Geometry) -> Option<Hit> {
        let center = self.center(ray_ctx.time);
        let oc = &ray_ctx.ray.origin - center;
        let a = ray_ctx.ray.direction.norm_squared();
        let b = oc.dot(&ray_ctx.ray.direction);
        let c = oc.norm_squared() - (self.radius * self.radius);
        let discr_sqr = b * b - a * c;

        let get_hit = |ray_ctx: &RayCtx, dist: Geometry| -> Hit {
            let p = ray_ctx.ray.point_at(dist);
            let n = Unit::new_unchecked((p - center) / self.radius);
            let uv = super::uv(&n);
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
}

impl<M> Bounded for MovingSphere<M> {
    fn bounding_box(&self, timespan: Timespan) -> AABB {
        self.aabb(timespan.start) + self.aabb(timespan.end)
    }
}


#[cfg(test)]
mod test {
    use crate::hittable::Sphere;
    use crate::hittable::test::test_pdf_integration;
    use crate::material::NoMat;
    use crate::random::next_std_f64;

    #[test]
    fn test_pdf() {
        let count = 10_000;
        // let count = 10;

        let radius = 3.0 * (1.0 + next_std_f64());
        let sphere = Sphere::radius(radius, NoMat);

        test_pdf_integration(sphere, count);
    }
}

