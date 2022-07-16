use crate::types::{V3, P3, Geometry, Probability, Timespan, Direction};

use super::{AABB, Hit, Hittable, RayCtx};
use nalgebra::Unit;
use itertools::Itertools;

#[derive(Debug)]
pub struct HittableList<T> {
    pub(crate) objects: Vec<T>,
    pub(crate) aabb: Option<AABB>,
}

impl <T: Hittable> HittableList<T> {
    pub fn new(objects: Vec<T>) -> HittableList<T> {
        let aabb = objects.iter()
            .flat_map(|o| o.bounding_box(0.0..1.0))
            .sum1();
        HittableList { objects, aabb }
    }
    pub fn empty() -> HittableList<T> {
        return Self::new(vec![]);
    }

    pub fn len(&self) -> usize {
        self.objects.len()
    }

    pub fn append(mut self, ref mut objects: Vec<T>) -> Self {
        let aabb = objects.iter()
            .flat_map(|o| o.bounding_box(0.0..1.0))
            .sum1();
        self.objects.append(objects);
        self.aabb = aabb.map(|o: AABB| o.combine(self.aabb));
        self
    }

    pub fn push(&mut self, object: T) {
        let aabb = object.bounding_box(0.0..1.0);
        self.objects.push(object);
        self.aabb = aabb.map(|o| o.combine(self.aabb));
    }
}

impl <T: Hittable> Hittable for HittableList<T> {
    fn hit(&self, ray: &RayCtx, dist_min: Geometry, dist_max: Geometry) -> Option<Hit> {
        let mut selected: (Geometry, Option<Hit>) = (dist_max, None);

        for o in &self.objects {
            if let Some(hit) = o.hit(ray, dist_min, selected.0) {
                if hit.dist < selected.0 {
                    selected = (hit.dist, Some(hit))
                }
            }
        }
        selected.1
    }

    fn bounding_box(&self, _: Timespan) -> Option<AABB> {
        self.aabb
    }

    fn pdf_value(&self, origin: &P3, direction: &Direction, hit: &Hit) -> Probability {
        self.objects
            .iter()
            .map(|o| o.pdf_value(origin, direction, hit))
            .sum::<Probability>() / self.objects.len() as Probability
    }

    fn random(&self, origin: &P3) -> Direction {
        crate::random::random_item(&self.objects)
            .unwrap()
            .random(origin)
    }
}
