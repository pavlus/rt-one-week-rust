
use crate::vec::V3;

use super::{AABB, Hit, Hittable, Ray};

#[derive(Debug)]
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
    fn hit(&self, ray: &Ray, dist_min: f64, dist_max: f64) -> Option<Hit> {
        let mut selected: (f64, Option<Hit>) = (f64::MAX, None);
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

    fn bounding_box(&self, _: f32, _: f32) -> Option<AABB> {
        Some(self.aabb)
    }

    fn pdf_value(&self, origin: &V3, direction: &V3, hit: &Hit) -> f64 {
        let weight = 1.0/self.objects.len() as f64;
        self.objects
            .iter()
            .map(|o| weight * o.pdf_value(origin, direction, hit))
            .sum()
    }

    fn random(&self, origin: &V3) -> V3 {
        crate::random::random_item(&self.objects)
            .unwrap()
            .random(origin)
    }
}
