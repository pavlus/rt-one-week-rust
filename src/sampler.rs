use crate::scenes::Scene;
use crate::types::{Distance, Color, ColorComponent};
use crate::random;
use rayon::prelude::*;

pub type Postprocessor = fn(Color) -> Color;

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
            let scale = self.samples as ColorComponent;
            let aa = self.samples;
            let row : Vec<(u32,u32,u32)> = (0..self.width).into_par_iter().map(|i| {
                let col: Color = (0..aa).map(|_| {
                    // let col: V3 = rayon::iter::repeatn((), self.samples).map(|_| {
                    let offset = random::rand_in_unit_disc();
                    let u = (i as Distance + &offset.x) / (self.width as Distance);
                    let v = (j as Distance + &offset.y) / (self.height as Distance);
                    scene.color(u, v)
                }).sum();

                let col = (self.pixel_postprocessor)(col / scale);

                let ir: u32 = (255.99 * col.x) as u32;
                let ig: u32 = (255.99 * col.y) as u32;
                let ib: u32 = (255.99 * col.z) as u32;

                assert![ir < 256];
                assert![ig < 256];
                assert![ib < 256];

                (ir, ig, ib)
            }).collect();
            row.iter().for_each(|(ir, ig, ib)| { print!("{} {} {} ", ir, ig, ib); });
            println!();
        }
    }
}
