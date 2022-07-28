use std::fmt::Debug;
use std::ops::Range;

use super::{AABB, Hit, Hittable, Material, RayCtx, P2, P3, V3};
use crate::random::next_std_in_range;
use crate::types::{Direction, Geometry, Probability, Timespan};
use crate::hittable::Bounded;


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

    const fn axis<const Axis: usize>() -> Direction {
        let result = V3::new(
            if Axis == X { 1.0 } else { 0.0 },
            if Axis == Y { 1.0 } else { 0.0 },
            if Axis == Z { 1.0 } else { 0.0 },
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

    fn random(&self, origin: &P3) -> Direction {
        let mut random_point = V3::from_element(1.0);
        random_point[A] = next_std_in_range(&self.a);
        random_point[B] = next_std_in_range(&self.b);
        random_point[K] = self.k;
        Direction::new_normalize(&random_point - origin.coords)
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

pub type XYRect<T> = AARect<T, X, Y, Z>;
pub type XZRect<T> = AARect<T, X, Z, Y>;
pub type YZRect<T> = AARect<T, Y, Z, X>;

