use crate::types::{P3, Geometry, Time, Direction};

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
    pub time: Time,
}

impl RayCtx {
    pub fn new(origin: P3, direction: Direction, time: Time) -> RayCtx {

        RayCtx { ray: Ray { origin, direction }, time }
    }
    pub fn from_ray(ray: Ray, time: Time) -> RayCtx {
        RayCtx { ray, time}
    }

    pub fn produce(self, origin: P3, direction: Direction) -> RayCtx {
        RayCtx::new(origin, direction, self.time)
    }

}
