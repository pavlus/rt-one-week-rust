use super::{Hittable, Ray, Renderer, V3};
use std::borrow::Borrow;
use crate::random::next_std_f64;
use std::ops::Range;
use crate::texture::Color;

pub struct RgbRenderer {
    pub hittable: Box<dyn Hittable>,
    pub miss_shader: fn(&Ray) -> V3,
}

impl Renderer for RgbRenderer {
    fn color(&self, r: &Ray) -> V3 {
        match self.hittable.hit(&r, 0.0001, 99999.0) {
            Some(hit) => {
                let emitted = hit.material.emmit(&hit);
                match hit
                    .material
                    .scatter_with_pdf(r, &hit)
                    .and_then(|(ray, pdf)| ray.validate().map(|ray| (ray, pdf))) {
                    Some((scattered, pdf)) => {
                        let on_light: V3 = V3::new(
                            next_std_f64_in_range(213.0..343.0),
                            554.,
                            next_std_f64_in_range(227.0..332.),
                        );
                        let to_light = on_light - hit.point;
                        let dist_sqr = to_light.sqr_length();
                        let to_light_direction = to_light.unit();
                        if /*to_light_direction.dot(hit.normal) < 0.0*/ false {
                            V3::new(0.0, 1.0, 0.0)
                        } else {
                            let light_area = (343.0 - 213.0) * (332.0 - 227.0);
                            let light_cosine = to_light_direction.y.abs();
                            if light_cosine < 0.000001 {
                                V3::new(1.0, 0.0, 0.0)
                            } else {
                                let pdf = dist_sqr / (light_cosine * light_area);
                                let scattered = r.produce(hit.point, to_light_direction, scattered.attenuation);
                                emitted.0
                                    + hit.material.scattering_pdf(r, &hit, &scattered) / pdf
                                    * scattered.attenuation * self.color(&scattered)
                                // V3::new(to_light_direction.x.abs(), to_light_direction.y.abs(), to_light_direction.z.abs()) // direction to light
                                // V3::all(to_light.length() / 600.0) // distance to light
                                // V3::all(hit.material.scattering_pdf(r, &hit, &scattered)) / pdf // light without color
                                // scattered.attenuation // color without light
                            }
                        }
                    }
                    None => emitted.0
                }
            }
            None => {
                return self.miss_shader.borrow()(r);
            }
        }
    }
}

fn next_std_f64_in_range(range: Range<f64>) -> f64 {
    let offset = range.start;
    let scale = range.end - range.start;
    next_std_f64() * scale + offset
}
