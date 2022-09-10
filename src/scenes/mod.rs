use crossbeam::atomic::AtomicConsume;
use itertools::Itertools;
use rand_distr::Normal;

pub use book::*;
pub use cornel_box::*;
pub use perlin::*;

use crate::Params;
use crate::camera::View;
use crate::hittable::{AABox, ConstantMedium, FlipNormalsOp, Hittable, Important, MovingSphere, RotateOp, Sphere, XYRect, XZRect, YZRect};
use crate::hittable::TranslateOp;
use crate::hittable::NoHit;
use crate::material::{Dielectric, DiffuseLight, Lambertian, GlossyMetal, NoMat};
use crate::noise::Perlin;
use crate::random2::DefaultRng;
use crate::ray::Ray;
use crate::renderer::{Renderer};
use crate::texture::{Checker, ImageTexture, PerlinTexture};
use crate::types::{Color, ColorComponent, Geometry, P2, P3, Scale, Timespan, V3};


pub mod perlin;
pub mod cornel_box;
pub mod book;

pub struct SceneDesc<H, I> {
    pub view: View,
    pub hittable: H,
    pub important: I,
    pub miss_shader: fn(&Ray) -> Color,
}


pub trait Scene: Send + Sync {
    fn color(&self, u: Geometry, v: Geometry, rng: &mut DefaultRng) -> Color;

    #[cfg(feature = "metrics")]
    fn generated_rays(&self) -> usize;
}

impl<H: Hittable, I: Important, R: Renderer<SceneDesc<H, I>>> Scene for (SceneDesc<H, I>, R) {
    fn color(&self, u: Geometry, v: Geometry, rng: &mut DefaultRng) -> Color {
        self.1.color(&self.0, &self.0.view.get_ray(P2::new(u, v), rng), rng)
    }

    fn generated_rays(&self) -> usize {
        self.0.view.ray_cnt.load_consume()
    }
}

pub fn get_cam(nx: u32, ny: u32, timespan: Timespan, ttl: i32) -> View {
    let aspect = (nx as Geometry) / (ny as Geometry);
    let from = V3::new(13.0, 2.0, 3.0);
    let at = V3::new(0.0, 0.0, 0.0);

    let dist_to_focus = 10.0;
    let aperture = 0.0;
    let vfov = 40.0;
    View::new_look(
        from, at,
        /*    up*/ V3::new(0.0, 1.0, 0.0),
        vfov,
        aspect,
        dist_to_focus,
        aperture,
        timespan,
        ttl,
    )
}

fn closeup_cam(nx: u32, ny: u32, timespan: Timespan, ttl: i32) -> View {
    let aspect = (nx as Geometry) / (ny as Geometry);
    let from = V3::new(-3.0, 3.0, 2.0);
    let at = V3::new(0.0, 0.0, -1.0);
    let dist_to_focus = (&from - &at).norm();
    let aperture = 0.01;
    View::new_look(
        from, at,
        /*    up*/ V3::new(0.0, 1.0, 0.0),
        /*  vfov*/ 80.0,
        aspect,
        dist_to_focus,
        aperture,
        timespan,
        ttl,
    )
}

fn sky(r: &Ray) -> Color {
    let t: Geometry = 0.5 * ((r.direction.y / r.direction.norm()) as Geometry + 1.0);
    return (1.0 - t) as ColorComponent * Color::from_element(1.0)
        + t as ColorComponent * Color::new(0.5, 0.7, 1.0);
}

fn const_color_dark(_: &Ray) -> Color { Color::new(0.05088, 0.05088, 0.05088) }

fn const_color_black(_: &Ray) -> Color { Color::new(0., 0., 0.) }

fn const_color_light(_: &Ray) -> Color { Color::new(0.3, 0.3, 0.3) }
