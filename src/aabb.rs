use std::ops::Add;

use crate::ray::Ray;
use crate::vec::V3;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AABB {
    pub min: V3,
    pub max: V3,
}

impl AABB {
    pub fn new(min: V3, max: V3) -> AABB {
        AABB { min, max }
    }

    pub fn hit(self, ray: &Ray, d_min: f64, d_max: f64) -> bool {
        let direction = ray.direction; // f64x3
        let start = ray.origin; // f64x3
        let (minx, maxx) = if direction.x.is_sign_negative() { (self.max.x, self.min.x) } else { (self.min.x, self.max.x) };
        let (miny, maxy) = if direction.y.is_sign_negative() { (self.max.y, self.min.y) } else { (self.min.y, self.max.y) };
        let (minz, maxz) = if direction.z.is_sign_negative() { (self.max.z, self.min.z) } else { (self.min.z, self.max.z) };

        let d0 = (V3::new(minx, miny, minz) - start) / direction; // (f64x3 -  f64x3) / f64x3
        let d1 = (V3::new(maxx, maxy, maxz) - start) / direction; // (f64x3 -  f64x3) / f64x3

        let minx = if d_min < d0.x { d0.x } else { d_min };
        let miny = if d_min < d0.y { d0.y } else { d_min };
        let minz = if d_min < d0.z { d0.z } else { d_min };
        let maxx = if d1.x < d_max { d1.x } else { d_max };
        let maxy = if d1.y < d_max { d1.y } else { d_max };
        let maxz = if d1.z < d_max { d1.z } else { d_max };

        maxx > minx && maxy > miny && maxz > minz
    }
}

impl Add for AABB {
    type Output = AABB;

    fn add(self, rhs: Self) -> Self::Output {
        let min = V3::new(
            f64::min(self.min.x, rhs.min.x),
            f64::min(self.min.y, rhs.min.y),
            f64::min(self.min.z, rhs.min.z),
        );
        let max = V3::new(
            f64::max(self.max.x, rhs.max.x),
            f64::max(self.max.y, rhs.max.y),
            f64::max(self.max.z, rhs.max.z),
        );
        AABB { min, max }
    }
}
