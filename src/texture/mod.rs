use super::vec::V3;

pub mod color;
pub use color::*;

pub mod checker;
pub use checker::*;

pub mod perlin;
pub use perlin::*;

pub mod image;
pub use self::image::*;


use std::fmt::Debug;

pub trait Texture: Debug + Sync + Send {
    fn value(&self, u: f64, v: f64, point: V3) -> Color;
}

#[inline(always)]
pub fn clamp(this: f64, lo: f64, hi: f64) -> f64 {
    if this < lo { lo } else if this > hi { hi } else { this }
}


