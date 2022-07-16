use std::fmt::Debug;
use std::ops::Deref;
pub use aabox::*;
pub use aarect::*;
pub use constant_medium::*;
pub use instance::*;
pub use list::*;
pub use sphere::*;
pub use moving_sphere::*;
pub use no_hit::*;

use crate::aabb::AABB;
use crate::material::{Dielectric, Material};
use crate::ray::RayCtx;
use crate::types::{Direction, Geometry, P2, P3, Probability, Timespan, V3};
use crate::random::rand_in_unit_sphere;
use nalgebra::{AbstractRotation, Rotation3, Unit, UnitQuaternion};
use crate::Color;

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

impl<'a> Hit<'a> {
    pub fn new(dist: Geometry, p: P3, n: Direction, material: &'a dyn Material, uv: P2) -> Hit<'a> {
        return Hit { dist, point: p, normal: n, material, uv };
    }
}

#[allow(unused_variables)]
pub trait Hittable: Sync + Debug {
    fn hit(&self, ray: &RayCtx, dist_min: Geometry, dist_max: Geometry) -> Option<Hit>;

    #[deprecated]
    fn bounding_box(&self, timespan: Timespan) -> Option<AABB> { None }

    #[deprecated]
    fn pdf_value(&self, origin: &P3, direction: &Direction, hit: &Hit) -> Probability {
        1.0
    }

    #[deprecated]
    fn random(&self, origin: &P3) -> Direction {
        Direction::new_unchecked(V3::new(0.0, 1.0, 0.0))
    }

}

pub trait Bounded {
    fn bounding_box(&self, timespan: Timespan) -> AABB;

    fn debug_aabb(&self, color: Color) -> AABoxMono<Dielectric> {
        AABoxMono::from((Dielectric::new_colored(color, 1.0), self.bounding_box(0.0..1.0)))
    }
}

impl<T: Hittable> Bounded for T {
    fn bounding_box(&self, timespan: Timespan) -> AABB {
        Hittable::bounding_box(self, timespan).unwrap()
    }
}

pub trait Important {
    fn pdf_value(&self, origin: &P3, direction: &Direction, hit: &Hit) -> Probability {
        1.0
    }

    fn random(&self, origin: &P3) -> Direction {
        Direction::new_unchecked(V3::new(0.0, 1.0, 0.0))
    }
}

// todo: remove this blanket later
impl<T: Hittable> Important for T {
    fn pdf_value(&self, origin: &P3, direction: &Direction, hit: &Hit) -> Probability {
        Hittable::pdf_value(self, origin, direction, hit)
    }

    fn random(&self, origin: &P3) -> Direction {
        Hittable::random(self, origin)
    }
}



impl<H: Hittable + ?Sized + 'static, T: Deref<Target=H> + Sync + Debug + ?Sized> Hittable for T {
    fn hit(&self, ray: &RayCtx, dist_min: Geometry, dist_max: Geometry) -> Option<Hit> {
        (**self).hit(ray, dist_min, dist_max)
    }

    fn bounding_box(&self, timespan: Timespan) -> Option<AABB> {
        (**self).bounding_box(timespan)
    }

    fn pdf_value(&self, origin: &P3, direction: &Direction, hit: &Hit) -> Probability {
        (**self).pdf_value(origin, direction, hit)
    }

    fn random(&self, origin: &P3) -> Direction {
        (**self).random(origin)
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
    use nalgebra::Unit;
    use crate::{Color, Probability};
    use crate::hittable::Hittable;
    use crate::random::rand_in_unit_sphere;
    use crate::ray::RayCtx;
    use crate::consts::TAU;
    use crate::types::{Direction, Geometry};


    pub fn test_pdf_integration<T: Hittable>(hittable: T, count: usize) {
        let origin = 10.0 * rand_in_unit_sphere();
        let integral = (0..count).into_iter()
            .map(|_| {
                let dir = Direction::new_unchecked(rand_in_unit_sphere().coords);
                let ray = RayCtx::new(origin, dir, Color::zeros(), 1.0, 2);
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
