use crate::vec::V3;

use super::Color;
use super::Texture;

#[derive(Debug)]
pub struct Checker {
    odd: Color,
    even: Color,
    step: f64,
}

impl Checker {
    pub(crate) fn new(even: Color, odd: Color, step: f64) -> Checker {
        Checker { even, odd, step }
    }
}

impl Texture for Checker {
    fn value(&self, u: f64, v: f64, point: V3) -> Color {
        let sines = f64::sin(self.step * point.x)
            * f64::sin(self.step * point.y)
            * f64::sin(self.step * point.z);
        if sines < 0.0 { self.odd } else { self.even }
    }
}
