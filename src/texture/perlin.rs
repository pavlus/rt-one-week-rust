use std::fmt::{Debug, Error, Formatter};

use crate::types::{P3, Scale, ColorComponent, P2};

use super::Color;
use super::Texture;

pub struct PerlinTexture {
    scale: Scale,
    noise: Box<dyn Fn(&P3, Scale) -> ColorComponent + Sync + Send>,
}

pub struct RgbPerlinTexture {
    r: PerlinTexture,
    g: PerlinTexture,
    b: PerlinTexture,
}


impl Debug for PerlinTexture {
    fn fmt(&self, _f: &mut Formatter<'_>) -> Result<(), Error> {
        Err(Error)
    }
}

impl Debug for RgbPerlinTexture {
    fn fmt(&self, _f: &mut Formatter<'_>) -> Result<(), Error> {
        Err(Error)
    }
}

impl PerlinTexture {
    pub fn new(noise: Box<dyn Fn(&P3, Scale) -> ColorComponent + Sync + Send>, scale: Scale) -> PerlinTexture {
        PerlinTexture { noise, scale }
    }

    pub fn value(&self, point: &P3) -> ColorComponent {
        (self.noise)(point, self.scale)
    }
}

#[allow(dead_code)]
impl RgbPerlinTexture {
    pub fn new(r: PerlinTexture, g: PerlinTexture, b: PerlinTexture) -> RgbPerlinTexture {
        RgbPerlinTexture { r, g, b }
    }
}

impl Texture for PerlinTexture {
    fn value(&self, _: &P2, point: &P3) -> Color {
        let noise = (self.noise)(&point, self.scale);
        noise * Color::from_element(1.0)
    }
}

impl Texture for RgbPerlinTexture {
    fn value(&self, _: &P2, point: &P3) -> Color {
        Color::new(
            self.r.value(point),
            self.g.value(point),
            self.b.value(point)
        )
    }
}
