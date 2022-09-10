use super::*;

pub trait TranslateOp<I, O> {
    fn translate(self, offset: V3) -> O;
}

#[derive(Debug, Clone)]
pub struct Translate<T> {
    target: T,
    offset: V3,
}


impl<T: Hittable + Sized> TranslateOp<T, Translate<T>> for T {
    fn translate(self, offset: V3) -> Translate<T> {
        Translate {
            target: self,
            offset,
        }
    }
}

impl<T: Hittable + Sized> TranslateOp<T, Translate<T>> for Translate<T> {
    fn translate(self, offset: V3) -> Translate<T> {
        Translate {
            target: self.target,
            offset: self.offset + offset,
        }
    }
}

/*
impl<T: Hittable> TranslateOp<Rotate<T>, IsometryT<T>> for Rotate<T> {
    fn translate(self, offset: V3) -> IsometryT<T> {
        IsometryOp::apply(
            self.target,
            Isometry3::from_parts(
                offset.into(),
                self.transform.into(),
            ),
        )
    }
}
*/
impl<T: Hittable + Positionable> TranslateOp<T, T> for T {
    fn translate(self, offset: V3) -> T {
        self.moved_by(&offset)
    }
}


impl<T: Hittable> Hittable for Translate<T> {
    fn hit(&self, ray_ctx: &RayCtx, dist_min: Geometry, dist_max: Geometry) -> Option<Hit> {
        let moved_r = RayCtx {
            ray: Ray {
                origin: &ray_ctx.ray.origin - &self.offset,
                direction: ray_ctx.ray.direction,
            },
            ..*ray_ctx
        };
        self.target
            .hit(&moved_r, dist_min, dist_max)
            .map(|hit| Hit { point: hit.point + &self.offset, ..hit })
    }

}


impl<B: Bounded> Bounded for Translate<B>{
    fn bounding_box(&self, timespan: Timespan) -> AABB {
        self.target
            .bounding_box(timespan)
            .moved_by(&self.offset)
    }
}

impl<I: Important> Important for Translate<I> {
    fn pdf_value(&self, origin: &P3, direction: &Direction, hit: &Hit) -> Probability {
        self.target.pdf_value(&(*origin - self.offset), direction, &hit)
    }

    fn random(&self, origin: &P3, rng: &mut DefaultRng) -> Direction {
        self.target.random(&(*origin - self.offset), rng)
    }
}
