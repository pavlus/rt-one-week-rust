use std::fmt::Debug;
use std::sync::atomic::{AtomicUsize, Ordering};
use crossbeam::atomic::AtomicConsume;

use nalgebra::{Rotation3, UnitQuaternion};

pub use aabox::*;
pub use aarect::*;
pub use constant_medium::*;
pub use instance::*;
pub use list::*;
pub use moving_sphere::*;
pub use no_hit::*;
pub use sphere::*;

use crate::aabb::AABB;
use crate::Color;
use crate::material::{Dielectric, Material};
use crate::random2::DefaultRng;
use crate::ray::RayCtx;
use crate::types::{Direction, Geometry, P2, P3, Probability, Timespan, V3};

mod sphere;
mod aarect;
mod list;
mod aabox;
mod constant_medium;
mod instance;
mod no_hit;
mod moving_sphere;

#[derive(Copy, Clone, Debug)]
pub struct Hit<'a> {
    pub point: P3,
    pub normal: Direction,
    pub dist: Geometry,
    // material data:
    pub uv: P2,
    pub material: &'a dyn Material,
}

#[allow(non_upper_case_globals)]
#[cfg(feature = "metrics")]
static hit_cnt: AtomicUsize = AtomicUsize::new(0);
#[cfg(feature = "metrics")]
pub fn total_hits() -> usize{
    hit_cnt.load_consume()
}

impl<'a> Hit<'a> {
    pub fn new(dist: Geometry, p: P3, n: Direction, material: &'a dyn Material, uv: P2) -> Hit<'a> {
        #[cfg(feature = "metrics")]
        hit_cnt.fetch_add(1, Ordering::Relaxed);
        return Hit { dist, point: p, normal: n, material, uv };
    }
}

pub trait Hittable: Send + Sync + Debug {
    fn hit(&self, ray: &RayCtx, dist_min: Geometry, dist_max: Geometry) -> Option<Hit>;
}

pub trait Bounded {
    fn bounding_box(&self, timespan: Timespan) -> AABB;

    fn debug_aabb(&self, color: Color) -> AABoxMono<Dielectric> {
        AABoxMono::from((Dielectric::new_colored(color, 1.0), self.bounding_box(0.0..1.0)))
    }
}

pub trait Important: Send + Sync {
    fn pdf_value(&self, _origin: &P3, _direction: &Direction, _hit: &Hit) -> Probability;

    fn random(&self, origin: &P3, rng: &mut DefaultRng) -> Direction;
}

pub trait ImportantHittable: Important + Hittable {}

impl<H: Important + Hittable> ImportantHittable for H {}


impl<H: Hittable + ?Sized> Hittable for Box<H> {
    fn hit(&self, ray: &RayCtx, dist_min: Geometry, dist_max: Geometry) -> Option<Hit> {
        (**self).hit(ray, dist_min, dist_max)
    }
}

impl<H: Hittable + ?Sized> Hittable for &H {
    fn hit(&self, ray: &RayCtx, dist_min: Geometry, dist_max: Geometry) -> Option<Hit> {
        (**self).hit(ray, dist_min, dist_max)
    }
}

impl<B: Bounded + ?Sized> Bounded for Box<B> {
    fn bounding_box(&self, timespan: Timespan) -> AABB {
        (**self).bounding_box(timespan)
    }
}

impl<I: Important + ?Sized> Important for Box<I> {
    fn pdf_value(&self, origin: &P3, direction: &Direction, hit: &Hit) -> Probability {
        (**self).pdf_value(origin, direction, hit)
    }

    fn random(&self, origin: &P3, rng: &mut DefaultRng) -> Direction {
        (**self).random(origin, rng)
    }
}


pub trait Positionable: Sized {
    fn move_by(&mut self, offset: &V3);
    fn moved_by(self, offset: &V3) -> Self;
}

pub trait Orientable: Sized {
    fn by_axis_angle(self, axis: &Direction, degrees: Geometry) -> Self {
        self.by_scaled_axis(axis.scale(degrees.to_radians()))
    }
    fn by_scaled_axis(self, scaled_axis: V3) -> Self {
        self.by_rotation(&Rotation3::from_scaled_axis(scaled_axis))
    }
    fn by_rotation_quat(self, rotation: &UnitQuaternion<Geometry>) -> Self {
        self.by_rotation(&rotation.to_rotation_matrix())
    }
    fn by_rotation(self, rotation: &Rotation3<Geometry>) -> Self;
}

pub trait Scalable: Sized {
    fn scale(self, factor: Geometry) -> Self;
}


#[cfg(test)]
mod test {
    use crate::Probability;
    use crate::consts::TAU;
    use crate::hittable::{Hittable, Important};
    use crate::random::rand_in_unit_sphere;
    use crate::ray::RayCtx;
    use crate::types::{Direction, Geometry};

    pub fn test_pdf_integration<T: Hittable + Important>(hittable: T, count: usize) {
        let origin = 10.0 * rand_in_unit_sphere();
        let integral = (0..count).into_iter()
            .map(|_| {
                let dir = Direction::new_unchecked(rand_in_unit_sphere().coords);
                let ray = RayCtx::new(origin, dir, 1.0);
                if let Some(hit) = hittable.hit(&ray, -99999.0, 99999.0) {
                    hittable.pdf_value(&origin, &dir, &hit)
                } else { 0.0 }
            }).sum::<Probability>() / (count as Probability);
        let expected = 1.0 / TAU as Probability;
        let epsilon = 1.0 / Probability::cbrt(count as Probability);
        let diff = Probability::abs(integral - expected);
        assert_ne!(integral, 0.0, "Looks like there were no hits!");
        assert!(
            diff < epsilon,
            "Expected: {}, actual: {}, epsilon: {}, difference: {}", expected, integral, epsilon, diff
        );
    }
}
