#[allow(dead_code)]

use std::borrow::BorrowMut;
use std::cell::RefCell;

use rand::distributions::{Distribution, Standard};
use rand::{SeedableRng, RngCore};
use rand_xoshiro::Xoshiro256Plus;
use crate::vec::{V3, Axis};
use rand_distr::{UnitDisc, UnitSphere};

thread_local! {
    static RND: RefCell<Xoshiro256Plus> =
            RefCell::new(Xoshiro256Plus::seed_from_u64(0))
}

pub fn next_std_f64() -> f64 {
    RND.with(|rnd_cell|
        Standard.sample((*rnd_cell.borrow_mut()).borrow_mut()))
}

pub fn next_f64<D: Distribution<f64>>(d: D) -> f64 {
    RND.with(|rnd_cell|
        d.sample((*rnd_cell.borrow_mut()).borrow_mut()))
}

pub fn next_std_f32() -> f32 {
    RND.with(|rnd_cell|
        Standard.sample((*rnd_cell.borrow_mut()).borrow_mut()))
}

pub fn next_std_u32() -> u32 {
    RND.with(|rnd_cell|
        Standard.sample((*rnd_cell.borrow_mut()).borrow_mut()))
}


pub fn next_f32<D: Distribution<f32>>(d: D) -> f32 {
    RND.with(|rnd_cell|
        d.sample((*rnd_cell.borrow_mut()).borrow_mut()))
}

pub fn next_color() -> V3 {
    V3::new(next_std_f64(), next_std_f64(), next_std_f64())
}

pub fn random_axis() -> &'static Axis {
    RND.with(|rnd_cell|
        Axis::random((*rnd_cell.borrow_mut()).borrow_mut()))
}

pub fn rand_in_unit_sphere() -> V3 {
    V3::from(RND.with(|rnd_cell| UnitSphere.sample((*rnd_cell.borrow_mut()).borrow_mut())))
}

pub fn rand_in_unit_hemisphere(normal: &V3) -> V3 {
    let result = rand_in_unit_sphere();
    if result.dot(*normal) > 0.0 { result } else { -result }
}

pub fn rand_cosine_direction() -> V3 {
    let r1 = next_std_f64();
    let r2 = next_std_f64();

    let phi = r1 * 2.0 * core::f64::consts::PI;
    let (sin, cos) = f64::sin_cos(phi);
    let sqrt_r2 = f64::sqrt(r2);

    V3::new(cos * sqrt_r2, sin * sqrt_r2, f64::sqrt(1.0 - r2))
}

pub fn rand_in_unit_disc() -> [f64; 2] {
    RND.with(|rnd_cell| UnitDisc.sample((*rnd_cell.borrow_mut()).borrow_mut()))
}

pub fn with_rnd<T, F>(op: F) -> T
    where F: FnOnce(&mut dyn RngCore) -> T {
    RND.with(|rnd_cell| op((*rnd_cell.borrow_mut()).borrow_mut()))
}
