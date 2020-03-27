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

pub enum RendererImpl {
    RGB(RgbRenderer),
    TTL(TtlRenderer),
}

impl Renderer for RendererImpl {
    fn color(&self, ray: &Ray) -> V3 {
        match (self) {
            RendererImpl::RGB(renderer) => renderer.color(ray),
            RendererImpl::TTL(renderer) => renderer.color(ray),
        }
    }
}
