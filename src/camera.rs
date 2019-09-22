use crate::ray::Ray;
use crate::vec::V3;


#[derive(Copy, Clone, Debug)]
pub struct Camera {
    lower_left: V3,
    horizontal: V3,
    vertical: V3,
    origin: V3,
}

static DEFAULT_COLOR: V3 = V3 { x: 1.0, y: 1.0, z: 1.0 };
const TTL: u32 = 16;

impl Camera {
    pub fn new(
        lower_left: V3,
        horizontal: V3,
        vertical: V3,
        origin: V3) -> Camera {
        Camera {
            lower_left,
            horizontal,
            vertical,
            origin,
        }
    }

    pub fn new_default() -> Camera {
        Camera {
            lower_left: V3::new(-2.0, -1.0, -0.8),
            horizontal: V3::new(4.0, 0.0, 0.0),
            vertical: V3::new(0.0, 2.0, 0.0),
            origin: V3::new(0.0, 0.0, 0.0),
        }
    }
    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left
                + ((u * self.horizontal)
                + (v * self.vertical))
                - self.origin,
            DEFAULT_COLOR,
            TTL,
        )
    }
}
