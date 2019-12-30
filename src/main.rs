use rayon::prelude::*;

use vec::V3;

use crate::scenes::*;

mod vec;
mod ray;
mod hittable;
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

#[allow(dead_code)]
mod scenes;

fn main() {
    let nx = 1024;
    let ny = 1024;
    let aa = 10000;
    let ttl = 8;

//    let nx = 400;
//    let ny = 400;
//    let aa = 8;
//    let ttl = 8;
    let gamma_correct = true;
    let clamp_color = true;

    println!("P3");
    println!("{} {}", nx, ny);
    println!("255");

//    let cam = cornel_box_cam(nx, ny, 0.0, 0.2, ttl);
//    let scene = rnd_scene(nx, ny, 0.0, 0.2, ttl);
    let scene = next_week(nx, ny, 0.0, 0.2, ttl);
//    let scene = perlin_scene(nx, ny, 0.0, 0.2, ttl);
//    let scene = img_scene(nx, ny, 0.0, 0.2, ttl);
//    let scene = img_lit_scene(nx, ny, 0.0, 0.2, ttl);
//    let scene = img_lit_rect_scene(nx, ny, 0.0, 0.2, ttl);
//    let scene = cornel_box_with_instances(nx, ny, 0.0, 0.2, ttl);
//    let scene = cornel_box_volumes(nx, ny, 0.0, 0.2, ttl);
//    dbg!(&renderer.hittable);
    for j in (0..ny).rev() {
        for i in 0..nx {
//            let col: V3 = (0..aa).map(|_| {
            let col: V3 = rayon::iter::repeatn((), aa).map(|_| {
                let [du, dv] = random::rand_in_unit_disc();
                let u = (i as f64 + du) / (nx as f64);
                let v = (j as f64 + dv) / (ny as f64);
                scene.color(u, v)
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
