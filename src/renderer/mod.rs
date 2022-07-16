pub use rgb_renderer::RgbRenderer;
pub use rgb_renderer_unbiased::RgbRendererUnbiased;
pub use ttl_renderer::TtlRenderer;

use crate::hittable::Hittable;
use crate::ray::{RayCtx, Ray};
use crate::types::{Color, Probability};
use std::str::FromStr;
use crate::Params;

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
    fn color(&self, r: &RayCtx) -> Color;
}

pub enum RendererImpl {
    RGB(RgbRenderer),
    RGBUnbiased(RgbRendererUnbiased),
    TTL(TtlRenderer),
}

impl RendererImpl {
    pub fn biased(scene_graph: Box<dyn Hittable>, important: Box<dyn Hittable>, miss_shader: fn(&Ray) -> Color, important_weight: Probability) -> RendererImpl {
        RendererImpl::RGB(RgbRenderer {
            hittable: scene_graph,
            important,
            miss_shader,
            important_weight,
        })
    }
    pub fn unbiased(scene_graph: Box<dyn Hittable>, miss_shader: fn(&Ray) -> Color) -> RendererImpl {
        RendererImpl::RGBUnbiased(RgbRendererUnbiased {
            hittable: scene_graph,
            miss_shader,
        })
    }

    pub fn ray_ttl(scene_graph: Box<dyn Hittable>, ttl: i32) -> RendererImpl {
        RendererImpl::TTL(TtlRenderer {
            hittable: scene_graph,
            ttl,
        })
    }

    pub fn pick_renderer(
        scene_graph: Box<dyn Hittable>,
        important: Box<dyn Hittable>,
        miss_shader: fn(&Ray) -> Color,
        params: &Params
    ) -> RendererImpl {
        match params.renderer_type {
            RendererType::RGBBiased => {
                RendererImpl::biased(scene_graph, important, miss_shader, params.important_weight)
            },
            RendererType::RGBUnbiased => {
                RendererImpl::unbiased(scene_graph, miss_shader)
            }
            RendererType::TTL => {
                RendererImpl::ray_ttl(scene_graph, params.bounces as i32)
            }
        }
    }
}

impl Renderer for RendererImpl {
    fn color(&self, ray_ctx: &RayCtx) -> Color {
        match self {
            RendererImpl::RGB(renderer) => renderer.color(ray_ctx),
            RendererImpl::RGBUnbiased(renderer) => renderer.color(ray_ctx),
            RendererImpl::TTL(renderer) => renderer.color(ray_ctx),
        }
    }
}

