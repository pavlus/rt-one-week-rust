pub use rgb_renderer::RgbRenderer;
pub use ttl_renderer::TtlRenderer;

use crate::hittable::Hittable;
use crate::ray::Ray;
use crate::vec::V3;

mod rgb_renderer;
mod ttl_renderer;

pub trait Renderer {
    fn color(&self, r: &Ray) -> V3;
}
