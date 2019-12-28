use std::sync::Arc;

use rayon::prelude::*;

use camera::Camera;
use ray::Ray;
use vec::V3;

use crate::bvh::BVH;
use crate::hittable::HittableList;
use crate::renderer::{Renderer, RgbRenderer};

use crate::scenes::*;

mod vec;
mod ray;
mod hittable;
mod camera;
mod material;
mod random;
mod aabb;
mod bvh;
mod texture;
mod noise;
mod renderer;
mod scenes;

fn main() {
    let nx = 512;
    let ny = 512;
    let aa = 800;
    let ttl = 12;
    let gamma_correct = true;
    let clamp_color = true;

    println!("P3");
    println!("{} {}", nx, ny);
    println!("255");

    let cam = cornel_box_cam(nx, ny, 0.0, 0.2, ttl);
    let renderer = RgbRenderer {
//    let renderer = TtlRenderer {
//        hittable: Box::new(Stage::new(perlin_scene()))
//        hittable: Box::new(Stage::new(img_scene()))
//        hittable: Box::new(Stage::new(img_lit_scene()))
//        hittable: Box::new(Stage::new(img_lit_rect_scene()))
//        hittable: Box::new(HittableList::new(cornel_box_scene()))
        hittable: Box::new(HittableList::new(cornel_box_with_instances())),
//        hittable:&Stage::new(rnd_scene())
//        hittable: BVH::new(rnd_scene())
//        ttl
    };
//    dbg!(&renderer.hittable);
    for j in (0..ny).rev() {
        for i in 0..nx {
//            let col: V3 = (0..aa).map(|_| {
            let col: V3 = rayon::iter::repeatn((), aa).map(|_| {
                let [du, dv] = random::rand_in_unit_disc();
                let u = (i as f64 + du) / (nx as f64);
                let v = (j as f64 + dv) / (ny as f64);
                let r = cam.get_ray(u, v);
                renderer.color(&r)
            }).sum();

            let mut col = col / aa as f64;

            if clamp_color {
                col = clamp(col)
            }

            if gamma_correct {
                col = gamma(col)
            }

            let ir: u32 = (255.99 * col.x) as u32;
            let ig: u32 = (255.99 * col.y) as u32;
            let ib: u32 = (255.99 * col.z) as u32;

            assert![ir < 256];
            assert![ig < 256];
            assert![ib < 256];

            print!("{} {} {} ", ir, ig, ib);
        }
        println!();
    }
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

fn cornel_box_cam(nx: u32, ny: u32, t_off: f32, t_span: f32, ttl: i32) -> Camera {
    let aspect = (nx as f64) / (ny as f64);
    let from = V3::new(278.0, 278.0, -680.0);
    let at = V3::new(278.0, 278.0, 0.0);

    let dist_to_focus = 10.0;
    let aperture = 0.0;
    let vfov = 80.0;
    Camera::new_look(
        from, at,
        /*    up*/ V3::new(0.0, 1.0, 0.0),
        vfov,
        aspect,
        dist_to_focus,
        aperture,
        t_off, t_span,
        ttl,
    )
}

fn get_cam(nx: u32, ny: u32, t_off: f32, t_span: f32, ttl: i32) -> Camera {
    let aspect = (nx as f64) / (ny as f64);
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
        t_off, t_span,
        ttl,
    )
}

fn _get_cam(nx: u32, ny: u32, t_off: f32, t_span: f32, ttl: i32) -> Camera {
    let aspect = (nx as f64) / (ny as f64);
    let from = V3::new(-3.0, 3.0, 2.0);
    let at = V3::new(0.0, 0.0, -1.0);
    let dist_to_focus = (from - at).length();
    let aperture = 0.01;
    Camera::new_look(
        from, at,
        /*    up*/ V3::new(0.0, 1.0, 0.0),
        /*  vfov*/ 120.0,
        aspect,
        dist_to_focus,
        aperture,
        t_off, t_span,
        ttl,
    )
}
