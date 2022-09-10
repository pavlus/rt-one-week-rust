use rand::distributions::Distribution;
use rand_distr::UnitSphere;

use crate::consts::FRAC_1_PI;
use crate::hittable::Hit;
use crate::random2::DefaultRng;
use crate::types::{Direction, Probability};

use super::PDF;

#[derive(Debug)]
pub struct IsotropicPDF;

impl PDF for IsotropicPDF {
    fn value(&self, _direction: &Direction, _: &Hit) -> Probability {
        0.25 * FRAC_1_PI as Probability
    }

    fn generate(&self, rng: &mut DefaultRng) -> Direction {
        Direction::new_unchecked(UnitSphere.sample(rng).into())
    }
}
