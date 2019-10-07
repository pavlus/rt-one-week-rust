use camera::Camera;
use ray::Ray;
use vec::V3;

use crate::hittable::{Hittable, MovingSphere, Sphere, Stage};
use crate::material::{Dielectric, Lambertian, Metal};
use crate::random::{next_color, next_std_f64};
use crate::bvh::BVH;

mod vec;
mod ray;
mod hittable;
mod camera;
mod material;
mod random;
mod aabb;
mod bvh;

fn main() {
    let nx = 800;
    let ny = 400;
    let aa = 100;

    println!("P3");
    println!("{} {}", nx, ny);
    println!("255");

    let cam = get_cam(nx, ny, 0.0, 0.2);
    let renderer = Renderer {
//        hittable: &Stage::new(rnd_scene())
        hittable: BVH::new(rnd_scene())
    };
//    dbg!(&renderer.hittable);
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

// naive took 6m12s with 800x400xaa100
fn rnd_scene() -> Vec<Box<dyn Hittable>> {
    let mut objs: Vec<Box<dyn Hittable>> = Vec::new();

    objs.push(Box::new(Sphere::new(V3::new(0.0, -1000.0, 0.0), 1000.0, Box::new(Lambertian::new(V3::new(0.8, 0.8, 0.9))))));

    objs.push(Box::new(Sphere::new(V3::new(4.0, 1.0, 0.0), 1.0, Box::new(Metal::new(V3::new(0.7, 0.6, 0.5))))));
    objs.push(Box::new(Sphere::new(V3::new(0.0, 1.0, 0.0), 1.0, Box::new(Dielectric::new(1.5)))));
    objs.push(Box::new(Sphere::new(V3::new(-4.0, 1.0, 0.0), 1.0, Box::new(Lambertian::new(V3::new(0.8, 0.8, 0.9))))));

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

fn get_cam(nx: u32, ny: u32, t_off: f32, t_span: f32) -> Camera {
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
    )
}

fn _get_cam(nx: u32, ny: u32, t_off: f32, t_span: f32) -> Camera {
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
        t_off, t_span,
    )
}

struct Renderer {
    pub hittable: Box<dyn Hittable>
}

impl Renderer {
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
