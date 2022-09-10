use std::fmt::Debug;
use std::ops::Range;
use rand::distributions::uniform::SampleRange;

use super::{AABB, Hit, Hittable, Material, RayCtx, P2, P3, V3};
use crate::types::{Direction, Geometry, Probability, Timespan};
use crate::hittable::{Bounded, Important, Positionable};
use crate::random2::DefaultRng;


const X: usize = 0;
const Y: usize = 1;
const Z: usize = 2;

#[derive(Clone, Debug)]
pub struct AARect<T, const A: usize, const B: usize, const K: usize> {
    a: Range<Geometry>,
    b: Range<Geometry>,
    k: Geometry,
    material: T,
}

impl<T, const A: usize, const B: usize, const K: usize> AARect<T, A, B, K> {
    pub fn new(a: Range<Geometry>, b: Range<Geometry>, k: Geometry, material: T) -> Self {
        Self { a, b, k, material }
    }

    fn uv(&self, a: Geometry, b: Geometry) -> P2 {
        let u = (a - self.a.start) / (self.a.end - self.a.start);
        let v = (b - self.b.start) / (self.b.end - self.b.start);
        P2::new(u, v)
    }

    const fn axis<const AXIS: usize>() -> Direction {
        let result = V3::new(
            if AXIS == X { 1.0 } else { 0.0 },
            if AXIS == Y { 1.0 } else { 0.0 },
            if AXIS == Z { 1.0 } else { 0.0 },
        );
        Direction::new_unchecked(result)
    }
}

impl<T: Material, const A: usize, const B: usize, const K: usize> Hittable for AARect<T, A, B, K> {
    fn hit(&self, ray_ctx: &RayCtx, dist_min: Geometry, dist_max: Geometry) -> Option<Hit> {
        let ray = &ray_ctx.ray;
        let dist = (self.k - ray.origin[K]) / ray.direction[K];
        if !(dist_min..=dist_max).contains(&dist) { return None; };

        let a = ray.origin[A] + dist * ray.direction[A];
        let b = ray.origin[B] + dist * ray.direction[B];

        if !(self.a.contains(&a) && self.b.contains(&b)) {
            return None;
        };

        let uv = self.uv(a, b);
        Some(Hit::new(dist, ray.point_at(dist), Self::axis::<K>(), &self.material, uv))
    }

}


impl<T, const A: usize, const B: usize, const K: usize> Bounded for AARect<T, A, B, K> {
    fn bounding_box(&self, _: Timespan) -> AABB {
        let a = Self::axis::<A>();
        let b = Self::axis::<B>();
        let k = Self::axis::<K>();
        let min = a.as_ref() * self.a.start + b.as_ref() * self.b.start + k.as_ref() * (self.k - 0.00001);
        let max = a.as_ref() * self.a.end + b.as_ref() * self.b.end + k.as_ref() * (self.k - 0.00001);
        AABB::new(min.into(), max.into())

    }
}


impl<T: Send + Sync, const A: usize, const B: usize, const K: usize> Important for AARect<T, A, B, K> {

    fn pdf_value(&self, _origin: &P3, direction: &Direction, hit: &Hit) -> Probability {
        let area = (self.a.end - self.a.start) * (self.b.end - self.b.start);
        let sqr_dist = hit.dist * hit.dist;
        let cosine = direction[K];
        let cos_area = Geometry::abs(cosine * area);

        if false && cfg!(test) {
            eprintln!("area: {}", area);
            eprintln!("sqr_dist: {}", sqr_dist);
            eprintln!("cosine: {}", cosine);
            eprintln!("cos_area: {}", cos_area);

            eprintln!("----------------------------");
        }
        sqr_dist as Probability / cos_area as Probability
    }

    fn random(&self, origin: &P3, rng: &mut DefaultRng) -> Direction {
        let mut random_point = V3::from_element(1.0);
        random_point[A] = self.a.clone().sample_single(rng);
        random_point[B] = self.b.clone().sample_single(rng);
        random_point[K] = self.k;
        Direction::new_normalize(&random_point - origin.coords)
    }
}


pub type XYRect<T> = AARect<T, X, Y, Z>;
pub type XZRect<T> = AARect<T, X, Z, Y>;
pub type YZRect<T> = AARect<T, Y, Z, X>;

impl<T, const A: usize, const B: usize, const K: usize> Positionable for AARect<T, A, B, K>{
    fn move_by(&mut self, offset: &V3) {
        self.a = (self.a.start+offset[A])..(self.a.end + offset[A]);
        self.b = (self.b.start+offset[A])..(self.b.end + offset[B]);
        self.k = self.k + offset[K];
    }

    fn moved_by(self, offset: &V3) -> Self {
        AARect::<T, A, B, K>::new(
            (self.a.start+offset[A])..(self.a.end + offset[A]),
            (self.b.start+offset[A])..(self.b.end + offset[B]),
            self.k + offset[K],
            self.material
        )
    }
}
