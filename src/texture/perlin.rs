use std::fmt::{Debug, Error, Formatter};

use crate::vec::V3;

use super::Color;
use super::Texture;

pub struct PerlinTexture {
    noise: Box<dyn Fn(V3, f64) -> f64 + Sync + Send>,
    scale: f64,
}

impl Debug for PerlinTexture {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        Err(Error)
    }
}

impl PerlinTexture {
    pub fn new(noise: Box<dyn Fn(V3, f64) -> f64 + Sync + Send>, scale: f64) -> PerlinTexture {
        PerlinTexture { noise, scale }
    }
}

impl Texture for PerlinTexture {
    fn value(&self, u: f64, v: f64, point: V3) -> Color {
        let noise = (self.noise)(point, self.scale);
        assert!(noise <= 1.0);
        assert!(noise >= 0.0);
        Color(noise * V3::ones())
    }
}