use std::borrow::Borrow;
use std::ops::Range;
use std::sync::Arc;

use super::{AABB, Hit, Hittable, Material, RayCtx, P2, P3, V3};
use crate::random::next_std_in_range;
use crate::types::{Distance, Probability};
use nalgebra::Unit;

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
    {x, y} => {Unit::new_unchecked(V3::z())};
    {x, z} => {Unit::new_unchecked(V3::y())};
    {y, z} => {Unit::new_unchecked(V3::x())};
}

macro_rules! aarect {
    {$name:tt, $a:tt, $b:tt, normal: $k:tt} =>{
        #[derive(Clone, Debug)]
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

            fn uv(&self, $a:Distance, $b: Distance) -> P2 {
                let u = ($a - self.$a.start)/(self.$a.end-self.$a.start);
                let v = ($b - self.$b.start)/(self.$b.end-self.$b.start);
                P2::new(u, v)
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

                let uv = self.uv($a, $b);
                Some(Hit::new(dist, ray.point_at(dist), norm_vec!($a, $b), self.material.borrow(), uv))
            }

            fn bounding_box(&self, _: f32, _: f32) -> Option<AABB> {
                Some(aarect_aabb!(self, $a, $b, self.k))
            }

            fn pdf_value(&self, origin: &P3, direction: &Unit<V3>, hit: &Hit) -> Probability {
                let area = (self.$a.end - self.$a.start) * (self.$b.end - self.$b.start);
                // let sqr_dist = (hit.dist * hit.dist).sqrt();
                let mut center = P3::new(0.0, 0.0, 0.0);
                center.$a = (self.$a.end + self.$a.start) / 2.0;
                center.$b = (self.$b.end + self.$b.start) / 2.0;
                center.$k = self.k;
                let sqr_dist = (&center - origin).norm_squared();
                let cosine = direction.$k;
                let cos_area = Distance::abs(cosine * area);
                sqr_dist as Probability / cos_area as Probability
            }

            fn random(&self, origin: &P3) -> Unit<V3> {
                let mut random_point = V3::from_element(1.0);
                random_point.$a = next_std_in_range(&self.$a);
                random_point.$b = next_std_in_range(&self.$b);
                random_point.$k = self.k;
                Unit::new_normalize(&random_point - origin.coords)
            }

        }

    };
}

aarect!(XYRect, x, y, normal: z);
aarect!(XZRect, x, z, normal: y);
aarect!(YZRect, y, z, normal: x);
