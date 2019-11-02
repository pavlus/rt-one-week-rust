use camera::Camera;
use ray::Ray;
use vec::V3;

use crate::hittable::{Hittable, MovingSphere, Sphere, HittableList, XYRect, XZRect, YZRect, FlipNormals, AABox};
use crate::material::{Dielectric, Lambertian, Metal, DiffuseLight, Material};
use crate::random::{next_color, next_std_f64};
use crate::bvh::BVH;
use crate::texture::{Color, Checker, PerlinTexture, ImageTexture};
use crate::noise::Perlin;
use std::path::Path;
use rayon::prelude::*;
use std::sync::Arc;

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

fn main() {
    let nx = 400;
    let ny = 400;
    let aa = 800;

    println!("P3");
    println!("{} {}", nx, ny);
    println!("255");

    let cam = cornel_box_cam(nx, ny, 0.0, 0.2, 32);
    let renderer = Renderer {
//        hittable: Box::new(Stage::new(perlin_scene()))
//        hittable: Box::new(Stage::new(img_scene()))
//        hittable: Box::new(Stage::new(img_lit_scene()))
//        hittable: Box::new(Stage::new(img_lit_rect_scene()))
//        hittable: Box::new(HittableList::new(cornel_box_scene()))
        hittable: Box::new(HittableList::new(cornel_box_scene_with_instances()))
//        hittable:&Stage::new(rnd_scene())
//        hittable: BVH::new(rnd_scene())
    };
//    dbg!(&renderer.hittable);
    for j in (0..ny).rev() {
        for i in 0..nx {
            let col: V3 = rayon::iter::repeatn((), aa).map(|_|{
//                let du: sf64 = dist.sample(&mut rand);
//                let dv: f64 = dist.sample(&mut rand);
                let du: f64 = random::next_std_f64() - 0.5;
                let dv: f64 = random::next_std_f64() - 0.5;
                let u = (i as f64 + du) / (nx as f64);
                let v = (j as f64 + dv) / (ny as f64);
                let r = cam.get_ray(u, v);
                renderer.color(&r)
            }).sum();

            let mut col = col / aa as f64;
            let gamma_correct = true;
            let clamp_color = true;

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

fn gamma(color: V3) -> V3 {
    V3::new(
        color.x.powf(1.0 / 2.2),
        color.y.powf(1.0 / 2.2),
        color.z.powf(1.0 / 2.2),
    )
}

fn perlin_scene() -> Vec<Box<dyn Hittable>> {
    let mut objs: Vec<Box<dyn Hittable>> = Vec::new();
    let perlin = random::with_rnd(|rnd| Perlin::new(rnd));
    objs.push(Box::new(Sphere::new(V3::new(0.0, -1000.0, 0.0), 1000.0, Box::new(
        Lambertian::texture(Box::new(PerlinTexture::new(
            Box::new(move |p, scale| perlin.noise(scale * p) * 0.5 + 0.5), 4.0,
        )))))));
    objs.push(Box::new(Sphere::new(V3::new(0.0, 2.0, 0.0), 2.0, Box::new(
        Lambertian::texture(Box::new(PerlinTexture::new(
            Box::new(move |p, scale| perlin.turb(scale * p)), 4.0,
        )))))));
    objs.push(Box::new(Sphere::new(V3::new(0.0, 2.0, 4.0), 2.0, Box::new(
        Lambertian::texture(Box::new(PerlinTexture::new(
            Box::new(move |p, scale| 0.5 * (1.0 + perlin.turb(scale * p))), 4.0,
        )))))));
    objs.push(Box::new(Sphere::new(V3::new(0.0, 2.0, -4.0), 2.0, Box::new(
        Lambertian::texture(Box::new(PerlinTexture::new(
            Box::new(move |p, scale| 0.5 * (1.0 + (scale * p.z + 10.0 * perlin.turb(p)).sin())), 5.0,
        )))))));
    objs
}

fn img_scene() -> Vec<Box<dyn Hittable>> {
    let mut objs: Vec<Box<dyn Hittable>> = Vec::new();
    objs.push(Box::new(Sphere::new(V3::new(0.0, -1000.0, 0.0), 1000.0, Box::new(
        Lambertian::texture(Box::new(Checker::new(Color::new(0.0, 0.0, 0.0), Color::new(1.0, 1.0, 1.0), 10.0)))))));
    objs.push(Box::new(Sphere::new(V3::new(0.0, 2.0, 0.0), 2.0, Box::new(
        Lambertian::texture(Box::new(ImageTexture::load("./textures/stone.png")))))));
    objs
}

fn img_lit_scene() -> Vec<Box<dyn Hittable>> {
    let mut objs: Vec<Box<dyn Hittable>> = Vec::new();
    objs.push(Box::new(Sphere::new(V3::new(0.0, -1000.0, 0.0), 1000.0, Box::new(
        Lambertian::texture(Box::new(Checker::new(Color::new(0.0, 0.0, 0.0), Color::new(1.0, 1.0, 1.0), 10.0)))))));
    objs.push(Box::new(Sphere::new(V3::new(0.0, 2.0, 2.0), 2.0, Box::new(
        Lambertian::texture(Box::new(ImageTexture::load("./textures/stone.png")))))));
    objs.push(Box::new(Sphere::new(V3::new(0.0, 3.0, -2.0), 2.0, Box::new(
        DiffuseLight::new(Box::new(Color::new(1.0, 1.0, 0.99)), 2.0)))));
    objs
}

fn img_lit_rect_scene() -> Vec<Box<dyn Hittable>> {
    let mut objs: Vec<Box<dyn Hittable>> = Vec::new();
    objs.push(Box::new(Sphere::new(V3::new(0.0, -1000.0, 0.0), 1000.0, Box::new(
        Lambertian::texture(Box::new(Checker::new(Color::new(0.0, 0.0, 0.0), Color::new(1.0, 1.0, 1.0), 10.0)))))));
    objs.push(Box::new(Sphere::new(V3::new(0.0, 1.0, 0.0), 1.0, Box::new(
        Lambertian::texture(Box::new(ImageTexture::load("./textures/stone.png")))))));
    objs.push(Box::new(XZRect::new(-1.0..1.0, -1.0..1.0, 2.5, Arc::new(
        DiffuseLight::new(Box::new(Color::new(1.0, 1.0, 0.99)), 4.0)))));
    objs.push(Box::new(XYRect::new(-1.0..1.0, 0.5..1.5, -1.5, Arc::new(
        DiffuseLight::new(Box::new(Color::new(1.0, 1.0, 0.99)), 4.0)))));
    objs
}

fn cornel_box_scene() -> Vec<Box<dyn Hittable>> {
    let red = Arc::new(Lambertian::color(Color::new(0.65, 0.05, 0.05)));
    let floor_white = Arc::new(Lambertian::color(Color::new(0.73, 0.73, 0.73)));
    let ceil_white = Arc::new(Lambertian::color(Color::new(0.73, 0.73, 0.73)));
    let back_white = Arc::new(Lambertian::color(Color::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::color(Color::new(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::new(Box::new(Color::new(1.0, 1.0, 1.0)), 15.0));

    let mut objs: Vec<Box<dyn Hittable>> = Vec::new();
    objs.push(FlipNormals::new(Box::new(YZRect::new(0.0..555.0, 0.0..555.0, 555.0, green))));
    objs.push(Box::new(YZRect::new(0.0..555.0, 0.0..555.0, 0.0, red)));
    objs.push(Box::new(XZRect::new(213.0..343.0, 227.0..332.0, 554.0, light)));
    objs.push(Box::new(XZRect::new(0.0..555.0, 0.0..555.0, 0.0, floor_white)));
    objs.push(FlipNormals::new(Box::new(XZRect::new(0.0..555.0, 0.0..555.0, 555.0, ceil_white))));
    objs.push(FlipNormals::new(Box::new(XYRect::new(0.0..555.0, 0.0..555.0, 555.0, back_white))));

    objs
}

fn cornel_box_scene_with_instances() -> Vec<Box<dyn Hittable>> {


    let mut objs: Vec<Box<dyn Hittable>> = cornel_box_scene();
    objs.push(Box::new(AABox::mono(
        130.0..295.0, 0.0..165.0, 65.0..230.0,
        Arc::new(Lambertian::new(V3::new(0.73, 0.73, 0.73))),
        )));

    objs.push(Box::new(AABox::mono(
        265.0..430.0, 0.0..330.0, 295.0..460.0,
        Arc::new(Lambertian::new(V3::new(0.73, 0.73, 0.73))),
        )));

    let light = Arc::new(DiffuseLight::new(Box::new(Color::new(1.0, 1.0, 1.0)), 25.0));
    objs.push(Box::new(XZRect::new(213.0..343.0, 227.0..332.0, 554.0, light)));
    objs.swap_remove(2);

    objs
}

// naive took 6m12s with 800x400xaa100
// BVH took 5m20s with 800x400xaa100
fn rnd_scene() -> Vec<Box<dyn Hittable>> {
    let mut objs: Vec<Box<dyn Hittable>> = Vec::new();

    objs.push(Box::new(Sphere::new(V3::new(0.0, -1000.0, 0.0), 1000.0, Box::new(
        Lambertian::texture(Box::new(Checker::new(
            Color::new(0.2, 0.3, 0.1),
            Color::new(0.9, 0.9, 0.9), 10.0,
        )))))));

    objs.push(Box::new(Sphere::new(V3::new(4.0, 1.0, 0.0), 1.0, Box::new(Metal::new(V3::new(0.7, 0.6, 0.5))))));
    objs.push(Box::new(Sphere::new(V3::new(0.0, 1.0, 0.0), 1.0, Box::new(Dielectric::new(1.5)))));
    objs.push(Box::new(Sphere::new(V3::new(-4.0, 1.0, 0.0), 1.0, Box::new(Lambertian::color(Color::new(0.8, 0.8, 0.9))))));

    for a in -10..=10 {
        for b in -10..=10 {
            let center = V3::new(0.9 * next_std_f64() + a as f64,
                                 0.2,
                                 0.9 * next_std_f64() + b as f64);

            if (center - V3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                objs.push(
                    // todo: #41620
                    match random::next_std_f32() {
                        0.0..=0.8 => Box::new(MovingSphere::new(
                            center,
                            center + V3::new(0.0, 0.5 * next_std_f64(), 0.0),
                            0.0, 1.0, 0.2,
                            Box::new(Lambertian::new(next_color() * next_color())),
                        )),
                        0.8..=0.95 => Box::new(Sphere::new(
                            center,
                            0.2,
                            Box::new(Metal::new(0.5 * (next_color() + 1.0))),
                        )),
                        _ => Box::new(Sphere::new(center, 0.2,
                                                  Box::new(Dielectric::new(1.5))))
                    });
            }
        }
    }
    objs
}

fn cornel_box_cam(nx: u32, ny: u32, t_off: f32, t_span: f32, ttl: i32) -> Camera {
    let aspect = (nx as f64) / (ny as f64);
    let from = V3::new(278.0, 278.0, -800.0);
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
        ttl
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
        ttl
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
        ttl
    )
}

struct Renderer {
    pub hittable: Box<dyn Hittable>
}

impl Renderer {
    fn color(&self, r: &Ray) -> V3 {
        match self.hittable.hit(&r, 0.0001, 99999.0) {
            Some(hit) => {
                let emitted = hit.material.emmit(&hit);
                return match hit
                    .material
                    .scatter(r, &hit)
                    .and_then(Ray::validate) {
                    Some(scattered) => { emitted.0 + scattered.attenuation * self.color(&scattered) }
                    None => emitted.0
                };
            }
            None => {
//                let unit_direction = r.direction.unit();
//                let t: f64 = 0.5 * (unit_direction.y + 1.0);
                return V3::new(0.0, 0.0, 0.01);
//                return (1.0 - t) * V3::ones() + t * V3::new(0.5, 0.7, 1.0);
            }
        };
    }
}
