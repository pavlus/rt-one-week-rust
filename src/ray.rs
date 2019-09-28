use crate::vec::V3;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Ray {
    origin: V3,
    direction: V3,
    attenuation: V3,
    time: f32,
    ttl: u32,
}

impl Ray {
    pub fn new(origin: V3, direction: V3, attenuation: V3, time: f32, ttl: u32) -> Ray {
        Ray { origin, direction, attenuation, time, ttl }
    }

    pub fn produce(self, origin: V3, direction: V3, attenuation: V3) -> Ray {
        Ray::new(origin, direction, attenuation, self.time, self.ttl - 1)
    }

    pub fn time(self) -> f32 { self.time }

    pub fn validate(self) -> Option<Ray> {
        if self.ttl > 0 { Some(self) } else { None }
    }

    pub fn origin(self) -> V3 {
        self.origin
    }

    pub fn direction(self) -> V3 {
        self.direction
    }

    pub fn attenuation(self) -> V3 {
        self.attenuation
    }

    pub fn point_at(self, p: f64) -> V3 {
        self.origin + p * self.direction
    }
}