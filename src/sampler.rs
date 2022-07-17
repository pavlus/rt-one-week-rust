use std::sync::atomic::{AtomicUsize, Ordering};
use crossbeam::atomic::AtomicConsume;
use rand::prelude::Distribution;
use rand::SeedableRng;
use rand_distr::UnitDisc;
use crate::types::{Geometry, Color, ColorComponent, V2};
use crate::Scene;
use rayon::prelude::*;
use crate::hittable::total_hits;
use crate::random2::DefaultRng;

pub type Postprocessor = fn(Color) -> Color;

pub struct SceneSampler {
    pub width: u32,
    pub height: u32,
    pub samples: usize,
    pub max_ray_bounces: i32,
    pub pixel_postprocessor: Postprocessor,
}

impl SceneSampler {
    pub fn do_render(self, scene: impl Scene) {
        println!("P3");
        println!("{} {}", self.width, self.height);
        println!("255");
        // PPM pixels go top to bottom left-to right,
        // View uv goes bottom to top, so Y is reversed
        let scene = &scene;
        #[cfg(feature = "metrics")]
            let rng_bytes = AtomicUsize::new(0);
        for j in (0..self.height).rev() {
            let scale = self.samples as ColorComponent;
            let aa = self.samples;
            let row: Vec<(u32, u32, u32)> = (0..self.width)
                .into_par_iter()
                .map(|i| {
                    let mut rng = DefaultRng::seed_from_u64(((i * 13 + j * 65537) % 44497) as u64);
                    let col: Color = (0..aa).map(|_| {
                        // let col: V3 = rayon::iter::repeatn((), self.samples).map(|_| {
                        // todo: better noise
                        let offset = V2::from(UnitDisc.sample(&mut rng));
                        let u = (i as Geometry + &offset.x) / (self.width as Geometry);
                        let v = (j as Geometry + &offset.y) / (self.height as Geometry);
                        scene.color(u, v, &mut rng)
                    }).sum();

                    let col = (self.pixel_postprocessor)(col / scale);

                    let ir: u32 = (255.99 * col.x) as u32;
                    let ig: u32 = (255.99 * col.y) as u32;
                    let ib: u32 = (255.99 * col.z) as u32;

                    assert![ir < 256];
                    assert![ig < 256];
                    assert![ib < 256];
                    #[cfg(feature = "metrics")]
                    rng_bytes.fetch_add(rng.1.load_consume(), Ordering::Relaxed);
                    (ir, ig, ib)
                }).collect();
            row.iter().for_each(|(ir, ig, ib)| { print!("{} {} {} ", ir, ig, ib); });
            println!();
        }
        #[cfg(feature = "metrics")]{
            eprintln!("Generated {} random bytes", rng_bytes.load_consume());
            eprintln!("Generated {} primary rays", scene.generated_rays());
            eprintln!("There were {} ray-object intersections", total_hits());
        }
    }
}
