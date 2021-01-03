use std::borrow::Borrow;
use std::ops::Range;
use std::sync::Arc;

use super::{AABB, Hit, Hittable, Material, RayCtx, P3, V3};
use crate::random::next_std_in_range;
use crate::types::Distance;

macro_rules! aarect_aabb {
    {$slf:ident, $a:tt, $b:tt, $off:expr} => {
        AABB::new(
            aarect_aabb!($slf, start, $a, $b, $off - 0.000001),
            aarect_aabb!($slf, end  , $a, $b, $off + 0.000001)
        )
    };
    {$slf:ident, $bound:ident, x, y, $off:expr} => {P3::new($slf.x.$bound, $slf.y.$bound, $off)};
    {$slf:ident, $bound:ident, x, z, $off:expr} => {P3::new($slf.x.$bound, $off, $slf.z.$bound)};
    {$slf:ident, $bound:ident, y, z, $off:expr} => {P3::new($off, $slf.y.$bound, $slf.z.$bound)};
}

macro_rules! norm_vec {
    {x, y} => {V3::z()};
    {x, z} => {V3::y()};
    {y, z} => {V3::x()};
}

macro_rules! aarect {
    {$name:tt, $a:tt, $b:tt, normal: $k:tt} =>{
        #[derive(Clone)]
        pub struct $name {
            $a: Range<Distance>,
            $b: Range<Distance>,
            k: Distance,
            material: Arc<dyn Material>
        }
        impl $name {
            pub fn new($a: Range<Distance>, $b:Range<Distance>, k:Distance, material: Arc<dyn Material>) -> $name {
                $name { $a, $b, k, material }
            }

            fn uv(&self, $a:Distance, $b: Distance) -> (Distance, Distance) {
                let u = ($a - self.$a.start)/(self.$a.end-self.$a.start);
                let v = ($b - self.$b.start)/(self.$b.end-self.$b.start);
                (u, v)
            }
        }

        impl Hittable for $name {
            fn hit(&self, ray_ctx: &RayCtx, dist_min: Distance, dist_max: Distance) -> Option<Hit> {
                let ray = &ray_ctx.ray;
                let dist = (self.k - ray.origin.$k) / ray.direction.$k;
                if !(dist_min..dist_max).contains(&dist) { return None; };

                let $a = ray.origin.$a + dist * ray.direction.$a;
                let $b = ray.origin.$b + dist * ray.direction.$b;

                if !(self.$a.contains(&$a) && self.$b.contains(&$b)) {
                    return None;
                };

                let (u, v) = self.uv($a, $b);
                Some(Hit::new(dist, ray.point_at(dist), norm_vec!($a, $b), self.material.borrow(), u, v))
            }

            fn bounding_box(&self, _: f32, _: f32) -> Option<AABB> {
                Some(aarect_aabb!(self, $a, $b, self.k))
            }

            fn pdf_value(&self, _origin: &P3, direction: &V3, hit: &Hit) -> f64 {
                let area = (self.$a.end - self.$a.start) * (self.$b.end - self.$b.start);
                // let sqr_dist = (hit.dist * hit.dist).sqrt();
                let sqr_dist = (hit.point - _origin).norm_squared();
                let cosine = direction.$k;
                let cos_area = Distance::abs(cosine * area);
                sqr_dist as f64 / cos_area as f64
            }

            fn random(&self, origin: &P3) -> V3 {
                let mut random_point = V3::from_element(1.0);
                random_point.$a = next_std_in_range(&self.$a);
                random_point.$b = next_std_in_range(&self.$b);
                random_point.$k = self.k;
                (&random_point - origin.coords)
            }

        }

    };
}

aarect!(XYRect, x, y, normal: z);
aarect!(XZRect, x, z, normal: y);
aarect!(YZRect, y, z, normal: x);
