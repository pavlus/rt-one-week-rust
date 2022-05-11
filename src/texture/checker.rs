use crate::types::{Distance, P2, P3};

use super::Color;
use super::Texture;

#[derive(Debug)]
pub struct Checker {
    odd: Color,
    even: Color,
    step: Distance,
}

impl Checker {
    pub(crate) fn new(even: Color, odd: Color, step: Distance) -> Checker {
        Checker { even, odd, step }
    }
}

impl Texture for Checker {
    fn value(&self, _uv: &P2, point: &P3) -> Color {
        let scaled = point * self.step;
        let sines = scaled.map(Distance::sin);
        let sines = sines.x * sines.y * sines.z;
        if sines < 0.0 { self.odd } else { self.even }
    }
}
