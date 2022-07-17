use nalgebra::{Rotation3, UnitQuaternion};
use rand::prelude::Distribution;
use rand_distr::UnitSphere;

use crate::hittable::{Hit, Hittable, Important, Orientable, Positionable, Scalable};
use crate::ray::RayCtx;
use crate::types::{Direction, Geometry, P3, Probability};
use crate::V3;
use crate::random2::DefaultRng;

#[derive(Debug)]
pub struct NoHit;

impl Hittable for NoHit {
    fn hit(&self, _ray: &RayCtx, _dist_min: Geometry, _dist_max: Geometry) -> Option<Hit> {
        None
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

impl Important for NoHit {
    fn pdf_value(&self, _origin: &P3, _direction: &Direction, _hit: &Hit) -> Probability {
        0.0
    }

    fn random(&self, _origin: &P3, rng: &mut DefaultRng) -> Direction {
        Direction::new_unchecked(UnitSphere.sample(rng).into())
    }
}
