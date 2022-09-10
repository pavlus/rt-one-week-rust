use structopt::StructOpt;

use types::V3;

use crate::renderer::RendererType;
use crate::sampler::Sampler;
use crate::scenes::*;
use crate::types::Color;

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
    pub(crate) important_weight: f64,
}

fn main() {
    let params: Params = Params::from_args();
    let cfg = Sampler {
        width: params.width as u32,
        height: params.height as u32,
        samples: params.samples as usize,
        max_ray_bounces: params.bounces as i32,
        pixel_postprocessor: crate::postprocess,
    };


    let scene: Scene = match params.scene.unwrap_or(SceneType::WeekendFinal) {
        SceneType::WeekendFinal => weekend_final(11, 0.0, 0.2, &params),
        SceneType::CornelInstances => cornel_box_with_instances(0.0, 0.2, &params),
        SceneType::CornelIs => cornel_box_with_is(0.0, 0.2, &params),
        SceneType::CornelIsReflection => cornel_box_is_reflection(0.0, 0.2, &params),
        SceneType::CornelVolumes => cornel_box_volumes(0.0, 0.2, &params),
        SceneType::NextWeekFinal => next_week(0.0, 0.2, &params),
        SceneType::Perlin => perlin_scene(0.0, 0.2, &params),
        // _ => weekend_final(renderer_type, 11, w, h, 0.0, 0.2, ttl),

    };
//    let scene = img_scene(cfg.width, cfg.height, 0.0, 0.2, cfg.max_ray_bounces);
//    let scene = img_lit_scene(cfg.width, cfg.height, 0.0, 0.2, cfg.max_ray_bounces);
//    let scene = img_lit_rect_scene(cfg.width, cfg.height, 0.0, 0.2, cfg.max_ray_bounces);

    cfg.do_render(scene);
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
