use crate::vec::Axis;
use crate::vec::V3;
use std::ops::Add;
use crate::ray::Ray;

#[derive(Copy, Clone, Debug)]
pub struct AABB {
    pub min: V3,
    pub max: V3,
}

impl AABB {
    pub fn new(min: V3, max: V3) -> AABB {
        AABB { min, max }
    }

    pub fn hit(self, ray: &Ray, d_min: f64, d_max: f64) -> bool {
        for axis in Axis::xyz().iter() {
            let direction = ray.direction[axis];
            let start = ray.origin[axis];
            let mut d0 = (self.min[axis] - start) / direction;
            let mut d1 = (self.max[axis] - start) / direction;
            if direction.is_sign_negative() {
                std::mem::swap(&mut d0, &mut d1);
            }
            let min = if d_min < d0 { d0 } else { d_min };
            let max = if d1 < d_max { d1 } else { d_max };
            if max <= min { return false; };
        }
        true
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