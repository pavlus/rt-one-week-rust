use crate::vec::V3;

use super::Texture;

#[derive(Debug, Copy, Clone)]
pub struct Color(pub V3);

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Color {
        Color(V3::new(r, g, b))
    }
    fn r(self) -> f64 { self.0.x }
    fn g(self) -> f64 { self.0.y }
    fn b(self) -> f64 { self.0.z }
}

impl Texture for Color {
    fn value(&self, _: f64, _: f64, _: V3) -> Color { *self }
}
