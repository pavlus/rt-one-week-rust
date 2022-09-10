use rand::{Rng, RngCore};
use rand::distributions::Standard;
use rand::seq::SliceRandom;

use crate::types::{ColorComponent, V3};
use nalgebra::Vector3;

#[derive(Copy, Clone)]
pub struct Perlin {
    ranvec: [Vector3<ColorComponent>; 256],
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
    #[inline]
    pub fn noise(&self, point: &V3) -> ColorComponent {
        // offsets inside cell
        let u = (point.x - point.x.floor()) as ColorComponent;
        let v = (point.y - point.y.floor()) as ColorComponent;
        let w = (point.z - point.z.floor()) as ColorComponent;

        // cell coordinates
        let i = point.x.floor() as usize & 255;
        let j = point.y.floor() as usize & 255;
        let k = point.z.floor() as usize & 255;
        let next_i = (i + 1) & 255;
        let next_j = (j + 1) & 255;
        let next_k = (k + 1) & 255;

        // cell corner vectors
        let mut c: [Vector3<ColorComponent>; 8] = [Vector3::from_element(0.0); 8];

        let perm_i = self.permx[i];
        let perm_j = self.permy[j];
        let perm_k = self.permz[k];

        let perm_next_j = self.permy[next_j];
        let perm_next_k = self.permz[next_k];
        let perm_next_i = self.permx[next_i];

        let _i: u64 = u64::from_be_bytes([perm_i, perm_i, perm_i, perm_i, perm_next_i, perm_next_i, perm_next_i, perm_next_i]);
        let _j: u64 = u64::from_be_bytes([perm_j, perm_j, perm_next_j, perm_next_j, perm_j, perm_j, perm_next_j, perm_next_j]);
        let _k: u64 = u64::from_be_bytes([perm_k, perm_next_k, perm_k, perm_next_k, perm_k, perm_next_k, perm_k, perm_next_k]);
        let indices: [u8; 8] = (_i ^ _j ^ _k).to_be_bytes();
        c[0b000] = self.ranvec[indices[0b000] as usize];
        c[0b001] = self.ranvec[indices[0b001] as usize];
        c[0b010] = self.ranvec[indices[0b010] as usize];
        c[0b011] = self.ranvec[indices[0b011] as usize];
        c[0b100] = self.ranvec[indices[0b100] as usize];
        c[0b101] = self.ranvec[indices[0b101] as usize];
        c[0b110] = self.ranvec[indices[0b110] as usize];
        c[0b111] = self.ranvec[indices[0b111] as usize];

        trilerp(&c, u, v, w)
    }

    fn generate<R: RngCore + ?Sized>(rnd: &mut R) -> [Vector3<ColorComponent>; 256] {
        let mut result: [Vector3<ColorComponent>; 256] = [Vector3::from_element(0.0); 256];
        for i in 0..256 {
            result[i] = Vector3::new(
                2.0 * rnd.sample::<ColorComponent, Standard>(Standard) - 1.0,
                2.0 * rnd.sample::<ColorComponent, Standard>(Standard) - 1.0,
                2.0 * rnd.sample::<ColorComponent, Standard>(Standard) - 1.0,
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

    pub fn turb(&self, p: &V3) -> ColorComponent {
        let mut acc = 0.0;
        let mut temp = *p;
        let mut weight = 1.0;
        for _ in 0..7 {
            acc += weight * self.noise(&temp);
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
fn trilerp(c: &[Vector3<ColorComponent>; 8], u: ColorComponent, v: ColorComponent, w: ColorComponent) -> ColorComponent {
    // Cubic Hermite spline h01:
    let uu = u * u * (3.0 - 2.0 * u);
    let vv = v * v * (3.0 - 2.0 * v);
    let ww = w * w * (3.0 - 2.0 * w);

    let uu_compl = 1.0 - uu;
    let vv_compl = 1.0 - vv;
    let ww_compl = 1.0 - ww;

    let mut acc = 0.0;
    acc += c[0b000].dot(&Vector3::new(u, v, w)) * uu_compl * vv_compl * ww_compl;
    acc += c[0b001].dot(&Vector3::new(u, v, -ww_compl)) * uu_compl * vv_compl * ww;
    acc += c[0b010].dot(&Vector3::new(u, -vv_compl, w)) * uu_compl * vv * ww_compl;
    acc += c[0b011].dot(&Vector3::new(u, -vv_compl, -ww_compl)) * uu_compl * vv * ww;
    acc += c[0b100].dot(&Vector3::new(-uu_compl, v, w)) * uu * vv_compl * ww_compl;
    acc += c[0b101].dot(&Vector3::new(-uu_compl, v, -ww_compl)) * uu * vv_compl * ww;
    acc += c[0b110].dot(&Vector3::new(-uu_compl, -vv_compl, w)) * uu * vv * ww_compl;
    acc += c[0b111].dot(&Vector3::new(-uu_compl, -vv_compl, -ww_compl)) * uu * vv * ww;

    acc
}
