use std::ops::Deref;

use nalgebra::Unit;

use crate::consts::{FRAC_1_PI, PI};
use crate::hittable::{Hit, Hittable};
use crate::onb::ONB;
use crate::random::{next_std_f64, rand_cosine_direction, rand_in_unit_sphere};
use crate::ray::RayCtx;
use crate::types::{Color, Geometry, P3, Probability, V3};

pub trait PDF {
    fn value(&self, direction: &Unit<V3>, hit: &Hit) -> Probability;
    fn generate(&self) -> Unit<V3>;
}

#[derive(Copy, Clone)]
pub struct CosinePDF {
    onb: ONB
}

impl CosinePDF {
    pub fn from_w(w: Unit<V3>) -> Self {
        CosinePDF { onb: ONB::from_w(w) }
    }
}

impl PDF for CosinePDF {
    fn value(&self, direction: &Unit<V3>, _: &Hit) -> Probability {
        let cosine = self.onb.w.dot(&direction.normalize()) as Probability;
        if cosine < 0.0 { return 0.0; }
        if cosine >= PI as Probability { return 1.0; }
        return cosine / PI as Probability;
    }

    fn generate(&self) -> Unit<V3> {
        // todo: check if we can do unchecked
        Unit::new_normalize(self.onb.local(&rand_cosine_direction().as_ref()))
    }
}

pub struct IsotropicPDF;

impl PDF for IsotropicPDF {
    fn value(&self, _direction: &Unit<V3>, _: &Hit) -> Probability {
        0.25 * FRAC_1_PI as Probability
    }

    fn generate(&self) -> Unit<V3> {
        Unit::new_unchecked(rand_in_unit_sphere().coords)
    }
}

pub struct HittablePDF<'a> {
    origin: &'a P3,
    hittable: &'a Box<dyn Hittable>,
}

impl<'a> HittablePDF<'a> {
    pub fn new(origin: &'a P3, hittable: &'a Box<dyn Hittable>) -> Self {
        HittablePDF { origin, hittable }
    }
}

impl PDF for HittablePDF<'_> {
    fn value(&self, direction: &Unit<V3>, hit: &Hit) -> Probability {
        let tmp_ray = RayCtx::new(hit.point, *direction, Color::from_element(0.0), 0.0, 1);
        if let Some(hit) = self.hittable.hit(&tmp_ray, 0.0001, Geometry::MAX){
            self.hittable.pdf_value(&self.origin, direction, &hit)
        } else {
            0.0
        }
    }

    fn generate(&self) -> Unit<V3> {
        self.hittable.random(&self.origin)
    }
}

pub struct MixturePDF<'a, A: ?Sized, B: ?Sized> {
    a: &'a A,
    b: &'a B,
    a_weight: f64,
}

impl<'a, A: PDF + ?Sized, B: PDF + ?Sized> MixturePDF<'a, A, B> {
    pub fn new(a: &'a A, b: &'a B, a_weight: f64) -> Self {
        MixturePDF { a, b, a_weight }
    }
}

impl<A: PDF + ?Sized, B: PDF + ?Sized> PDF for MixturePDF<'_, A, B> {
    fn value(&self, direction: &Unit<V3>, hit: &Hit) -> Probability {
        let a_value = self.a.value(direction, hit);
        let b_value = self.b.value(direction, hit);
        // assert!(0.0 <= a_value && a_value <= 1.0, "a = {}", a_value);
        // assert!(0.0 <= b_value && b_value <= 1.0, "b = {}", b_value);
        let result = self.a_weight * a_value + (1.0 - self.a_weight) * b_value;
        result
    }

    fn generate(&self) -> Unit<V3> {
        if next_std_f64() < self.a_weight {
            self.a.generate()
        } else {
            self.b.generate()
        }
    }
}

impl<T: Deref<Target = dyn PDF>> PDF for T {
    fn value(&self, direction: &Unit<V3>, hit: &Hit) -> Probability {
        (**self).value(direction, hit)
    }

    fn generate(&self) -> Unit<V3> {
        (**self).generate()
    }
}
