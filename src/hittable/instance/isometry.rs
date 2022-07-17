use nalgebra::Isometry3;

use crate::aabb::AABB;
use crate::hittable::{Bounded, Hit, Hittable, Important, Orientable, Positionable};
use crate::random2::DefaultRng;
use crate::ray::{Ray, RayCtx};
use crate::types::{Direction, Geometry, P3, Probability, Timespan};

pub trait IsometryOp<I, O> {
    fn apply(self, transform: Isometry3<Geometry>) -> O;
}

#[derive(Debug, Clone)]
pub struct IsometryT<T> {
    target: T,
    transform: Isometry3<Geometry>,
}

impl<I: Hittable> IsometryOp<I, IsometryT<I>> for I {
    fn apply(self, transform: Isometry3<Geometry>) -> IsometryT<I> {
        IsometryT {
            target: self,
            transform,
        }
    }
}

impl<I: Hittable + Orientable + Positionable> IsometryOp<I, I> for I {
    fn apply(self, transform: Isometry3<Geometry>) -> I {
        self
            .by_rotation_quat(&transform.rotation)
            .moved_by(&transform.translation.vector)
    }
}

impl<I: Hittable> Hittable for IsometryT<I> {
    fn hit(&self, ray_ctx: &RayCtx, dist_min: Geometry, dist_max: Geometry) -> Option<Hit> {
        let ray = ray_ctx.ray;
        let origin = self.transform.inverse_transform_point(&ray.origin);
        let direction: Direction = self.transform.inverse_transform_unit_vector(&ray.direction);
        let ray_ctx = RayCtx { ray: Ray { origin, direction }, ..*ray_ctx };

        self.target.hit(&ray_ctx, dist_min, dist_max)
            .map(|hit| {
                let point = self.transform * &hit.point;
                let normal = self.transform.rotation * &hit.normal;

                Hit {
                    point,
                    normal,
                    ..hit
                }
            })
    }
}

impl<I: Important> Important for IsometryT<I> {
    fn pdf_value(&self, origin: &P3, direction: &Direction, hit: &Hit) -> Probability {
        let origin = self.transform.inverse_transform_point(&origin);
        let direction = self.transform.inverse_transform_unit_vector(direction);
        let hit = Hit {
            point: origin,
            ..*hit
        };
        self.target.pdf_value(&origin, &direction, &hit)
    }

    fn random(&self, origin: &P3, rng: &mut DefaultRng) -> Direction {
        self.transform * &self.target.random(&self.transform.inverse_transform_point(&origin), rng)
    }
}


impl<B: Bounded> Bounded for IsometryT<B> {
    fn bounding_box(&self, timespan: Timespan) -> AABB {
        self.target.bounding_box(timespan)
            .by_rotation_quat(&self.transform.rotation)
            .moved_by(&self.transform.translation.vector)
    }
}
