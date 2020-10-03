use std::borrow::Borrow;
use std::ops::Range;
use std::sync::Arc;

use super::{AABB, Hit, Hittable, Material, Ray, V3};
use crate::random::next_std_f64_in_range;

macro_rules! aarect_aabb {
    {$slf:ident, $a:tt, $b:tt, $off:expr} => {
        AABB::new(
            aarect_aabb!($slf, start, $a, $b, $off - 0.001),
            aarect_aabb!($slf, end  , $a, $b, $off + 0.001)
        )
    };
    {$slf:ident, $bound:ident, x, y, $off:expr} => {V3::new($slf.x.$bound, $slf.y.$bound, $off)};
    {$slf:ident, $bound:ident, x, z, $off:expr} => {V3::new($slf.x.$bound, $off, $slf.z.$bound)};
    {$slf:ident, $bound:ident, y, z, $off:expr} => {V3::new($off, $slf.y.$bound, $slf.z.$bound)};
}

macro_rules! norm_vec {
    {x, y} => {V3::new(0.0,0.0,1.0)};
    {x, z} => {V3::new(0.0,1.0,0.0)};
    {y, z} => {V3::new(1.0,0.0,0.0)};
}

macro_rules! aarect {
    {$name:tt, $a:tt, $b:tt, normal: $k:tt} =>{
        #[derive(Debug, Clone)]
        pub struct $name {
            $a: Range<f64>,
            $b: Range<f64>,
            k: f64,
            material: Arc<dyn Material>
        }
        impl $name {
            pub fn new($a: Range<f64>, $b:Range<f64>, k:f64, material: Arc<dyn Material>) -> $name {
                $name { $a, $b, k, material }
            }

            fn uv(&self, $a:f64, $b: f64) -> (f64, f64) {
                let u = ($a - self.$a.start)/(self.$a.end-self.$a.start);
                let v = ($b - self.$b.start)/(self.$b.end-self.$b.start);
                (u, v)
            }
        }

        impl Hittable for $name {
            fn hit(&self, ray: &Ray, dist_min: f64, dist_max: f64) -> Option<Hit> {
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

            fn pdf_value(&self, _origin: &V3, direction: &V3, hit: &Hit) -> f64 {
                let area = (self.$a.end - self.$a.start) * (self.$b.end - self.$b.start);
                let sqr_dist = (hit.dist * hit.dist);
                let cosine = direction.$k;
                let cos_area = f64::abs(cosine * area);
                sqr_dist / cos_area
            }

            fn random(&self, origin: &V3) -> V3 {
                let random_point = V3{
                    $a: next_std_f64_in_range(&self.$a),
                    $b: next_std_f64_in_range(&self.$b),
                    $k: self.k
                 };
                (random_point - *origin)
            }

        }

    };
}

aarect!(XYRect, x, y, normal: z);
aarect!(XZRect, x, z, normal: y);
aarect!(YZRect, y, z, normal: x);
