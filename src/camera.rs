use crate::ray::Ray;
use crate::vec::V3;
use std::f64::consts::PI;
use std::alloc::handle_alloc_error;


#[derive(Copy, Clone, Debug)]
pub struct Camera {
    lower_left: V3,
    horizontal: V3,
    vertical: V3,
    origin: V3,
    ttl: u32,
}

static DEFAULT_COLOR: V3 = V3 { x: 0.0, y: 0.0, z: 0.0 };
const TTL: u32 = 16;

impl Camera {
    pub fn new(
        lower_left: V3,
        horizontal: V3,
        vertical: V3,
        origin: V3,
        ttl: u32,
    ) -> Camera {
        Camera {
            lower_left,
            horizontal,
            vertical,
            origin,
            ttl,
        }
    }

    pub fn new_default() -> Camera {
        Camera {
            lower_left: V3::new(-2.0, -1.0, -0.8),
            horizontal: V3::new(4.0, 0.0, 0.0),
            vertical: V3::new(0.0, 2.0, 0.0),
            origin: V3::new(0.0, 0.0, 0.0),
            ttl: 16,
        }
    }

    pub fn new_look(from: V3, at: V3, up: V3, vfov: f64, aspect: f64) -> Camera {
        let theta = vfov * PI / 180.0;
        let height = (theta / 2.0).tan();
        let width = aspect * height;

        // normalized vector from origin to POI
        let w = (from - at).unit();
        // cross-product of upwards vector and w will give us normal to plane they are in.
        // it's also normal to both of them, being normal to upwards direction makes it horizontal
        let u = up.cross(w);
        // given that we have u and w is normal to plane of viewport -- v is their cross-product
        let v = w.cross(u);
        Camera {
            // from origin substruct half of horizontal viewport and half of vertival viewport,
            // then offset by w
            lower_left: from - (width / 2.0) * u - (height / 2.0) * v - w,
            horizontal: width * u,
            vertical: height * v,
            origin: from,
            ttl: 16,
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
