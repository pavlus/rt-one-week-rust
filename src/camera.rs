use std::sync::atomic::{AtomicUsize, Ordering};
use crate::ray::{Ray, RayCtx};
use crate::types::{V3, Geometry, Timespan, P2, Scale, Time};
use crate::onb::ONB;
use nalgebra::{Matrix3, Unit};
use rand::distributions::Standard;
use rand::prelude::Distribution;
use rand_distr::UnitDisc;
use crate::random2::DefaultRng;


// todo: bring random into context
#[derive(Debug)]
pub struct View {
    camera: LensCamera,
    pub timespan_start: Time,
    pub timespan_end: Time,
    pub ttl: i32,
    #[cfg(feature = "metrics")]
    pub ray_cnt: AtomicUsize,
}

impl View {
    pub fn new_look(
        from: V3, at: V3, up: V3,
        vfov: Scale, aspect: Scale,
        focus_distance: Geometry, aperture: Geometry,
        timespan: Timespan,
        ttl: i32,
    ) -> View {

        View {
            camera: LensCamera::new_look(from, at, up, vfov, aspect, focus_distance, aperture),
            timespan_start: timespan.start,
            timespan_end: timespan.end,
            ttl,
            #[cfg(feature = "metrics")]
            ray_cnt: AtomicUsize::new(0),
        }
    }

    pub fn get_ray(&self, uv: P2, rng: &mut DefaultRng) -> RayCtx {
        #[cfg(feature = "metrics")]
        self.ray_cnt.fetch_add(1, Ordering::Relaxed);
        let ray = self.camera.get_ray(uv, rng);
        RayCtx::from_ray(
            ray,
            interpolation::lerp(&self.timespan_start, &self.timespan_end, &mut Standard.sample(rng))
        )
    }
}

#[derive(Copy, Clone, Debug)]
struct LensCamera {
    inverse_projection_matrix: Matrix3<Geometry>,
    origin: V3,
    onb: ONB,
    lens_radius: Geometry,
}

impl LensCamera {
    pub fn new_look(
        from: V3, at: V3, up: V3,
        vfov: Scale, aspect: Scale,
        focus_distance: Geometry, aperture: Geometry,
    ) -> LensCamera {
        let theta = vfov.to_radians();
        let height = (theta / 2.0).tan();
        let width = aspect * height;

        let basis = ONB::from_up_w(Unit::new_normalize(up), Unit::new_normalize(&from - &at));

        let x = basis.local(&(V3::x() * focus_distance * width));
        let y = basis.local(&(V3::y() * focus_distance * height));
        let z = -focus_distance * (basis.local(&V3::new(width / 2.0, height / 2.0, 1.0)));
        LensCamera {
            inverse_projection_matrix: Matrix3::from_columns(&[x, y, z]),
            origin: from,
            onb: basis,
            lens_radius: aperture / 2.0,
        }
    }

    pub fn get_ray(&self, uv: P2, rng: &mut DefaultRng) -> Ray {
        // lens deviation
        let disk: [Geometry; 2] = UnitDisc.sample(rng);
        let deviation: V3 = V3::new(disk[0], disk[1], 0.0) * self.lens_radius;
        let offset = self.onb.local(&deviation);
        let p = self.inverse_projection_matrix * V3::new(uv.x, uv.y, 1.0);
        let direction = Unit::new_normalize(p - &offset);
        let origin = (&self.origin + &offset).into();
        Ray { origin, direction }
    }
}
