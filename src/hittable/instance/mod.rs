use nalgebra::Rotation3;

use crate::hittable::{Bounded, Important, Orientable, Positionable};
use crate::random2::DefaultRng;
use crate::ray::Ray;
use crate::types::{Direction, Geometry, P3, Probability, Timespan};

use super::{AABB, Hit, Hittable, RayCtx, V3};

pub use self::isometry::*;
pub use self::rotation::*;
pub use self::translation::*;

mod isometry;
mod rotation;
mod translation;

pub trait FlipNormalsOp<I, O> {
    fn flip_normals(self) -> O;
}


#[derive(Debug, Clone)]
pub struct FlipNormals<T>(T);

impl<I: Hittable + Sized> FlipNormalsOp<I, FlipNormals<I>> for I {
    fn flip_normals(self) -> FlipNormals<I> {
        FlipNormals(self)
    }
}

impl<I: Hittable> FlipNormalsOp<FlipNormals<I>, I> for FlipNormals<I> {
    fn flip_normals(self) -> I {
        self.0
    }
}

impl<T: Hittable> Hittable for FlipNormals<T> {
    fn hit(&self, ray: &RayCtx, dist_min: Geometry, dist_max: Geometry) -> Option<Hit> {
        self.0
            .hit(ray, dist_min, dist_max)
            .map(|hit| Hit { normal: -hit.normal, ..hit })
    }
}


impl<B: Bounded> Bounded for FlipNormals<B>{
    fn bounding_box(&self, timespan: Timespan) -> AABB {
        self.0.bounding_box(timespan)
    }
}

impl<I: Important> Important for FlipNormals<I>{
    fn pdf_value(&self, origin: &P3, direction: &Direction, hit: &Hit) -> Probability {
        self.0.pdf_value(origin, direction, hit)
    }

    fn random(&self, origin: &P3, rng: &mut DefaultRng) -> Direction {
        self.0.random(origin, rng)
    }
}
