use super::{Hittable, Ray, Renderer, V3};
use std::borrow::Borrow;
use crate::random::next_std_f64;
use std::ops::{Range, Mul, Deref};
use crate::texture::Color;
use crate::pdf::{CosinePDF, PDF, HittablePDF, MixturePDF};
use crate::hittable::XZRect;
use std::sync::Arc;
use crate::material::{DiffuseLight, Lambertian, Material};
use crate::scatter::Scatter::{Specular, Diffuse};
use std::fmt::Debug;
use rand::random;
use crate::onb::ONB;

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
                    hit.material.emmit(&hit)
                } else {
                    Color(V3::zeros())
                };
                match hit
                    .material
                    .scatter_with_pdf(r, &hit) {
                    Some(scatter) => {
                        match scatter {
                            Specular(scattered) => {
                                if let Some(valid) = scattered.validate() {
                                    let scattered_color = self.color(&valid);
                                    emitted.0 + valid.attenuation * scattered_color
                                } else {
                                    emitted.0
                                }
                            }
                            Diffuse(pdf, attenuation) => {
                                // let pdf = HittablePDF::new(hit.point, &self.important);
                                let pdf = MixturePDF::new(pdf, HittablePDF::new(hit.point, &self.important));
                                if let Some(scattered) = r.produce(
                                    hit.point,
                                    pdf.generate().unit(),
                                    attenuation.0
                                ).validate() {
                                    let pdf_value = pdf.value(&scattered.direction, &hit);
                                    let spdf = hit.material.scattering_pdf(&hit, &scattered);
                                    let mut weight = spdf / pdf_value;
                                    if weight.is_nan() {
                                        weight = 0.0
                                    } /*else if weight > 1.0 {
                                        *//*if weight > 2.0 {
                                            dbg!(weight);
                                        }*//*
                                        weight = 1.0
                                    }*/
                                    let scattered_color = self.color(&scattered);
                                    let result = emitted.0 +
                                        weight * scattered.attenuation * scattered_color;
                                    result
                                // 0.5*scattered.direction.unit() + 0.5
                                // scattered.attenuation // color without light
                                // V3::all(weight) * scattered_color // light without color
                                // V3::all(pdf_value) * scattered_color
                                //     V3::all(weight) * scattered_color
                                // V3::all(spdf) * scattered_color
                                } else {
                                    emitted.0
                                }
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
