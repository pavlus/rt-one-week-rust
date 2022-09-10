use rand::distributions::{Distribution, Standard};

use crate::hittable::Hit;
use crate::random2::DefaultRng;
use crate::types::{Direction, Probability};

use super::PDF;

#[derive(Debug)]
pub struct MixturePDF<'a, A: ?Sized, B: ?Sized> {
    a: &'a A,
    b: &'a B,
    a_weight: Probability,
}

impl<'a, A: PDF + ?Sized, B: PDF + ?Sized> MixturePDF<'a, A, B> {
    pub fn new(a: &'a A, b: &'a B, a_weight: Probability) -> Self {
        MixturePDF { a, b, a_weight }
    }
}

impl<A: PDF + ?Sized, B: PDF + ?Sized> PDF for MixturePDF<'_, A, B> {
    fn value(&self, direction: &Direction, hit: &Hit) -> Probability {
        let a_value = self.a.value(direction, hit);
        let b_value = self.b.value(direction, hit);
        let result = self.a_weight * a_value + (1.0 - self.a_weight) * b_value;
        result
    }

    fn generate(&self, rng: &mut DefaultRng) -> Direction {
        if Distribution::<Probability>::sample(&Standard, rng) < self.a_weight {
            self.a.generate(rng)
        } else {
            self.b.generate(rng)
        }
    }
}
