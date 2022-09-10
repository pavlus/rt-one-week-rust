use std::path::Path;

use image::RgbImage;

use crate::types::{P3, Geometry, ColorComponent, P2};

use super::{Color, Texture};
use super::clamp;

pub struct ImageTexture;
impl ImageTexture {
    pub fn load(path: &str) -> RgbImage {
        let buffer = image::open(&Path::new(path)).unwrap();
        buffer.to_rgb8()
    }
}

impl Texture for RgbImage {
    fn value(&self, uv: &P2, _: &P3) -> Color {
        let w = self.width() as Geometry;
        let h = self.height() as Geometry;

        let i = clamp(w * uv.x, 0.0, w - 1.0);
        let j = clamp(h * (1.0 - uv.y) - 0.001, 0.0, h - 1.0);

        let color = self.get_pixel(i as u32, j as u32);
        let r = color[0] as ColorComponent / 255.0;
        let g = color[1] as ColorComponent / 255.0;
        let b = color[2] as ColorComponent / 255.0;

        Color::new(r.powf(2.2), g.powf(2.2), b.powf(2.2))
    }
}
