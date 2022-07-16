use std::fmt::Debug;
use std::ops::Range;

use super::{AABB, Hit, Hittable, Material, RayCtx, P2, P3, V3};
use crate::random::next_std_in_range;
use crate::types::{Direction, Geometry, Probability, Timespan};
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
    {$($name:ident)::*, $a:tt, $b:tt, normal: $k:tt} =>{
        #[derive(Clone, Debug)]
        pub struct $($name)::*<T: Material> {
            $a: Range<Geometry>,
            $b: Range<Geometry>,
            k: Geometry,
            material: T
        }
        impl<T: Material>  $($name)::*<T> {
            pub fn new($a: Range<Geometry>, $b:Range<Geometry>, k:Geometry, material: T) ->  Self {
                 Self { $a, $b, k, material }
            }

            fn uv(&self, $a:Geometry, $b: Geometry) -> P2 {
                let u = ($a - self.$a.start)/(self.$a.end-self.$a.start);
                let v = ($b - self.$b.start)/(self.$b.end-self.$b.start);
                P2::new(u, v)
            }
        }

        impl<T: Material> Hittable for  $($name)::*<T> {
            fn hit(&self, ray_ctx: &RayCtx, dist_min: Geometry, dist_max: Geometry) -> Option<Hit> {
                let ray = &ray_ctx.ray;
                let dist = (self.k - ray.origin.$k) / ray.direction.$k;
                if !(dist_min..=dist_max).contains(&dist) { return None; };

                let $a = ray.origin.$a + dist * ray.direction.$a;
                let $b = ray.origin.$b + dist * ray.direction.$b;

                if !(self.$a.contains(&$a) && self.$b.contains(&$b)) {
                    return None;
                };

                let uv = self.uv($a, $b);
                Some(Hit::new(dist, ray.point_at(dist), norm_vec!($a, $b), &self.material, uv))
            }

            fn bounding_box(&self, _: Timespan) -> Option<AABB> {
                Some(aarect_aabb!(self, $a, $b, self.k))
            }

            fn pdf_value(&self, origin: &P3, direction: &Direction, hit: &Hit) -> Probability {
                let area = (self.$a.end - self.$a.start) * (self.$b.end - self.$b.start);
                let sqr_dist = hit.dist * hit.dist;
                let cosine = direction.$k;
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
