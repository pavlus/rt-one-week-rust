use crate::types::{P3, Distance, Scale};

use super::Color;
use super::Texture;

#[derive(Debug)]
pub struct Checker {
    odd: Color,
    even: Color,
    step: Scale,
}

impl Checker {
    pub(crate) fn new(even: Color, odd: Color, step: Scale) -> Checker {
        Checker { even, odd, step }
    }
}

impl Texture for Checker {
    fn value(&self, _u: Distance, _v: Distance, point: &P3) -> Color {
        let sines = Scale::sin(self.step * point.x)
            * Scale::sin(self.step * point.y)
            * Scale::sin(self.step * point.z);
        if sines < 0.0 { self.odd } else { self.even }
    }
}
