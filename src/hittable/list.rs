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
        self.objects
            .iter()
            // todo[performance]: try enabling again after implementing heavier object
//            .filter(|h| h.bounding_box(ray.time, ray.time)
//                .map(|aabb| aabb.hit(ray, dist_min, dist_max))
//                .unwrap_or(true)
//            )
            .map(|h| h.hit(ray, dist_min, dist_max))
            .filter_map(std::convert::identity)
            .min_by(|s, o| s.dist.partial_cmp(&o.dist).unwrap())
    }

    fn bounding_box(&self, t_min: f32, t_max: f32) -> Option<AABB> {
        Some(self.aabb)
    }
}
