use nalgebra::Unit;
use rand::distributions::{Distribution, Standard};
use rand::Rng;

use crate::{Probability, V3};
use crate::consts::{PI, TAU};
use crate::hittable::Hit;
use crate::onb::ONB;
use crate::random2::DefaultRng;
use crate::types::{Direction, Geometry};

use super::PDF;

#[derive(Copy, Clone, Debug)]
pub struct CosinePDF {
    onb: ONB,
}

impl CosinePDF {
    pub fn from_w(w: Unit<V3>) -> Self {
        CosinePDF { onb: ONB::from_w(w) }
    }
}

impl PDF for CosinePDF {
    fn value(&self, direction: &Direction, _: &Hit) -> Probability {
        let cosine = self.onb.w.dot(&direction) as Probability;
        if cosine < 0.0 { return 0.0; }
        if cosine >= PI as Probability { return 1.0; }
        return cosine / PI as Probability;
    }

    fn generate(&self, rng: &mut DefaultRng) -> Direction {
        Distribution::sample(self, rng)
    }
}

impl Distribution<Direction> for CosinePDF {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Direction {
        let r1: Geometry = Standard.sample(rng);
        let r2 = Standard.sample(rng);
        let z = Geometry::sqrt(1.0 - r2);

        let phi = r1 * TAU as Geometry;
        let (sin, cos) = Geometry::sin_cos(phi);
        let sqrt_r2 = Geometry::sqrt(r2);

        let x = cos * sqrt_r2;
        let y = sin * sqrt_r2;

        let result = V3::new(x, y, z);
        Direction::new_unchecked(self.onb.local(&result))
    }
}
