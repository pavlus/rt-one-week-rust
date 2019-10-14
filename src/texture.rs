use crate::vec::V3;
use crate::noise::Perlin;
use std::fmt::{Debug, Formatter, Error};

#[derive(Debug, Copy, Clone)]
pub struct Color(pub V3);

pub trait Texture: Debug {
    fn value(&self, u: f64, v: f64, point: V3) -> Color;
}


impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Color {
        Color(V3::new(r, g, b))
    }
    fn r(self) -> f64 { self.0.x }
    fn g(self) -> f64 { self.0.y }
    fn b(self) -> f64 { self.0.z }
}

impl Texture for Color {
    fn value(&self, u: f64, v: f64, point: V3) -> Color { *self }
}

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

pub struct PerlinTexture {
    noise: Box<dyn Fn(V3, f64)->f64>,
    scale: f64,
}

impl Debug for PerlinTexture {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        Err(Error)
    }
}

impl PerlinTexture {
    pub fn new(noise: Box<dyn Fn(V3, f64)->f64>, scale: f64) -> PerlinTexture {
        PerlinTexture { noise , scale}
    }
}

impl Texture for PerlinTexture {
    fn value(&self, u: f64, v: f64, point: V3) -> Color {
        let noise =((self.noise)(point, self.scale));
        assert!(noise<=1.0);
        assert!(noise>=0.0);
        Color(noise * V3::ones())
    }
}
