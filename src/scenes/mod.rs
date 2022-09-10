use itertools::Itertools;
use rand_distr::Normal;

use crate::hittable::{AABox, ConstantMedium, Hittable, HittableList, RotateYOp, FlipNormalsOp, TranslateOp, MovingSphere, Sphere, XYRect, XZRect, YZRect, NoHit, Translate};
use crate::material::{Dielectric, DiffuseLight, Lambertian, Metal, NoMat};
use crate::noise::Perlin;
use crate::random::{next_color, with_rnd, next_std_u32, next_std, next_std_distance};
use crate::texture::{Checker, ImageTexture, PerlinTexture};
use crate::camera::Camera;
use crate::types::{Color, ColorComponent, Geometry, P2, P3, Scale, Time, V3};
use crate::renderer::{Renderer, RendererImpl};
use crate::ray::Ray;
use crate::bvh::BVH;
use crate::{Params, random};

pub mod perlin;
pub use perlin::*;

pub mod cornel_box;
pub use cornel_box::*;

pub mod image;
pub use self::image::*;

pub mod book;
pub use book::*;

pub struct Scene {
    pub camera: Camera,
    pub renderer: RendererImpl,
}

impl Scene {
    pub fn color(&self, u: Geometry, v: Geometry) -> Color {
        self.renderer.color(&self.camera.get_ray(P2::new(u, v)))
    }
}

fn get_cam(nx: u32, ny: u32, t_off: Time, t_span: Time, ttl: i32) -> Camera {
    let aspect = (nx as Geometry) / (ny as Geometry);
    let from = V3::new(13.0, 2.0, 3.0);
    let at = V3::new(0.0, 0.0, 0.0);

    let dist_to_focus = 10.0;
    let aperture = 0.0;
    let vfov = 40.0;
    Camera::new_look(
        from, at,
        /*    up*/ V3::new(0.0, 1.0, 0.0),
        vfov,
        aspect,
        dist_to_focus,
        aperture,
        t_off..t_span,
        ttl,
    )
}

fn closeup_cam(nx: u32, ny: u32, t_off: Time, t_span: Time, ttl: i32) -> Camera {
    let aspect = (nx as Geometry) / (ny as Geometry);
    let from = V3::new(-3.0, 3.0, 2.0);
    let at = V3::new(0.0, 0.0, -1.0);
    let dist_to_focus = (&from - &at).norm();
    let aperture = 0.01;
    Camera::new_look(
        from, at,
        /*    up*/ V3::new(0.0, 1.0, 0.0),
        /*  vfov*/ 80.0,
        aspect,
        dist_to_focus,
        aperture,
        t_off..t_span,
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
