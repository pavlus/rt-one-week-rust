use std::fmt::Debug;
use itertools::Itertools;
use crate::aabb::AABB;
use crate::hittable::{Hit, Hittable, Bounded};
use crate::ray::RayCtx;
use crate::types::{Geometry, Timespan};

#[derive(Debug)]
pub struct BVHIndexed<T> {
    objects: Vec<T>,
    nodes: Vec<BVHNode>,
}

#[derive(Debug)]
enum BVHNode {
    Node {
        // nodes idx
        left: usize,
        // nodes idx
        right: usize,
        aabb: AABB,
    },
    Leaf {
        obj_indices: Vec<usize>,
        aabb: AABB,
    },
}

impl Bounded for BVHNode {
    fn bounding_box(&self, _timespan: Timespan) -> AABB {
        match &self {
            BVHNode::Node { aabb, .. } => { *aabb }
            BVHNode::Leaf { aabb, .. } => { *aabb }
        }
    }
}

impl<T> Bounded for BVHIndexed<T> {
    fn bounding_box(&self, timespan: Timespan) -> AABB {
        self.nodes.last().unwrap().bounding_box(timespan)
    }
}

pub trait BoundedHittable: Hittable + Bounded {}

impl<T: Hittable + Bounded + ?Sized + 'static> BoundedHittable for T {}

const CUTOFF: usize = 8;

impl<T: Bounded> BVHIndexed<T> {
    pub(crate) fn new(objects: Vec<T>, timespan: Timespan) -> BVHIndexed<T> {
        let mut nodes: Vec<BVHNode> = Vec::new();
        let mut indices = (0..objects.len()).collect_vec();
        BVHIndexed::construct(&objects, indices.as_mut_slice(), &mut nodes, timespan);
        BVHIndexed {
            objects,
            nodes,
        }
    }
    fn construct(objs: &Vec<T>, indices: &mut [usize], nodes: &mut Vec<BVHNode>, timespan: Timespan) -> usize {
        if indices.len() <= CUTOFF {
            let aabb = indices.iter().filter_map(|&i| objs.get(i))
                .map(|h| h.bounding_box(timespan.clone()))
                .sum1().unwrap();
            nodes.push(BVHNode::Leaf {
                obj_indices: indices.into(),
                aabb,
            });
            return nodes.len() - 1;
        }
        let axis = Self::pick_axis(&objs);
        indices.sort_by(|&a, &b| {
            let a = objs.get(a).unwrap().bounding_box(timespan.clone());
            let b = objs.get(b).unwrap().bounding_box(timespan.clone());
            a.center()[axis]
                .partial_cmp(&b.center()[axis])
                .unwrap()
        });
        let (a, b) = indices.split_at_mut(indices.len() / 2);
        let left = BVHIndexed::construct(objs, a, nodes, timespan.clone());
        let right = BVHIndexed::construct(objs, b, nodes, timespan.clone());
        let aabb = nodes.get(left).unwrap().bounding_box(timespan.clone())
            + nodes.get(right).unwrap().bounding_box(timespan.clone());
        nodes.push(BVHNode::Node { left, right, aabb });
        return nodes.len() - 1;
    }

    fn pick_axis(objs: &Vec<T>) -> usize {
        objs.iter()
            .map(|h| Bounded::bounding_box(h, 0.0..1.0))
            .sum::<AABB>()
            .max_axis()
    }
}

impl<T: Hittable + Bounded> BVHIndexed<T> {
    fn hit(&self, node_idx: usize, ray_ctx: &RayCtx, dist_min: Geometry, dist_max: Geometry) -> Option<Hit> {
        match self.nodes.get(node_idx).unwrap() {
            BVHNode::Node { left, right, aabb } => {
                if aabb.hit(&ray_ctx.ray, dist_min, dist_max) {
                    let left = self.hit(*left, ray_ctx, dist_min, dist_max);
                    let right = self.hit(*right, ray_ctx, dist_min, dist_max);
                    left.zip(right).map(|(left, right)| if left.dist < right.dist { left } else { right })
                        .or(left).or(right)
                } else { None }
            }
            BVHNode::Leaf { aabb, obj_indices } => {
                if aabb.hit(&ray_ctx.ray, dist_min, dist_max) {
                    let mut selected: (Geometry, Option<Hit>) = (dist_max, None);

                    for &i in obj_indices.iter() {
                        if let Some(hit) = self.objects.get(i).unwrap().hit(ray_ctx, dist_min, selected.0) {
                            if hit.dist < selected.0 {
                                selected = (hit.dist, Some(hit))
                            }
                        }
                    }
                    selected.1
                } else { None }
            }
        }
    }
}

impl<T: Hittable + Bounded> Hittable for BVHIndexed<T> {
    fn hit(&self, ray_ctx: &RayCtx, dist_min: Geometry, dist_max: Geometry) -> Option<Hit> {
        BVHIndexed::hit(self, self.nodes.len() - 1, ray_ctx, dist_min, dist_max)
    }
}


