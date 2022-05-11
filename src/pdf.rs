use crate::types::{V3, P3, Distance, Color, Probability};
use crate::onb::ONB;
use crate::random::{rand_cosine_direction, next_std_f32, rand_in_unit_sphere};
use std::ops::Deref;
use crate::hittable::{Hittable, Hit};
use crate::ray::RayCtx;
use crate::consts::{FRAC_1_PI, PI};
use nalgebra::Unit;

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
        if cosine >= PI { return 1.0; }
        return cosine / PI;
    }

    fn generate(&self) -> Unit<V3> {
        // todo: check if we can do unchecked
        Unit::new_normalize(self.onb.local(&rand_cosine_direction().as_ref()))
    }
}

#[derive(Copy, Clone)]
pub struct IsotropicPDF {
    onb: ONB
}

impl IsotropicPDF {
    pub fn from_w(w: Unit<V3>) -> Self {
        IsotropicPDF { onb: ONB::from_w(w) }
    }
}

impl PDF for IsotropicPDF {
    fn value(&self, _direction: &Unit<V3>, _: &Hit) -> Probability {
        0.25 * FRAC_1_PI
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
        if let Some(hit) = self.hittable.hit(&tmp_ray, 0.0001, Distance::MAX){
            self.hittable.pdf_value(&self.origin, direction, &hit)
        } else {
            0.0
        }
    }

    fn generate(&self) -> Unit<V3> {
        self.hittable.random(&self.origin)
    }
}

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
    fn value(&self, direction: &Unit<V3>, hit: &Hit) -> Probability {
        let a_value = self.a.value(direction, hit);
        let b_value = self.b.value(direction, hit);
        let result = 0.5 * a_value + 0.5 * b_value;
        result
    }

    fn generate(&self) -> Unit<V3> {
        if next_std_f32() < 0.5 {
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
