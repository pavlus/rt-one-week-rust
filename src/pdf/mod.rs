use std::fmt::Debug;
use std::ops::Deref;

use crate::hittable::Hit;
use crate::random2::DefaultRng;
use crate::types::{Direction, Probability};

mod cosine;
mod isotropic;
mod hittable;
mod mixture;

pub use cosine::*;
pub use isotropic::*;
pub use hittable::*;
pub use mixture::*;

pub trait PDF: Debug {
    fn value(&self, direction: &Direction, hit: &Hit) -> Probability;
    fn generate(&self, rng: &mut DefaultRng) -> Direction;
}

impl<T: Deref<Target=dyn PDF> + Debug> PDF for T {
    fn value(&self, direction: &Direction, hit: &Hit) -> Probability {
        (**self).value(direction, hit)
    }

    fn generate(&self, rng: &mut DefaultRng) -> Direction {
        (**self).generate(rng)
    }
}
