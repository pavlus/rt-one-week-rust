pub use rgb_renderer::RgbRenderer;
pub use rgb_renderer_unbiased::RgbRendererUnbiased;
pub use ttl_renderer::TtlRenderer;

use crate::hittable::Hittable;
use crate::ray::Ray;
use crate::vec::V3;
use std::str::FromStr;

mod rgb_renderer;
mod ttl_renderer;
mod rgb_renderer_unbiased;



#[derive(Debug)]
pub enum RendererType {
    RGBBiased,
    RGBUnbiased,
    TTL
}

impl FromStr for RendererType{
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "biased" => Result::Ok(RendererType::RGBBiased),
            "unbiased" => Result::Ok(RendererType::RGBUnbiased),
            "bounces-heatmap" => Result::Ok(RendererType::TTL),
            other => Result::Err(format!("Unknown variant: '{}'", other))
        }
    }
}

pub trait Renderer {
    fn color(&self, r: &Ray) -> V3;
}

pub enum RendererImpl {
    RGB(RgbRenderer),
    RGBUnbiased(RgbRendererUnbiased),
    TTL(TtlRenderer),
}

impl RendererImpl {
    pub fn biased(scene_graph: Box<dyn Hittable>, important: Box<dyn Hittable>, miss_shader: fn(&Ray) -> V3) -> RendererImpl{
        RendererImpl::RGB(RgbRenderer {
            hittable: scene_graph,
            important,
            miss_shader,
        })
    }
    pub fn unbiased(scene_graph: Box<dyn Hittable>, miss_shader: fn(&Ray) -> V3) -> RendererImpl{
        RendererImpl::RGBUnbiased(RgbRendererUnbiased {
            hittable: scene_graph,
            miss_shader,
        })
    }

    pub fn ray_ttl(scene_graph: Box<dyn Hittable>, ttl: i32) -> RendererImpl{
        RendererImpl::TTL(TtlRenderer {
            hittable: scene_graph,
            ttl,
        })
    }

    pub fn pick_renderer(
        renderer_type: RendererType, scene_graph: Box<dyn Hittable>,
        important: Box<dyn Hittable>,
        miss_shader: fn(&Ray) -> V3,
        ttl: i32
    ) -> RendererImpl{
        match renderer_type {
            RendererType::RGBBiased => {
                RendererImpl::biased(scene_graph, important, miss_shader)
            },
            RendererType::RGBUnbiased => {
                RendererImpl::unbiased(scene_graph, miss_shader)
            },
            RendererType::TTL => {
                RendererImpl::ray_ttl(scene_graph, ttl)
            },
        }
    }
}

impl Renderer for RendererImpl {
    fn color(&self, ray: &Ray) -> V3 {
        match self {
            RendererImpl::RGB(renderer) => renderer.color(ray),
            RendererImpl::RGBUnbiased(renderer) => renderer.color(ray),
            RendererImpl::TTL(renderer) => renderer.color(ray),
        }
    }
}

