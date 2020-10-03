use super::{Hittable, Ray, Renderer, V3};
use std::borrow::Borrow;
use crate::texture::Color;
use crate::pdf::{PDF, HittablePDF, MixturePDF};
use crate::scatter::Scatter::{Specular, Diffuse};
use crate::hittable::Hit;

pub struct RgbRenderer {
    pub hittable: Box<dyn Hittable>,
    pub important: Box<dyn Hittable>,
    pub miss_shader: fn(&Ray) -> V3,
}

impl Renderer for RgbRenderer {
    fn color(&self, r: &Ray) -> V3 {
        match self.hittable.hit(&r, 0.0001, 99999.0) {
            Some(hit) => {
                let emitted = if hit.normal.dot(r.direction.unit()) < 0.0 {
                    hit.material.emmit(&hit).0
                } else {
                    V3::zeros()
                };
                emitted + match hit
                    .material
                    .scatter_with_pdf(r, &hit) {
                    Some(scatter) => {
                        match scatter {
                            Specular(scattered) => {
                                self.specular(scattered)
                            }
                            Diffuse(mat_pdf, attenuation) => {
                                self.biased_diffuse(r, &hit, attenuation, mat_pdf)
                            }
                        }
                    }
                    None => V3::zeros() // no hit
                }
            }
            None => {
                self.miss_shader.borrow()(r)
            }
        }
    }
}

impl RgbRenderer {
    fn biased_diffuse<'a>(&self, r: &Ray, hit: &Hit, attenuation: Color, mat_pdf: Box<dyn PDF>) -> V3 {
        let mat_dir = mat_pdf.generate();  // unbiased sample, just in case we need it
        let pdf = MixturePDF::new(
            mat_pdf,
            HittablePDF::new(hit.point, &self.important)
        );
        if let Some(mut scattered) = r.produce(
            hit.point,
            pdf.generate().unit(),
            attenuation.0,
        ).validate() {
            let pdf_value = pdf.value(&scattered.direction, &hit);
            let spdf = hit.material.scattering_pdf(&hit, &scattered.direction);
            let mut weight = spdf / pdf_value;
            if weight.is_nan() {
                // coin toss of mixture PDF gave us ray from non-overlapping part of importance PDF,
                // and weighted probability of hitting that important object is zero too or NaN,
                // so we get NaN weight. Let's scatter light unbiased, by material PDF, this will
                // also give us pdf_value = spdf, since they are from same material, so weight is 1.
                weight = 1.0;
                scattered.direction = mat_dir;
            }
            let scattered_color = self.color(&scattered);
            // let scattered_color = 0.5 * scattered.direction.unit() + 0.5; // scatter direction
            // let scattered_color = 0.5 * hit.normal + 0.5; // surface normal vectors
            // scattered.attenuation // color without weight
            // V3::all(weight) * scattered_color // weight without color
            // V3::new(weight, spdf, pdf_value) // neon party for debugging probability density
            weight * scattered.attenuation * scattered_color
        } else {
            V3::zeros() // max depth
        }
    }
}

impl RgbRenderer {
    fn specular(&self, scattered: Ray) -> V3 {
        if let Some(valid) = scattered.validate() {
            let scattered_color = self.color(&valid);
            valid.attenuation * scattered_color
        } else {
            V3::zeros() // max depth
        }
    }
}
