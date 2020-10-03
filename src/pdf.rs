use crate::vec::V3;
use crate::onb::ONB;
use crate::random::{rand_cosine_direction, next_std_f32, rand_in_unit_sphere};
use std::ops::Deref;
use crate::hittable::{Hittable, Hit};
use std::fmt::Debug;
use crate::ray::Ray;
use core::f64::consts::PI;
use std::f64::consts;

pub trait PDF: Debug {
    fn value(&self, direction: &V3, hit: &Hit) -> f64;
    fn generate(&self) -> V3;
}

#[derive(Debug, Copy, Clone)]
pub struct CosinePDF {
    onb: ONB
}

impl CosinePDF {
    pub fn from_w(w: &V3) -> Self {
        CosinePDF { onb: ONB::from_w(w) }
    }
}

impl PDF for CosinePDF {
    fn value(&self, direction: &V3, _: &Hit) -> f64 {
        let cosine = self.onb.w.dot(direction.unit());
        if cosine < 0.0 { 0.0 } else if cosine >= PI { 1.0 } else { cosine / PI }
    }

    fn generate(&self) -> V3 {
        self.onb.local(rand_cosine_direction())
    }
}

#[derive(Debug, Copy, Clone)]
pub struct IsotropicPDF {
    onb: ONB
}

impl IsotropicPDF {
    pub fn from_w(w: &V3) -> Self {
        IsotropicPDF { onb: ONB::from_w(w) }
    }
}

impl PDF for IsotropicPDF {
    fn value(&self, _direction: &V3, _: &Hit) -> f64 {
        0.25 * consts::FRAC_1_PI
    }

    fn generate(&self) -> V3 {
        rand_in_unit_sphere()
    }
}

#[derive(Debug)]
pub struct HittablePDF<'a> {
    origin: V3,
    hittable: &'a Box<dyn Hittable>,
}

impl<'a> HittablePDF<'a> {
    pub fn new(origin: V3, hittable: &'a Box<dyn Hittable>) -> Self {
        HittablePDF { origin, hittable }
    }
}

impl PDF for HittablePDF<'_> {
    fn value(&self, direction: &V3, hit: &Hit) -> f64 {
        let tmp_ray = Ray::new(hit.point, *direction, V3::zeros(), 0.0, 1);
        if let Some(hit) = self.hittable.hit(&tmp_ray, 0.0001, f64::MAX){
            self.hittable.pdf_value(&self.origin, direction, &hit)
        } else {
            0.0
        }
    }

    fn generate(&self) -> V3 {
        self.hittable.random(&self.origin)
    }
}

#[derive(Debug)]
pub struct MixturePDF<A, B> {
    a: A,
    b: B,
}

impl<A: PDF, B: PDF> MixturePDF<A, B> {
    pub fn new(a: A, b: B) -> Self {
        MixturePDF { a, b }
    }
}

impl<A: PDF, B: PDF> PDF for MixturePDF<A, B> {
    fn value(&self, direction: &V3, hit: &Hit) -> f64 {
        let a_value = self.a.value(direction, hit);
        let b_value = self.b.value(direction, hit);
        let result = 0.5 * a_value + 0.5 * b_value;
        result
    }

    fn generate(&self) -> V3 {
        if next_std_f32() < 0.5 {
            self.a.generate()
        } else {
            self.b.generate()
        }
    }
}

impl<T: Deref<Target = dyn PDF>> PDF for T where T: Debug {
    fn value(&self, direction: &V3, hit: &Hit) -> f64 {
        (**self).value(direction, hit)
    }

    fn generate(&self) -> V3 {
        (**self).generate()
    }
}
