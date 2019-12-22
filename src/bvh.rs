use crate::aabb::AABB;
use crate::Hittable;
use crate::hittable::{Hit, HittableList};
use crate::ray::Ray;
use crate::random::random_axis;

#[derive(Debug)]
pub struct BVH {
    left: Box<dyn Hittable>,
    right: Box<dyn Hittable>,
    aabb: Option<AABB>,
}

impl BVH {
    pub(crate) fn new(objs: Vec<Box<dyn Hittable>>) -> Box<dyn Hittable> {
        BVH::construct(objs)
    }
    fn construct(mut objs: Vec<Box<dyn Hittable>>) -> Box<dyn Hittable> {
        if objs.len() == 1 { return objs.remove(0); }
        if objs.len() <= 8 { return Box::new(HittableList::new(objs)); }
        let axis = random_axis();
        objs.sort_by(|a, b| {
            // TIME!!!!
            a.bounding_box(0.0, 1.0).unwrap().min[axis]
                .partial_cmp(&b.bounding_box(0.0, 1.0).unwrap().min[axis])
                .unwrap()
        });
        let mut a = objs;
        let b = a.split_off(a.len() / 2);
        let left = BVH::construct(a);
        let right = BVH::construct(b);
        let aabb = match (left.bounding_box(0.0, 1.0), right.bounding_box(0.0, 1.0)) {
            (Some(l), Some(r)) => Some(l + r),
            (None, Some(r)) => Some(r),
            (Some(l), None) => Some(l),
            _ => None
        };
        Box::new(BVH { left, right, aabb })
    }
}

impl Hittable for BVH {
    fn hit(&self, ray: &Ray, dist_min: f64, dist_max: f64) -> Option<Hit> {
        if !self.aabb.unwrap().hit(ray, dist_min, dist_max) { return None; }
        let left = self.left.hit(ray, dist_min, dist_max);
        let right = self.right.hit(ray, dist_min, dist_max);
        match (left, right) {
            (Some(hit), None) => Some(hit),
            (None, Some(hit)) => Some(hit),
            (Some(l_hit), Some(r_hit)) => if l_hit.dist < r_hit.dist { left } else { right },
            _ => None
        }
    }

    fn bounding_box(&self, t_min: f32, t_max: f32) -> Option<AABB> {
        self.aabb
    }
}
