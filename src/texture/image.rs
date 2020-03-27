use std::path::Path;

use image::RgbImage;

use crate::vec::V3;

use super::{Color, Texture};
use super::clamp;

pub struct ImageTexture;
impl ImageTexture {
    pub fn load(path: &str) -> RgbImage {
        let buffer = image::open(&Path::new(path)).unwrap();
        buffer.to_rgb()
    }
}

impl Texture for RgbImage {
    fn value(&self, u: f64, v: f64, _: V3) -> Color {
        let w = self.width() as f64;
        let h = self.height() as f64;

        let i = clamp(w * u, 0.0, w - 1.0);
        let j = clamp(h * (1.0 - v) - 0.001, 0.0, h - 1.0);

        let color = self.get_pixel(i as u32, j as u32);
        let r = color[0] as f64 / 255.0;
        let g = color[1] as f64 / 255.0;
        let b = color[2] as f64 / 255.0;

        Color(V3::new(r.powf(2.2), g.powf(2.2), b.powf(2.2)))
    }
}
