use crate::vec::V3;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Ray{
    pub origin: V3,
    pub direction: V3,
}

impl Ray {
    pub fn point_at(self, p: f64) -> V3 {
        self.origin + p * self.direction
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct RayCtx {
    pub ray: Ray,
    pub attenuation: V3,
    pub time: f32,
    pub ttl: i32,
}

impl RayCtx {
    pub fn new(origin: V3, direction: V3, attenuation: V3, time: f32, ttl: i32) -> RayCtx {
        RayCtx { ray: Ray { origin, direction }, attenuation, time, ttl }
    }

    pub fn produce(self, origin: V3, direction: V3, attenuation: V3) -> RayCtx {
        RayCtx::new(origin, direction, attenuation, self.time, self.ttl - 1)
    }

    pub fn validate(self) -> Option<RayCtx> {
        if self.ttl > 0 { Some(self) } else { None }
    }

}
