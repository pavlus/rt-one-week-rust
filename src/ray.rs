use crate::types::{V3, P3, Distance, Time, Color};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Ray{
    pub origin: P3,
    pub direction: V3, // todo: make it unit length type
}

impl Ray {
    pub fn point_at(self, p: Distance) -> P3 {
        (self.origin.coords + p * self.direction).into()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct RayCtx {
    pub ray: Ray,
    pub attenuation: Color,
    pub time: Time,
    pub ttl: i32,
}

impl RayCtx {
    pub fn new(origin: P3, direction: V3, attenuation: Color, time: Time, ttl: i32) -> RayCtx {
        RayCtx { ray: Ray { origin, direction }, attenuation, time, ttl }
    }

    pub fn produce(&self, origin: P3, direction: V3, attenuation: Color) -> RayCtx {
        RayCtx::new(origin, direction, attenuation, self.time, self.ttl - 1)
    }

    pub fn validate(self) -> Option<RayCtx> {
        if self.ttl > 0 { Some(self) } else { None }
    }

}
