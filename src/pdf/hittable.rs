use crate::hittable::{Hit, Hittable, Important};
use crate::random2::DefaultRng;
use crate::ray::RayCtx;
use crate::types::{Direction, Geometry, P3, Probability};

use super::PDF;

#[derive(Debug)]
pub struct HittablePDF<'a, T> {
    origin: &'a P3,
    hittable: &'a T,
}

impl<'a, T> HittablePDF<'a, T> {
    pub fn new(origin: &'a P3, hittable: &'a T) -> Self {
        HittablePDF { origin, hittable }
    }
}

impl<T: Hittable + Important> PDF for HittablePDF<'_, T> {
    fn value(&self, direction: &Direction, hit: &Hit) -> Probability {
        let tmp_ray = RayCtx::new(hit.point, *direction, 0.0);
        if let Some(hit) = self.hittable.hit(&tmp_ray, 0.0001, Geometry::MAX) {
            Important::pdf_value(self.hittable, &self.origin, direction, &hit)
        } else {
            0.0
        }
    }

    fn generate(&self, rng: &mut DefaultRng) -> Direction {
        Important::random(self.hittable, &self.origin, rng)
    }
}
