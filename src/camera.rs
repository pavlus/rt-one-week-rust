use std::f64::consts::PI;

use rand::distributions::Standard;
use rand::Rng;

use crate::ray::Ray;
use crate::vec::V3;

#[derive(Copy, Clone, Debug)]
pub struct Camera {
    lower_left: V3,
    horizontal: V3,
    vertical: V3,
    origin: V3,
    u: V3,
    v: V3,
    w: V3,
    lens_radius: f32,
    ttl: u32,
}

static DEFAULT_COLOR: V3 = V3 { x: 0.0, y: 0.0, z: 0.0 };
const TTL: u32 = 16;

impl Camera {
    pub fn new_look(
        from: V3, at: V3, up: V3,
        vfov: f64, aspect: f64,
        focus_distance: f64, aperture: f32,
    ) -> Camera {
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
            // from origin subtract half of horizontal viewport and half of vertical viewport,
            // then offset by w; todo: research focus distance impact on values
            lower_left: from - focus_distance * ((width / 2.0) * u + (height / 2.0) * v + w),
            horizontal: focus_distance * width * u,
            vertical: focus_distance * height * v,
            origin: from,
            u,
            v,
            w,
            lens_radius: aperture / 2.0,
            ttl: 16,
        }
    }
    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * rand_in_unit_sphere();
        let offset = rd.x * self.u + rd.y * self.v;
        let tmp_origin = self.origin + offset;
        Ray::new(
            tmp_origin,
            self.lower_left
                + ((s * self.horizontal)
                + (t * self.vertical))
                - tmp_origin,
            DEFAULT_COLOR,
            TTL,
        )
    }
}


fn rand_in_unit_sphere() -> V3 {
    let mut rand = rand::thread_rng();
    loop {
        let v: V3 = 2.0 * V3::new(rand.sample(Standard), rand.sample(Standard), 0.0);
        if v.sqr_length() <= 1 as f64 {
            return v;
        }
    }
}