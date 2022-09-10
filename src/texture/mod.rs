use std::fmt::Debug;
pub use checker::*;
pub use color::*;
pub use perlin::*;

pub use self::image::*;
use crate::types::{P3, Color, P2};

pub mod color;
pub mod checker;
pub mod perlin;
pub mod image;

pub trait Texture: Sync + Send + Debug {
    fn value(&self, uv: &P2, point: &P3) -> Color;
}

#[inline(always)]
pub fn clamp<T: PartialOrd>(this: T, lo: T, hi: T) -> T {
    if this < lo { lo } else if this > hi { hi } else { this }
}


