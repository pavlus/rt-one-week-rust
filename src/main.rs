use structopt::StructOpt;

use types::V3;

use crate::renderer::{RendererImpl, RendererType};
use crate::sampler::SceneSampler;
use crate::scenes::*;
use crate::types::{Color, Probability};

mod consts;
mod types;
mod ray;
mod hittable;
mod onb;
mod pdf;
mod scatter;
mod camera;
#[allow(dead_code)]
mod material;
#[allow(dead_code)]
mod random;
mod aabb;
mod bvh;
mod texture;
mod noise;
mod renderer;
mod sampler;

#[allow(dead_code)]
mod scenes;
mod random2;

#[derive(Debug, StructOpt, Clone, Copy)]
enum SceneType {
    #[structopt(name = "weekend_final")]
    WeekendFinal,
    #[structopt(name = "perlin")]
    Perlin,
    #[structopt(name = "cornel_instances")]
    CornelInstances,
    #[structopt(name = "cornel_is")]
    CornelIs,
    #[structopt(name = "cornel_is_reflection")]
    CornelIsReflection,
    #[structopt(name = "cornel_volumes")]
    CornelVolumes,
    #[structopt(name = "cornel_playground")]
    CornelPlayground,
    #[structopt(name = "next_week_final")]
    NextWeekFinal,
}

#[derive(Debug, StructOpt)]
pub struct Params {
    #[structopt(subcommand)]
    scene: Option<SceneType>,
    #[structopt(short = "r", long = "renderer", default_value = "unbiased")]
    pub(crate) renderer_type: RendererType,

    #[structopt(short = "w", long = "width", default_value = "512")]
    pub(crate) width: u32,
    #[structopt(short = "h", long = "height", default_value = "512")]
    pub(crate) height: u32,
    #[structopt(short = "s", long = "samples", default_value = "400")]
    pub(crate) samples: u16,
    #[structopt(short = "b", long = "bounces", default_value = "12")]
    pub(crate) bounces: u16,
    #[structopt(short = "i", long = "important-weight", default_value = "0.5")]
    pub(crate) important_weight: Probability,

    // todo: seed random
}

fn main() {
    let params: Params = Params::from_args();
    let cfg = SceneSampler {
        width: params.width as u32,
        height: params.height as u32,
        samples: params.samples as usize,
        max_ray_bounces: params.bounces as i32,
        pixel_postprocessor: postprocess,
    };

    let renderer = RendererImpl::pick_renderer(&params);
    match params.scene.unwrap_or(SceneType::WeekendFinal) {
        SceneType::WeekendFinal => {
            cfg.do_render((weekend_final(11, 0.0..0.2, &params), renderer));
        }
        SceneType::CornelInstances => {
            cfg.do_render((cornel_box_with_instances(0.0..0.2, &params), renderer));
        }
        SceneType::CornelIs => {
            cfg.do_render((cornel_box_with_is(0.0..0.2, &params), renderer));
        }
        SceneType::CornelIsReflection => {
            cfg.do_render((cornel_box_is_reflection(0.0..0.2, &params), renderer));
        }
        SceneType::CornelVolumes => {
            cfg.do_render((cornel_box_volumes(0.0..0.2, &params), renderer));
        }
        SceneType::CornelPlayground => {
            cfg.do_render((cornel_box_test(0.0..0.2, &params), renderer));
        }
        SceneType::NextWeekFinal => {
            cfg.do_render((next_week(0.0..0.2, &params), renderer));
        }
        SceneType::Perlin => {
            cfg.do_render((perlin_scene(0.0..0.2, &params), renderer));
        }
        // _ => weekend_final(renderer_type, 11, w, h, 0.0, 0.2, ttl),
    };
//    let scene = img_scene(cfg.width, cfg.height, 0.0, 0.2, cfg.max_ray_bounces);
//    let scene = img_lit_scene(cfg.width, cfg.height, 0.0, 0.2, cfg.max_ray_bounces);
//    let scene = img_lit_rect_scene(cfg.width, cfg.height, 0.0, 0.2, cfg.max_ray_bounces);
}

pub fn postprocess(color: Color) -> Color {
    gamma(clamp(color))
}

fn clamp(color: Color) -> Color {
    Color::new(
        texture::clamp(color.x, 0.0, 1.0),
        texture::clamp(color.y, 0.0, 1.0),
        texture::clamp(color.z, 0.0, 1.0),
    )
}

fn gamma(color: Color) -> Color {
    Color::new(
        color.x.powf(1.0 / 2.2),
        color.y.powf(1.0 / 2.2),
        color.z.powf(1.0 / 2.2),
    )
}
