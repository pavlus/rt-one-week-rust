use std::fmt::Debug;
pub use aabox::*;
pub use aarect::*;
pub use constant_medium::*;
pub use instance::*;
pub use list::*;
pub use sphere::*;

use crate::aabb::AABB;
use crate::material::Material;
use crate::ray::RayCtx;
use crate::types::{V3, P3, Distance, Time, P2, Probability};
use crate::random::rand_in_unit_sphere;
use nalgebra::Unit;

mod sphere;
mod aarect;
mod list;
mod aabox;
mod constant_medium;
mod instance;

#[derive(Copy, Clone)]
pub struct Hit<'a> {
    pub point: P3,
    pub normal: Unit<V3>,
    pub dist: Distance,
    // material data:
    pub uv: P2,
    pub material: &'a dyn Material,
}

impl<'a> Hit<'a> {
    pub fn new(dist: Distance, p: P3, n: Unit<V3>, material: &'a dyn Material, uv: P2) -> Hit<'a> {
        return Hit { dist, point: p, normal: n, material, uv, traversal_cnt: 0 };
    }
}

#[allow(unused_variables)]
pub trait Hittable: Sync + Debug {
    fn hit(&self, ray: &RayCtx, dist_min: Distance, dist_max: Distance) -> Option<Hit>;
    fn bounding_box(&self, t_min: Time, t_max: Time) -> Option<AABB> { None }

    fn pdf_value(&self, origin: &P3, direction: &Unit<V3>, hit: &Hit) -> Probability {
        1.0
    }

    fn random(&self, origin: &P3) -> Unit<V3> {
        Unit::new_unchecked(V3::new(0.0, 1.0, 0.0))
    }

}

impl<T:Hittable + ?Sized> Hittable for Box<T>{
    fn hit(&self, ray: &RayCtx, dist_min: Distance, dist_max: Distance) -> Option<Hit> {
        Hittable::hit(&**self, ray, dist_min, dist_max)
    }

    fn bounding_box(&self, t_min: Time, t_max: Time) -> Option<AABB> {
        Hittable::bounding_box(&**self, t_min, t_max)
    }

    fn pdf_value(&self, origin: &P3, direction: &Unit<V3>, hit: &Hit) -> Probability {
        Hittable::pdf_value(&**self, origin, direction, hit)
    }

    fn random(&self, origin: &P3) -> Unit<V3> {
        Hittable::random(&**self, origin)
    }
}

#[derive(Debug)]
pub struct NoHit;
impl Hittable for NoHit{
    fn hit(&self, _ray: &RayCtx, _dist_min: Distance, _dist_max: Distance) -> Option<Hit> {
        None
    }

    fn bounding_box(&self, _t_min: Time, _t_max: Time) -> Option<AABB> {
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
    use crate::hittable::Hittable;
    use crate::random::rand_in_unit_sphere;
    use crate::ray::RayCtx;
    use crate::types::V3;
    use crate::consts::TAU;

    pub fn test_pdf_integration<T: Hittable>(hittable: T, count: usize) {
        let origin = rand_in_unit_sphere();
        let integral = (0..count).into_iter()
            .map(|_| {
                let dir = Unit::new_unchecked(rand_in_unit_sphere().coords);
                let ray = RayCtx::new(origin, dir, V3::from_element(1.0), 1.0, 2);
                if let Some(hit) = hittable.hit(&ray, -99999.0, 99999.0) {
                    hittable.pdf_value(&origin, &dir, &hit)
                } else { 0.0 }
            }).sum::<f64>() / (count as f64);
        let expected = 1.0 / TAU;
        let epsilon = 1.0 / f64::cbrt(count as f64);
        assert!(
            f64::abs(integral - expected) < epsilon,
            format!("Expected: {}, actual: {}, epsilon: {}", expected, integral, epsilon)
        );
    }
}
