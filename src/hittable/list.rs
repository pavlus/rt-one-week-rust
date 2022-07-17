use itertools::Itertools;
use rand::prelude::IteratorRandom;

use crate::hittable::{Bounded, Important};
use crate::random2::DefaultRng;
use crate::types::{Direction, Geometry, P3, Probability, Timespan};

use super::{AABB, Hit, Hittable, RayCtx};


impl<T: Hittable> Hittable for Vec<T> {
    fn hit(&self, ray: &RayCtx, dist_min: Geometry, dist_max: Geometry) -> Option<Hit> {
        let mut selected: (Geometry, Option<Hit>) = (dist_max, None);

        for o in self {
            if let Some(hit) = o.hit(ray, dist_min, selected.0) {
                if hit.dist < selected.0 {
                    selected = (hit.dist, Some(hit))
                }
            }
        }
        selected.1
    }

}


impl<T: Bounded> Bounded for Vec<T> {
    fn bounding_box(&self, timespan: Timespan) -> AABB {
        self.iter()
            .map(|o| o.bounding_box(timespan.clone()))
            .sum1().unwrap()
    }
}


impl<I: Important> Important for Vec<I> {
    fn pdf_value(&self, origin: &P3, direction: &Direction, hit: &Hit) -> Probability {
        self.iter()
            .map(|o| o.pdf_value(origin, direction, hit))
            .sum::<Probability>() / self.len() as Probability
    }

    fn random(&self, origin: &P3, rng: &mut DefaultRng) -> Direction {
        self.iter()
            .choose(rng)
            .unwrap()
            .random(origin, rng)
    }
}
