use rand::{Rng, RngCore};
use rand::distributions::Standard;
use rand::seq::SliceRandom;

use crate::types::V3;
use nalgebra::Vector3;

#[derive(Copy, Clone)]
pub struct Perlin {
    ranvec: [Vector3<f64>; 256],
    permx: [u8; 256],
    permy: [u8; 256],
    permz: [u8; 256],
}

impl Perlin {
    pub fn new<R: Rng + ?Sized>(rnd: &mut R) -> Perlin {
        Perlin {
            ranvec: Perlin::generate(rnd),
            permx: Perlin::generate_permutations(rnd),
            permy: Perlin::generate_permutations(rnd),
            permz: Perlin::generate_permutations(rnd),
        }
    }

    /// returns values in range [-1.0, 1.0)
    pub fn noise(&self, point: &V3) -> f64 {
        // offsets inside cell
        let u = (point.x - point.x.floor()) as f64;
        let v = (point.y - point.y.floor()) as f64;
        let w = (point.z - point.z.floor()) as f64;

        // cell coordinates
        let i = point.x.floor() as usize & 255;
        let j = point.y.floor() as usize & 255;
        let k = point.z.floor() as usize & 255;

        // cell corner vectors
        let mut c: [[[Vector3<f64>; 2]; 2]; 2] = [[[Vector3::from_element(0.0); 2]; 2]; 2];
        for di in 0..=1 {
            for dj in 0..=1 {
                for dk in 0..=1 {
                    c[di][dj][dk] = self.ranvec[
                        (self.permx[(i + di) & 255]
                            ^ self.permy[(j + dj) & 255]
                            ^ self.permz[(k + dk) & 255]
                        ) as usize
                        ].clone_owned();
                }
            }
        }
        trilerp(&c, u, v, w)
    }

    fn generate<R: RngCore + ?Sized>(rnd: &mut R) -> [Vector3<f64>; 256] {
        let mut result: [Vector3<f64>; 256] = [Vector3::from_element(0.0); 256];
        for i in 0..256 {
            result[i] = Vector3::new(
                2.0 * rnd.sample::<f64, Standard>(Standard) - 1.0,
                2.0 * rnd.sample::<f64, Standard>(Standard) - 1.0,
                2.0 * rnd.sample::<f64, Standard>(Standard) - 1.0,
            ).normalize();
        }
        result
    }

    fn generate_permutations<R: Rng + ?Sized>(rnd: &mut R) -> [u8; 256] {
        let mut result: [u8; 256] = [0; 256];
        for i in 0..256 {
            result[i] = i as u8;
        }
        (&mut result).shuffle(rnd);
        result
    }

    pub fn turb(&self, p: &V3) -> f64 {
        let mut acc = 0.0;
        let mut temp = p.to_owned();
        let mut weight = 1.0;
        for _ in 0..7 {
            acc += weight * self.noise(&temp.into());
            weight *= 0.5;
            temp = 2.0 * &temp;
        }
        let result = acc.abs();
        result
    }
}

/// trilinear cubic inerpolated values of Perlin noise
/// c -- cell corner vectors
/// u, v, w -- coordinates inside cell
fn trilerp(c: &[[[Vector3<f64>; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
    // Cubic Hermite spline h01:
    let uu = u * u * (3.0 - 2.0 * u);
    let vv = v * v * (3.0 - 2.0 * v);
    let ww = w * w * (3.0 - 2.0 * w);

    let mut acc = 0.0;
    for i in 0..=1 {
        for j in 0..=1 {
            for k in 0..=1 {
                let weight = Vector3::new(u - i as f64, v - j as f64, w - k as f64);
                acc += (i as f64 * uu + (1.0 - i as f64) * (1.0 - uu))
                    * (j as f64 * vv + (1.0 - j as f64) * (1.0 - vv))
                    * (k as f64 * ww + (1.0 - k as f64) * (1.0 - ww))
                    * c[i][j][k].dot(&weight);
            }
        }
    }
    acc
}
