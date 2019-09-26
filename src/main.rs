mod vec;
mod ray;
mod hittable;
mod camera;
mod material;

use vec::V3;
use ray::Ray;
use camera::Camera;
use crate::hittable::{Sphere, Hittable, Stage};
use core::borrow::Borrow;
use rand::distributions::{Normal, Distribution, Standard};
use rand::prelude::thread_rng;
use rand::Rng;
use crate::material::{Lambertian, Metal, Dielectric};
use std::f64::consts::PI;

fn main() {
    let nx = 1024;
    let ny = 768;
    let aa = 400;

    println!("P3");
    println!("{} {}", nx, ny);
    println!("255");

    let aspect =(nx as f64) / (ny as f64);
    let cam = Camera::new_look(
        /*  from*/ V3::new(-2.0, 2.0, 1.0),
        /*    at*/ V3::new(0.0, 0.0, -1.0),
        /*    up*/ V3::new(0.0, 1.0, 0.0),
        /*  vfov*/ 90.0,
        /*aspect*/ aspect,
    );
    let renderer = Renderer {
        hittable: &Stage::new(
            vec![
                Box::new(Sphere::new(
                    V3::new(-1.0, 0.0, -1.2), 0.5,
                    Box::new(Metal::new_fuzzed(V3::new(0.8, 0.8, 0.8), 1.0)))),
                Box::new(Sphere::new(
                    V3::new(-0.3, 0.2, -0.8), 0.4,
//                    Box::new(Dielectric::new_colored(V3::new(1.0, 1.0, 1.0), 1.5)))),
                    Box::new(Dielectric::new_colored(V3::new(1.0, 0.6, 0.6), 1.5)))),
                Box::new(Sphere::new(
                    V3::new(1.0, 0.0, -1.2), 0.5,
                    Box::new(Dielectric::new(1.5)))),
                Box::new(Sphere::new(
                    V3::new(0.0, -100.5, -1.0), 100.0,
                    Box::new(Lambertian::new(V3::new(0.8, 0.8, 0.3))))),
            ]
        )
    };
    let mut rand = rand::thread_rng();
//    let dist = Normal::new(0.0, 1.0);
    let dist = Standard;

    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col: V3 = V3::zeros();
            for _ in 0..aa {
//                let du: f64 = dist.sample(&mut rand);
//                let dv: f64 = dist.sample(&mut rand);
                let du: f64 = rand.gen::<f64>() - 0.5;
                let dv: f64 = rand.gen::<f64>() - 0.5;
                let u = (i as f64 + du) / (nx as f64);
                let v = (j as f64 + dv) / (ny as f64);
                let r = cam.get_ray(u, v);
                col = col + renderer.color(&r);
            }

            col = col / aa as f64;
            // non gamma-corrected
            let ir: u32 = (255.99 * col.x) as u32;
            let ig: u32 = (255.99 * col.y) as u32;
            let ib: u32 = (255.99 * col.z) as u32;
            assert![ir < 256];
            assert![ig < 256];
            assert![ib < 256];
            /*let ir: u32 = (255.99 * col.x.sqrt()) as u32;
            let ig: u32 = (255.99 * col.y.sqrt()) as u32;
            let ib: u32 = (255.99 * col.z.sqrt()) as u32;
            */
            print!("{} {} {} ", ir, ig, ib);
        }
        println!();
    }
}


struct Renderer<'a> {
    pub hittable: &'a Stage
}

impl Renderer<'_> {
    fn color(&self, r: &Ray) -> V3 {
        match self.hittable.hit(&r, 0.0001, 99999.0) {
            Some(hit) => {
                return match hit
                    .material()
                    .scatter(r, &hit)
                    .and_then(Ray::validate) {
                    Some(scattered) => { scattered.attenuation() * self.color(&scattered) }
                    None => r.attenuation()
                };
            }
            None => {
                let unit_direction = r.direction().unit();
                let t: f64 = 0.5 * (unit_direction.y + 1.0);
//                return V3::new(1.0, 1.0, 1.0);
                return (1.0 - t) * V3::ones() + t * V3::new(0.5, 0.7, 1.0);
            }
        };
    }
}
