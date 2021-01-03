use crate::ray::RayCtx;
use crate::types::{V3, Distance, Timespan, P2, Scale, Color};
use crate::random;
use crate::onb::ONB;

#[derive(Clone, Debug)]
pub struct Camera {
    lower_left: V3,
    horizontal: V3,
    vertical: V3,
    origin: V3,
    onb: ONB,
    lens_radius: Distance,
    timespan: Timespan,
    ttl: i32,
}

impl Camera {
    pub fn new_look(
        from: V3, at: V3, up: V3,
        vfov: Scale, aspect: Scale,
        focus_distance: Distance, aperture: Distance,
        timespan: Timespan,
        ttl: i32
    ) -> Camera {
        let theta = vfov.to_radians();
        let height = (theta / 2.0).tan();
        let width = aspect * height;

        // normalized vector from origin to POI
        let w = (&from - &at).normalize();
        // cross-product of upwards vector and w will give us normal to plane they are in.
        // it's also normal to both of them, being normal to upwards direction makes it horizontal
        let u = up.cross(&w);
        // given that we have u and w is normal to plane of viewport -- v is their cross-product
        let v = w.cross(&u);
        let onb = ONB { u, v, w };
        Camera {
            // from origin subtract half of horizontal viewport and half of vertical viewport,
            // then offset by w; todo: research focus distance impact on values
            lower_left: -focus_distance * (onb.local(&V3::new(width / 2.0, height / 2.0 , 1.0))),
            horizontal: onb.local(&(V3::x() * focus_distance *  width)),
            vertical: onb.local(&(V3::y() * focus_distance * height)),
            origin: from,
            onb,
            lens_radius: aperture / 2.0,
            timespan,
            ttl,
        }
    }

    pub fn get_ray(&self, coordinates: P2) -> RayCtx {
        let default_color = Color::new(0.0, 0.0, 0.0);

        let deviation: V3 = &random::rand_in_unit_disc() * self.lens_radius;
        let offset = self.onb.local(&deviation);
        let x: V3 = coordinates.x * &self.horizontal;
        let y: V3 = coordinates.y * &self.vertical;
        RayCtx::new(
            (&self.origin + &offset).into(),
            (&self.lower_left + x + y - &offset).normalize(),
            default_color,
            interpolation::lerp(&self.timespan.start, &self.timespan.end, &random::next_std_f32()),
            self.ttl,
        )
    }
}
