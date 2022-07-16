use crate::types::{V3, P3, Geometry, Time, Color, Direction};
use nalgebra::Unit;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Ray{
    pub origin: P3,
    pub direction: Direction,
}

impl Ray {
    pub fn point_at(self, p: Geometry) -> P3 {
        (self.origin + (p * self.direction.as_ref())).into()
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
    pub fn new(origin: P3, direction: Direction, attenuation: Color, time: Time, ttl: i32) -> RayCtx {
        RayCtx { ray: Ray { origin, direction }, attenuation, time, ttl }
    }
    pub fn from_ray(ray: Ray, attenuation: Color, time: Time, ttl: i32) -> RayCtx {
        RayCtx { ray, attenuation, time, ttl }
    }

    pub fn produce(&self, origin: P3, direction: Direction, attenuation: Color) -> RayCtx {
        RayCtx::new(origin, direction, attenuation, self.time, self.ttl - 1)
    }

    pub fn validate(self) -> Option<RayCtx> {
        if self.ttl > 0 { Some(self) } else { None }
    }

}
