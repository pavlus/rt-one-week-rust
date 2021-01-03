use std::ops::Add;

use crate::ray::Ray;
use crate::types::{V3, P3, Distance};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AABB {
    pub min: P3,
    pub max: P3,
}

impl AABB {
    pub fn new(min: P3, max: P3) -> AABB {
        AABB { min, max }
    }

    pub fn hit(self, ray: &Ray, d_min: Distance, d_max: Distance) -> bool {
        let direction = &ray.direction; // f64x3
        let start = &ray.origin.coords; // f64x3
        let (minx, maxx) = if direction.x.is_sign_negative() { (self.max.x, self.min.x) } else { (self.min.x, self.max.x) };
        let (miny, maxy) = if direction.y.is_sign_negative() { (self.max.y, self.min.y) } else { (self.min.y, self.max.y) };
        let (minz, maxz) = if direction.z.is_sign_negative() { (self.max.z, self.min.z) } else { (self.min.z, self.max.z) };

        let d0 = (V3::new(minx, miny, minz) - start).component_div(direction); // (f64x3 -  f64x3) / f64x3
        let d1 = (V3::new(maxx, maxy, maxz) - start).component_div(direction); // (f64x3 -  f64x3) / f64x3

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
        let min = P3::new(
            Distance::min(self.min.x, rhs.min.x),
            Distance::min(self.min.y, rhs.min.y),
            Distance::min(self.min.z, rhs.min.z),
        );
        let max = P3::new(
            Distance::max(self.max.x, rhs.max.x),
            Distance::max(self.max.y, rhs.max.y),
            Distance::max(self.max.z, rhs.max.z),
        );
        AABB { min, max }
    }
}
