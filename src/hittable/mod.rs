use std::fmt::Debug;
use std::ops::Deref;
pub use aabox::*;
pub use aarect::*;
pub use constant_medium::*;
pub use instance::*;
pub use list::*;
pub use sphere::*;

use crate::aabb::AABB;
use crate::material::Material;
use crate::ray::RayCtx;
use crate::types::{V3, P3, Geometry, P2, Probability, Timespan};
use crate::random::rand_in_unit_sphere;
use nalgebra::Unit;

mod sphere;
mod aarect;
mod list;
mod aabox;
mod constant_medium;
mod instance;

#[derive(Copy, Clone, Debug)]
pub struct Hit<'a> {
    pub point: P3,
    pub normal: Unit<V3>,
    pub dist: Geometry,
    // material data:
    pub uv: P2,
    pub material: &'a dyn Material,
}

impl<'a> Hit<'a> {
    pub fn new(dist: Geometry, p: P3, n: Unit<V3>, material: &'a dyn Material, uv: P2) -> Hit<'a> {
        return Hit { dist, point: p, normal: n, material, uv };
    }
}

#[allow(unused_variables)]
pub trait Hittable: Sync + Debug {
    fn hit(&self, ray: &RayCtx, dist_min: Geometry, dist_max: Geometry) -> Option<Hit>;
    fn bounding_box(&self, timespan: Timespan) -> Option<AABB> { None }

    fn pdf_value(&self, origin: &P3, direction: &Unit<V3>, hit: &Hit) -> Probability {
        1.0
    }

    fn random(&self, origin: &P3) -> Unit<V3> {
        Unit::new_unchecked(V3::new(0.0, 1.0, 0.0))
    }
}

impl<H: Hittable + ?Sized + 'static, T: Deref<Target=H> + Sync + Debug + ?Sized> Hittable for T {
    fn hit(&self, ray: &RayCtx, dist_min: Geometry, dist_max: Geometry) -> Option<Hit> {
        (**self).hit(ray, dist_min, dist_max)
    }

    fn bounding_box(&self, timespan: Timespan) -> Option<AABB> {
        (**self).bounding_box(timespan)
    }

    fn pdf_value(&self, origin: &P3, direction: &Unit<V3>, hit: &Hit) -> Probability {
        (**self).pdf_value(origin, direction, hit)
    }

    fn random(&self, origin: &P3) -> Unit<V3> {
        (**self).random(origin)
    }
}

#[derive(Debug)]
pub struct NoHit;

impl Hittable for NoHit {
    fn hit(&self, _ray: &RayCtx, _dist_min: Geometry, _dist_max: Geometry) -> Option<Hit> {
        None
    }

    fn bounding_box(&self, _: Timespan) -> Option<AABB> {
        None
    }

    fn pdf_value(&self, _origin: &P3, _direction: &Unit<V3>, _hit: &Hit) -> Probability {
        1.0
    }

    fn random(&self, _origin: &P3) -> Unit<V3> {
        Unit::new_unchecked(rand_in_unit_sphere().coords)
    }
}

#[cfg(test)]
mod test {
    use nalgebra::Unit;
    use crate::Color;
    use crate::hittable::Hittable;
    use crate::random::rand_in_unit_sphere;
    use crate::ray::RayCtx;
    use crate::consts::TAU;


    pub fn test_pdf_integration<T: Hittable>(hittable: T, count: usize) {
        let origin = 10.0 * rand_in_unit_sphere();
        let integral = (0..count).into_iter()
            .map(|_| {
                let dir = Unit::new_unchecked(rand_in_unit_sphere().coords);
                let ray = RayCtx::new(origin, dir, Color::zeros(), 1.0, 2);
                if let Some(hit) = hittable.hit(&ray, -99999.0, 99999.0) {
                    hittable.pdf_value(&origin, &dir, &hit)
                } else { 0.0 }
            }).sum::<f64>() / (count as f64);
        let expected = 1.0 / TAU;
        let epsilon = 1.0 / f64::cbrt(count as f64);
        let diff = f64::abs(integral - expected);
        assert_ne!(integral, 0.0, "Looks like there were no hits!");
        assert!(
            diff < epsilon,
            "Expected: {}, actual: {}, epsilon: {}, difference: {}", expected, integral, epsilon, diff
        );
    }
}
