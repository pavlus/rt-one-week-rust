use std::fmt::Debug;
use std::ops::Bound;

use super::{AABB, Hit, Hittable, Material, RayCtx, V3};
use crate::random::rand_in_unit_sphere;
use crate::types::{P3, Time, Geometry, Timespan, Scale, P2, Probability, Direction};
use crate::consts::{FRAC_PI_2, PI, TAU};
use nalgebra::Unit;
use crate::hittable::{Bounded, Positionable, Scalable};
use crate::material::NoMat;

#[derive(Debug, Clone)]
pub struct Sphere<M> {
    pub center: P3,
    pub radius: Geometry,
    pub aabb: AABB,
    pub material: M,
}

impl<M: Material> Sphere<M> {
    pub fn new(center: P3, radius: Geometry, material: M) -> Self {
        let radius_vec = V3::from_element(radius);
        let aabb = AABB::new((&center.coords - &radius_vec).into(), (&center.coords + &radius_vec).into());
        Sphere { center, radius, aabb, material }
    }

    pub fn radius(radius: Geometry, material: M) -> Self {
        Sphere::new(P3::default(), radius, material)
    }

    pub fn unit(material: M) -> Self {
        Sphere::radius(1.0, material)
    }

    #[inline]
    fn center(&self, _: Time) -> &P3 {
        &self.center
    }

    fn aabb(&self, _: Timespan) -> AABB {
        self.aabb
    }
}


impl<M: Material> Positionable for Sphere<M> {
    fn move_by(&mut self, offset: &V3) {
        self.center += offset;
        self.aabb.move_by(offset);
    }

    fn moved_by(self, offset: &V3) -> Self {
        Sphere{
            center: self.center + offset,
            aabb: self.aabb.moved_by(offset),
            ..self
        }
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
            Some(x1)
        } else if dist_min <= x2 && x2 <= dist_max {
            Some(x2)
        } else { None };

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

    fn pdf_value(&self, origin: &P3, direction: &Direction, hit: &Hit) -> Probability {
        let sqr_r = self.radius * self.radius;
        let direction = &self.center - origin;
        let squared = 1.0 - sqr_r / direction.norm_squared();
        if squared < 0.0 {  // in case origin is inside the sphere
            return 1.0 / TAU as Probability;
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

    fn random(&self, origin: &P3) -> Direction {
        Unit::new_normalize(self.radius * rand_in_unit_sphere().coords + (&self.center - origin))
    }
}


pub(super) fn uv(unit_point: &Unit<V3>) -> P2 {
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

