use crate::ray::Ray;
use crate::vec::V3;
use crate::random;

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
    t0: f32,
    t1: f32,
    ttl: i32,
}

static DEFAULT_COLOR: V3 = V3::zeros();

impl Camera {
    pub fn new_look(
        from: V3, at: V3, up: V3,
        vfov: f64, aspect: f64,
        focus_distance: f64, aperture: f32,
        t0: f32, t1: f32,
        ttl: i32
    ) -> Camera {
        let theta = vfov.to_radians();
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
            t0,
            t1,
            ttl,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let [dx, dy] = random::rand_in_unit_disc();
        let offset = self.lens_radius as f64 * dx * self.u + self.lens_radius as f64 * dy * self.v;
        let tmp_origin = self.origin + offset;
        Ray::new(
            tmp_origin,
            self.lower_left
                + ((s * self.horizontal)
                + (t * self.vertical))
                - tmp_origin,
            DEFAULT_COLOR,
            interpolation::lerp(&self.t0, &self.t1, &random::next_std_f32()),
            self.ttl,
        )
    }
}
