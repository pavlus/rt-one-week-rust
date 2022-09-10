use crate::types::{Geometry, P2, P3};

use super::Color;
use super::Texture;

#[derive(Debug)]
pub struct Checker {
    odd: Color,
    even: Color,
    step: Geometry,
}

impl Checker {
    pub(crate) fn new(even: Color, odd: Color, step: Geometry) -> Checker {
        Checker { even, odd, step }
    }
}

impl Texture for Checker {
    fn value(&self, _uv: &P2, point: &P3) -> Color {
        let scaled = point * self.step;
        let sines = scaled.map(Geometry::sin);
        let sines = sines.x * sines.y * sines.z;
        if sines < 0.0 { self.odd } else { self.even }
    }
}
