use std::fmt::Debug;

pub use checker::*;
pub use color::*;
pub use perlin::*;

use super::vec::V3;

pub use self::image::*;

pub mod color;
pub mod checker;
pub mod perlin;
pub mod image;

pub trait Texture: Debug + Sync + Send {
    fn value(&self, u: f64, v: f64, point: V3) -> Color;
}

#[inline(always)]
pub fn clamp(this: f64, lo: f64, hi: f64) -> f64 {
    if this < lo { lo } else if this > hi { hi } else { this }
}


