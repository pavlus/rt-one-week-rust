use std::fmt::Debug;

use super::{AABB, Hit, Hittable, Material, RayCtx, V3};
use crate::random::rand_in_unit_sphere;
use crate::types::{P3, Time, Geometry, Timespan, Scale, P2, Probability};
use crate::consts::{FRAC_PI_2, PI, TAU};
use nalgebra::Unit;

#[derive(Debug)]
pub struct Sphere<M> {
    pub center: P3,
    pub radius: Geometry,
    pub aabb: AABB,
    pub material: M,
}

impl<M: Clone + Debug> Clone for Sphere<M> {
    fn clone(&self) -> Self {
        Sphere {
            material: self.material.clone(),
            ..*self
        }
    }
}

impl<M: Material> Sphere<M> {
    pub fn new(center: P3, radius: Geometry, material: M) -> Sphere<M> {
        let radius_vec = V3::from_element(radius);
        let aabb = AABB::new((&center.coords - &radius_vec).into(), (&center.coords + &radius_vec).into());
        Sphere { center, radius, aabb, material }
    }
    #[inline]
    fn center(&self, _: Time) -> &P3 {
        &self.center
    }

    fn aabb(&self, _: Timespan) -> AABB {
        self.aabb
    }
}

#[derive(Debug)]
pub struct MovingSphere<M> {
    center_t0: P3,
    center_t1: P3,
    time0: Time,
    duration: Time,
    radius: Geometry,
    material: M,
}

impl<M :Material> MovingSphere<M> {
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

impl<M: Material> Hittable for Sphere<M> {
    fn hit(&self, ray_ctx: &RayCtx, dist_min: Geometry, dist_max: Geometry) -> Option<Hit> {
        let center = self.center(ray_ctx.time);
        let radius = self.radius;
        let oc = &ray_ctx.ray.origin - center;
        let b = oc.dot(&ray_ctx.ray.direction);
        let c = oc.norm_squared() - (radius * radius);
        let discr_sqr = b * b - c;

        if discr_sqr < 0.0 { return None; }

        let tmp = discr_sqr.sqrt();
        let x1 = -b - tmp;
        let x2 = -b + tmp;
        let x = if dist_min <= x1 && x1 <= dist_max {
            Option::Some(x1)
        } else if dist_min <= x2 && x2 <= dist_max {
            Option::Some(x2)
        } else { Option::None };

        if let Some(dist) = x {
            let p = ray_ctx.ray.point_at(dist);
            let n = Unit::new_unchecked((&p - center) / radius);
            let uv = uv(&n);
            Some(Hit::new(dist, p, n, &self.material, uv))
        } else { None }
    }

    fn bounding_box(&self, timespan: Timespan) -> Option<AABB> {
        Some(self.aabb(timespan))
    }

    fn pdf_value(&self, origin: &P3, _direction: &Unit<V3>, _hit: &Hit) -> Probability {
        let sqr_r = self.radius * self.radius;
        let direction = &self.center - origin;
        let squared = 1.0 - sqr_r / direction.norm_squared();
        if squared < 0.0 {
            return 1.0 / TAU;
        }
        let cos_theta_max = Geometry::sqrt(squared);
        let solid_angle = TAU * (1.0 - cos_theta_max);

        if false && cfg!(test) {
            eprintln!("sqr_r: {sqr_r}");
            eprintln!("direction: {:?}", direction);
            eprintln!("cos_theta_max: {cos_theta_max}");
            eprintln!("solid_angle: {solid_angle}");
        }

        (1.0 / solid_angle) as Probability
    }

    fn random(&self, origin: &P3) -> Unit<V3> {
        Unit::new_normalize(self.radius * rand_in_unit_sphere().coords + (&self.center - origin))
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
            let uv = uv(&n);
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

    fn bounding_box(&self, timespan: Timespan) -> Option<AABB> {
        Some(self.aabb(timespan.start) + self.aabb(timespan.end))
    }
}


fn uv(unit_point: &Unit<V3>) -> P2 {
    let phi = Geometry::atan2(unit_point.z, unit_point.x);
    let theta = unit_point.y.asin();

    let u = 1.0 - (phi + PI) / TAU;
    let v = (theta + FRAC_PI_2) / PI;
    P2::new(u, v)
}

#[cfg(test)]
mod test {
    use crate::random::next_std_f64;
    use crate::hittable::Sphere;
    use crate::material::NoMat;
    use crate::hittable::test::test_pdf_integration;
    use crate::types::P3;

    #[test]
    fn test_pdf() {
        let count = 10_000;
        // let count = 10;

        let radius = 3.0 * (1.0 + next_std_f64());
        let sphere = Sphere::new(P3::default(), radius, NoMat);

        test_pdf_integration(sphere, count);
    }
}

