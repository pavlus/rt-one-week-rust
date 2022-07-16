use crate::ray::{Ray, RayCtx};
use crate::types::{V3, Geometry, Timespan, P2, Scale, Color};
use crate::random;
use crate::onb::ONB;
use nalgebra::{Matrix3, Matrix3x2, Unit};


// todo: bring random into context
#[derive(Clone, Debug)]
pub struct View {
    camera: LensCamera,
    timespan: Timespan,
    ttl: i32,
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
            timespan,
            ttl,
        }
    }

    pub fn get_ray(&self, uv: P2) -> RayCtx {
        let default_color = Color::new(0.0, 0.0, 0.0);

        let ray = self.camera.get_ray(uv);
        RayCtx::from_ray(
            ray,
            default_color,
            interpolation::lerp(&self.timespan.start, &self.timespan.end, &random::next_std()),
            self.ttl,
        )
    }
}

#[derive(Clone, Debug)]
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

    pub fn get_ray(&self, uv: P2) -> Ray {
        // lens deviation
        let deviation: V3 = &random::rand_in_unit_disc() * self.lens_radius;
        let offset = self.onb.local(&deviation);
        let p = self.inverse_projection_matrix * V3::new(uv.x, uv.y, 1.0);
        let direction = Unit::new_normalize(p - &offset);
        let origin = (&self.origin + &offset).into();
        Ray { origin, direction }
    }
}
