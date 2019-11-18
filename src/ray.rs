use crate::vec::V3;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Ray {
    pub origin: V3,
    pub direction: V3,
    pub attenuation: V3,
    pub time: f32,
    pub ttl: i32,
}

impl Ray {
    pub fn new(origin: V3, direction: V3, attenuation: V3, time: f32, ttl: i32) -> Ray {
        Ray { origin, direction, attenuation, time, ttl }
    }

    pub fn produce(self, origin: V3, direction: V3, attenuation: V3) -> Ray {
        Ray::new(origin, direction, attenuation, self.time, self.ttl - 1)
    }

    pub fn validate(self) -> Option<Ray> {
        if self.ttl > 0 { Some(self) } else { None }
    }

    pub fn point_at(self, p: f64) -> V3 {
        self.origin + p * self.direction
    }
}