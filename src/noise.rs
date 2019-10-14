use crate::vec::V3;
use rand::{Rng, RngCore};
use rand::seq::SliceRandom;
use std::fmt::{Display, Formatter, Error, Debug};
use crate::random;
use rand::distributions::{Standard, Distribution};

pub struct Perlin {
    ranvec: [V3; 256],
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

    pub fn noise(&self, point: V3) -> f64 {
        let u = point.x - point.x.floor();
        let v = point.y - point.y.floor();
        let w = point.z - point.z.floor();

        let i = point.x.floor() as usize & 255;
        let j = point.y.floor() as usize & 255;
        let k = point.z.floor() as usize & 255;

        let mut c: [[[V3; 2]; 2]; 2] = [[[V3::zeros(); 2]; 2]; 2];
        for di in 0..=1 {
            for dj in 0..=1 {
                for dk in 0..=1 {
                    c[di][dj][dk] = self.ranvec[
                        (self.permx[(i + di) & 255]
                            ^ self.permx[(j + dj) & 255]
                            ^ self.permx[(k + dk) & 255]
                        ) as usize
                        ];
                }
            }
        }
        let tri = trilerp(&c, u, v, w);
        let val = self.ranvec[
            (self.permx[i & 255] ^ self.permy[j & 255] ^ self.permz[k & 255]) as usize & 255];
        tri
    }

    fn generate<R: RngCore + ?Sized>(rnd: &mut R) -> [V3; 256] {
        let mut result: [V3; 256] = [V3::zeros(); 256];
        for i in 0..256 {
            result[i] = V3::new(
                2.0 * rnd.sample::<f64, Standard>(Standard) - 1.0,
                2.0 * rnd.sample::<f64, Standard>(Standard) - 1.0,
                2.0 * rnd.sample::<f64, Standard>(Standard) - 1.0,
            ).unit();
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
}

/*impl Debug for Perlin {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let rf_str = self.ranvec.iter().fold(String::from("["), |acc, val| acc + format!("{}, ", val).as_str()) + "]";
        let px_str = self.permx.iter().fold(String::from("["), |acc, val| acc + format!("{}, ", val).as_str()) + "]";
        let py_str = self.permy.iter().fold(String::from("["), |acc, val| acc + format!("{}, ", val).as_str()) + "]";
        let pz_str = self.permz.iter().fold(String::from("["), |acc, val| acc + format!("{}, ", val).as_str()) + "]";
        write!(f, "Perlin {{\n  ranvec: {},\n  permx: {},\n  permy: {},\n  permz: {}\n}}",
               rf_str, px_str, py_str, pz_str);
        Ok(())
    }
}*/

fn trilerp(c: &[[[V3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
    let uu = u * u * (3.0 - 2.0 * u);
    let vv = v * v * (3.0 - 2.0 * v);
    let ww = w * w * (3.0 - 2.0 * w);

    let mut acc = 0.0;
    for i in 0..=1 {
        for j in 0..=1 {
            for k in 0..=1 {
                let weight = V3::new(u - i as f64, v - j as f64, w - k as f64);
//                dbg![weight];
                acc += (i as f64 * uu + (1.0 - i as f64) * (1.0 - uu))
                    * (j as f64 * vv + (1.0 - j as f64) * (1.0 - vv))
                    * (k as f64 * ww + (1.0 - k as f64) * (1.0 - ww))
                    * c[i][j][k].dot(weight);
            }
        }
    }
//    dbg![c, acc, u, v, w, uu, vv, ww];

    acc
}
