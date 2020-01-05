use rayon::prelude::*;

use crate::renderer::{Renderer, RendererImpl};
use crate::scenes::Scene;
use crate::vec::V3;
use crate::random;
use itertools::Itertools;

pub type Postprocessor = fn(V3) -> V3;

#[derive(Debug, Clone, Copy)]
pub struct Sampler {
    pub width: u32,
    pub height: u32,
    pub samples: usize,
    pub max_ray_bounces: i32,
    pub pixel_postprocessor: Postprocessor,
}

impl Sampler {
    pub fn do_render(self, scene: Scene) -> () {
        println!("P3");
        println!("{} {}", self.width, self.height);
        println!("255");
        for j in (0..self.height).rev() {
            for i in 0..self.width {
                let scale = self.samples as f64;
//            let col: V3 = (0..aa).map(|_| {
                let col: V3 = rayon::iter::repeatn((), self.samples).map(|_| {
                    let [du, dv] = random::rand_in_unit_disc();
                    let u = (i as f64 + du) / (self.width as f64);
                    let v = (j as f64 + dv) / (self.height as f64);
                    scene.color(u, v)
                }).sum();

                let col = (self.pixel_postprocessor)(col / scale);

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
}
