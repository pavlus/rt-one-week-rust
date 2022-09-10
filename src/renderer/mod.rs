pub use monte_carlo_importance::MonteCarlo;
pub use monte_carlo_brure_force::BruteForce;

use crate::hittable::{Hittable, Important};
use crate::ray::{RayCtx};
use crate::types::{Color, Probability};
use std::str::FromStr;
use crate::{Params, SceneDesc};
use crate::random2::DefaultRng;

mod monte_carlo_brure_force;
mod monte_carlo_importance;


#[derive(Debug)]
pub enum RendererType {
    RGBBiased,
    RGBUnbiased,
}

impl FromStr for RendererType{
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "biased" => Result::Ok(RendererType::RGBBiased),
            "unbiased" => Result::Ok(RendererType::RGBUnbiased),
            other => Result::Err(format!("Unknown variant: '{}'", other))
        }
    }
}

pub trait Renderer<S>: Sync + Send {
    fn color(&self, scene: &S, r: &RayCtx, rng: &mut DefaultRng) -> Color;
}

pub enum RendererImpl {
    RGB(MonteCarlo),
    RGBUnbiased(BruteForce),
}

impl RendererImpl {
    pub fn biased(important_weight: Probability) -> Self {
        RendererImpl::RGB(MonteCarlo { important_weight})
    }
    pub fn unbiased() -> Self {
        RendererImpl::RGBUnbiased(BruteForce)
    }

    pub fn pick_renderer(params: &Params) -> Self {
        match params.renderer_type {
            RendererType::RGBBiased => {
                RendererImpl::biased(params.important_weight)
            },
            RendererType::RGBUnbiased => {
                RendererImpl::unbiased()
            }
        }
    }
}

impl<H: Hittable, I: Hittable + Important> Renderer<SceneDesc<H, I>> for RendererImpl {
    fn color(&self, scene: &SceneDesc<H, I>, ray_ctx: &RayCtx, rng: &mut DefaultRng) -> Color {
        match self {
            RendererImpl::RGB(renderer) => renderer.color(scene, ray_ctx, rng),
            RendererImpl::RGBUnbiased(renderer) => renderer.color(scene, ray_ctx, rng),
        }
    }
}

