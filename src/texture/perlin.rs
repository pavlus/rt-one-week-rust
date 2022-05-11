use std::fmt::{Debug, Error, Formatter};

use crate::types::{P3, Distance, Scale, ColorComponent, P2};

use super::Color;
use super::Texture;

pub struct PerlinTexture {
    noise: Box<dyn Fn(&P3, Scale) -> ColorComponent + Sync + Send>,
    scale: Scale,
}

impl Debug for PerlinTexture {
    fn fmt(&self, _f: &mut Formatter<'_>) -> Result<(), Error> {
        Err(Error)
    }
}

impl PerlinTexture {
    pub fn new(noise: Box<dyn Fn(&P3, Scale) -> ColorComponent + Sync + Send>, scale: Scale) -> PerlinTexture {
        PerlinTexture { noise, scale }
    }
}

impl Texture for PerlinTexture {
    fn value(&self, _: &P2, point: &P3) -> Color {
        let noise = (self.noise)(&point, self.scale);
        debug_assert!(noise <= 1.0);
        debug_assert!(noise >= 0.0);
        noise * Color::from_element(1.0)
    }
}
