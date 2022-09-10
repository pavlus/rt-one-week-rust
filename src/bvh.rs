use std::fmt::Debug;
use crate::aabb::AABB;
use crate::hittable::{Hit, HittableList, Hittable};
use crate::ray::RayCtx;
use crate::types::{Geometry, Timespan};
use crate::V3;

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
        let axis = Self::pick_axis(&mut objs);
        objs.sort_by(|a, b| {
            // TIME!!!!
            a.bounding_box(0.0..1.0).unwrap().min[axis]
                .partial_cmp(&b.bounding_box(0.0..1.0).unwrap().min[axis])
                .unwrap()
        });
        let mut a = objs;
        let b = a.split_off(Self::pick_split(&a));
        let left = BVH::construct(a);
        let right = BVH::construct(b);
        let aabb = match (left.bounding_box(0.0..1.0), right.bounding_box(0.0..1.0)) {
            (Some(l), Some(r)) => Some(l + r),
            (None, Some(r)) => Some(r),
            (Some(l), None) => Some(l),
            _ => None
        };
        Box::new(BVH { left, right, aabb })
    }

    fn pick_split(a: &Vec<Box<dyn Hittable>>) -> usize {
        a.len() / 2
    }

    fn pick_axis(objs: &mut Vec<Box<dyn Hittable>>) -> usize {
        let aabb = objs.iter()
            .map(|h| h.bounding_box(0.0..1.0))
            .filter_map(|x| x)
            .sum::<AABB>();
        let span: V3 = (aabb.max.coords - aabb.min.coords).map(Geometry::abs);
        span.iamax()
    }
}

impl Hittable for BVH {
    fn hit(&self, ray_ctx: &RayCtx, dist_min: Geometry, dist_max: Geometry) -> Option<Hit> {
        if !self.aabb.unwrap().hit(&ray_ctx.ray, dist_min, dist_max) { return None; }
        let left = self.left.hit(ray_ctx, dist_min, dist_max);
        let right = self.right.hit(ray_ctx, dist_min, dist_max);
        match (left, right) {
            (Some(hit), None) => Some(hit),
            (None, Some(hit)) => Some(hit),
            (Some(l_hit), Some(r_hit)) => if l_hit.dist < r_hit.dist {
                left
            } else {
                right
            },
            _ => None
        }
    }

    fn bounding_box(&self, _: Timespan) -> Option<AABB> {
        self.aabb
    }

    //todo: random and PDF
}
