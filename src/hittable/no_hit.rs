use nalgebra::{Rotation3, Unit, UnitQuaternion};
use crate::aabb::AABB;
use crate::hittable::{Hit, Hittable, MovingSphere, Orientable, Positionable, Scalable};
use crate::random::rand_in_unit_sphere;
use crate::ray::RayCtx;
use crate::types::{Direction, Geometry, P3, Probability, Timespan};
use crate::V3;

#[derive(Debug)]
pub struct NoHit;

impl Hittable for NoHit {
    fn hit(&self, _ray: &RayCtx, _dist_min: Geometry, _dist_max: Geometry) -> Option<Hit> {
        None
    }

    fn bounding_box(&self, _: Timespan) -> Option<AABB> {
        None
    }

    fn pdf_value(&self, origin: &P3, direction: &Direction, hit: &Hit) -> Probability {
        1.0
    }

    fn random(&self, origin: &P3) -> Direction {
        Unit::new_unchecked(rand_in_unit_sphere().coords)
    }
}

impl Orientable for NoHit {
    fn by_axis_angle(self, _axis: &Direction, _degrees: Geometry) -> Self { self }

    fn by_scaled_axis(self, _scaled_axis: V3) -> Self { self }

    fn by_rotation_quat(self, _rotation: &UnitQuaternion<Geometry>) -> Self { self }

    fn by_rotation(self, _rotation: &Rotation3<Geometry>) -> Self { self }
}

impl Positionable for NoHit {
    fn move_by(&mut self, _offset: &V3) {}

    fn moved_by(self, _offset: &V3) -> Self { self }
}

impl Scalable for NoHit {
    fn scale(self, _factor: Geometry) -> Self { self }
}
