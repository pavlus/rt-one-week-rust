use super::{Hittable, RayCtx, Renderer};
use std::borrow::Borrow;
use crate::pdf::{PDF, HittablePDF, MixturePDF};
use crate::scatter::Scatter::{Specular, Diffuse};
use crate::hittable::Hit;
use crate::ray::Ray;
use crate::types::{Color, ColorComponent, Probability};

pub struct RgbRenderer {
    pub hittable: Box<dyn Hittable>,
    pub important: Box<dyn Hittable>,
    pub miss_shader: fn(&Ray) -> Color,
    pub important_weight: Probability,
}

impl Renderer for RgbRenderer {
    fn color(&self, ray_ctx: &RayCtx) -> Color {
        self.hittable.hit(&ray_ctx, 0.000_001, 99999.0)
            .map(|hit|{

                let emitted = if hit.normal.dot(&ray_ctx.ray.direction.normalize()) < 0.0 {
                    hit.material.emmit(&hit)
                } else {
                    Color::from_element(0.0)
                };
                if let Some(reflected) = hit
                    .material
                    .scatter_with_pdf(ray_ctx, &hit)
                    .map(|scatter| {
                        match scatter {
                            Specular(scattered) => {
                                self.specular(scattered)
                            }
                            Diffuse(mat_pdf, attenuation) => {
                                self.biased_diffuse(ray_ctx, &hit, attenuation, mat_pdf)
                            }
                        }
                    }) {
                    reflected + emitted
                } else {
                    emitted
                }
            }).unwrap_or(self.miss_shader.borrow()(&ray_ctx.ray))
        }
}

impl RgbRenderer {
    fn biased_diffuse<'a>(&self, ray_ctx: &RayCtx, hit: &Hit, attenuation: Color, mat_pdf: Box<dyn PDF>) -> Color {
        let mat_dir = mat_pdf.generate();  // unbiased sample, just in case we need it
        let hittable_pdf = HittablePDF::new(&hit.point, &self.important);
        let pdf = MixturePDF::new(
            &hittable_pdf,
            mat_pdf.as_ref(),
            self.important_weight
        );
        if let Some(mut scattered) = ray_ctx.produce(
            hit.point,
            pdf.generate(),
            attenuation,
        ).validate() {
            let pdf_value = pdf.value(&scattered.ray.direction, &hit);
            let spdf = mat_pdf.value(&scattered.ray.direction, &hit);
            let mut weight = spdf / pdf_value;
            if weight.is_nan() || weight.is_infinite() {
                // coin toss of mixture PDF gave us ray from non-overlapping part of importance PDF,
                // and weighted probability of hitting that important object is zero too or NaN,
                // so we get NaN weight. Let's scatter light unbiased, by material PDF, this will
                // also give us pdf_value = spdf, since they are from same material, so weight is 1.
                weight = 1.0;
                scattered.ray.direction = mat_dir;
            }
            let scattered_color = self.color(&scattered);
            // let scattered_color = 0.5 * scattered.direction.normalize() + 0.5; // scatter direction
            // let scattered_color = 0.5 * hit.normal + 0.5; // surface normal vectors
            // scattered.attenuation // color without weight
            // V3::all(weight) * scattered_color // weight without color
            // V3::new(weight, spdf, pdf_value) // neon party for debugging probability density
            // V3::from_element(weight) // contribution
            // weight
            weight as ColorComponent
                * scattered.attenuation.component_mul(&scattered_color)
        } else {
            Color::from_element(0.0) // max depth
        }
    }
}

impl RgbRenderer {
    fn specular(&self, scattered: RayCtx) -> Color {
        if let Some(valid) = scattered.validate() {
            let scattered_color = self.color(&valid);
            valid.attenuation.component_mul(&scattered_color)
        } else {
            Color::from_element(0.0) // max depth
        }
    }
}
