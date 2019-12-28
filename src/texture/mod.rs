use super::vec::V3;
use crate::noise::Perlin;
use std::fmt::{Debug, Formatter, Error};
use image::RgbImage;
use std::path::Path;

pub mod color;
pub use color::*;

pub mod checker;
pub use checker::*;

pub mod perlin;
pub use perlin::*;

pub trait Texture: Debug + Sync + Send {
    fn value(&self, u: f64, v: f64, point: V3) -> Color;
}

#[derive(Debug)]
pub struct ImageTexture {
    buffer: Box<RgbImage>
}

impl ImageTexture {
    pub fn load(path: &str) -> ImageTexture {
        let buffer = image::open(&Path::new(path)).unwrap();
        let rgb = buffer.to_rgb();
        ImageTexture {
            buffer: Box::new(rgb)
        }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, point: V3) -> Color {
        let w = self.buffer.width() as f64;
        let h = self.buffer.height() as f64;

        let i = clamp(w * u, 0.0, w - 1.0);
        let j = clamp((h * (1.0 - v) - 0.001), 0.0, h - 1.0);

        let color = self.buffer.get_pixel(i as u32, j as u32);
        let r = color[0] as f64 / 255.0;
        let g = color[1] as f64 / 255.0;
        let b = color[2] as f64 / 255.0;

        Color(V3::new(r.powf(2.2), g.powf(2.2), b.powf(2.2)))
    }
}

#[inline(always)]
pub fn clamp(this: f64, lo: f64, hi: f64) -> f64 {
    if this < lo { lo } else if this > hi { hi } else { this }
}


