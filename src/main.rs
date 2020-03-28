use structopt::StructOpt;

use vec::V3;

use crate::renderer::Renderer;
use crate::sampler::Sampler;
use crate::scenes::*;

mod vec;
mod ray;
mod hittable;
mod onb;
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

#[derive(Debug, StructOpt)]
enum SceneType {
    #[structopt(name = "weekend_final")]
    WeekendFinal,
    #[structopt(name = "perlin")]
    Perlin,
    #[structopt(name = "cornel_instances")]
    CornelInstances,
    #[structopt(name = "cornel_is")]
    CornelIs,
    #[structopt(name = "cornel_volumes")]
    CornelVolumes,
    #[structopt(name = "next_week_final")]
    NextWeekFinal,
}

#[derive(Debug, StructOpt)]
struct Params {
    #[structopt(subcommand)]
    scene: Option<SceneType>,

    #[structopt(short = "w", long = "width", default_value = "512")]
    width: u16,
    #[structopt(short = "h", long = "height", default_value = "512")]
    height: u16,
    #[structopt(short = "s", long = "samples", default_value = "400")]
    samples: u16,
    #[structopt(short = "b", long = "bounces", default_value = "12")]
    bounces: u16,
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
        SceneType::WeekendFinal => weekend_final(11, cfg.width, cfg.height, 0.0, 0.2, cfg.max_ray_bounces),
        SceneType::CornelInstances => cornel_box_with_instances(cfg.width, cfg.height, 0.0, 0.2, cfg.max_ray_bounces),
        SceneType::CornelIs => cornel_box_with_is(cfg.width, cfg.height, 0.0, 0.2, cfg.max_ray_bounces),
        SceneType::CornelVolumes => cornel_box_volumes(cfg.width, cfg.height, 0.0, 0.2, cfg.max_ray_bounces),
        SceneType::NextWeekFinal => next_week(cfg.width, cfg.height, 0.0, 0.2, cfg.max_ray_bounces),
        SceneType::Perlin => perlin_scene(cfg.width, cfg.height, 0.0, 0.2, cfg.max_ray_bounces),
    };
//    let scene = img_scene(cfg.width, cfg.height, 0.0, 0.2, cfg.max_ray_bounces);
//    let scene = img_lit_scene(cfg.width, cfg.height, 0.0, 0.2, cfg.max_ray_bounces);
//    let scene = img_lit_rect_scene(cfg.width, cfg.height, 0.0, 0.2, cfg.max_ray_bounces);

    cfg.do_render(scene);
}

pub fn postprocess(color: V3) -> V3 {
    gamma(clamp(color))
}

fn clamp(color: V3) -> V3 {
    V3::new(
        texture::clamp(color.x, 0.0, 1.0),
        texture::clamp(color.y, 0.0, 1.0),
        texture::clamp(color.z, 0.0, 1.0),
    )
}

fn _clamp(color: V3) -> V3 {
    let max = f64::max(color.x, f64::max(color.y, color.z));
    if max > 1.0 { color / max } else { color }
}

fn gamma(color: V3) -> V3 {
    V3::new(
        color.x.powf(1.0 / 2.2),
        color.y.powf(1.0 / 2.2),
        color.z.powf(1.0 / 2.2),
    )
}
