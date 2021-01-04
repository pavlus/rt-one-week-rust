
use crate::types::{V3, P3, Distance, Time, Color};

use super::{AABB, Hit, Hittable, RayCtx};
use nalgebra::Unit;

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
    aabb: AABB,
}

impl HittableList {
    pub fn new(objects: Vec<Box<dyn Hittable>>) -> HittableList {
        let aabb = (|| {
            let mut aabbs = objects.iter().flat_map(|o| o.bounding_box(0.0, 1.0));
            let first = aabbs.next()?;
            Some(aabbs.fold(first, |a, b| a + b))
        })();
        HittableList { objects, aabb: aabb.unwrap() }
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &RayCtx, dist_min: Distance, dist_max: Distance) -> Option<Hit> {
        let mut selected: (Distance, Option<Hit>) = (Distance::MAX, None);
        for o in &self.objects {
            if let Some(hit) = o.hit(ray, dist_min, dist_max){
                if hit.dist < selected.0 {
                    selected = (hit.dist, Some(hit))
                }
            }
        }
        selected.1
        /*self.objects
            .iter()
            // todo[performance]: try enabling again after implementing heavier object
//            .filter(|h| h.bounding_box(ray.time, ray.time)
//                .map(|aabb| aabb.hit(ray, dist_min, dist_max))
//                .unwrap_or(true)
//            )
            .map(|h| h.hit(ray, dist_min, dist_max))
            .filter_map(std::convert::identity)
            .min_by(|s, o| s.dist.partial_cmp(&o.dist).unwrap())*/
    }

    fn bounding_box(&self, _: Time, _: Time) -> Option<AABB> {
        Some(self.aabb)
    }

    fn pdf_value(&self, origin: &P3, direction: &Unit<V3>, hit: &Hit) -> f64 {
        self.objects
            .iter()
            .map(|o| o.pdf_value(origin, direction, hit))
            .sum::<f64>() / self.objects.len() as f64
    }

    fn random(&self, origin: &P3) -> Unit<V3> {
        crate::random::random_item(&self.objects)
            .unwrap()
            .random(origin)
    }
}
