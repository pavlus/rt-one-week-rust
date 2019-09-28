use std::f64::consts::PI;

use camera::Camera;
use ray::Ray;
use vec::V3;

use crate::hittable::{Hittable, Sphere, MovingSphere, Stage};
use crate::material::{Dielectric, Lambertian, Metal};

mod vec;
mod ray;
mod hittable;
mod camera;
mod material;
mod random;

fn main() {
    let nx = 800;
    let ny = 400;
    let aa = 600;

    println!("P3");
    println!("{} {}", nx, ny);
    println!("255");

    let cam = get_cam(nx, ny, 0.0);
    let renderer = Renderer {
        hittable: &Stage::new(
            vec![
                Box::new(Sphere::new(
                    V3::new(-1.0, 0.0, -1.2), 0.5,
                    Box::new(Metal::new_fuzzed(V3::new(0.8, 0.8, 0.8), 1.0)))),
                Box::new(MovingSphere::new(
                    V3::new(-0.3, 0.2, -0.8),
                    V3::new(-0.3, 1.2, -0.8),
                    0.0, 1.0,
                    0.4,
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
    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col: V3 = V3::zeros();
            for _ in 0..aa {
//                let du: f64 = dist.sample(&mut rand);
//                let dv: f64 = dist.sample(&mut rand);
                let du: f64 = random::next_std_f64() - 0.5;
                let dv: f64 = random::next_std_f64() - 0.5;
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

fn get_cam(nx: u32, ny: u32, t_off: f32) -> Camera {
    let aspect = (nx as f64) / (ny as f64);
    let from = V3::new(-3.0, 3.0, 2.0);
    let at = V3::new(0.0, 0.0, -1.0);
    let dist_to_focus = (from - at).length();
    let aperture = 0.01;
    Camera::new_look(
        from, at,
        /*    up*/ V3::new(0.0, 1.0, 0.0),
        /*  vfov*/ 90.0,
        aspect,
        dist_to_focus,
        aperture,
        t_off, t_off + 0.2,
    )
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
