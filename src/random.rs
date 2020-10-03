#[allow(dead_code)]

use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::ops::Range;

use rand::{RngCore, SeedableRng};
use rand::distributions::{Distribution, Standard};
use rand::seq::SliceRandom;
use rand_distr::{UnitDisc, UnitSphere};
use rand_xoshiro::Xoshiro256Plus;

use crate::vec::{Axis, V3};

thread_local! {
    static RND: RefCell<Xoshiro256Plus> =
            RefCell::new(Xoshiro256Plus::seed_from_u64(0))
}

pub fn next_std_f64() -> f64 {
    RND.with(|rnd_cell|
        Standard.sample((*rnd_cell.borrow_mut()).borrow_mut()))
}

pub fn next_std_i32() -> i32 {
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

pub fn flip_coin() -> bool {
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

pub fn random_item<T>(from: &[T]) -> Option<&T> {
    RND.with(|rnd_cell|
        from.choose((*rnd_cell.borrow_mut()).borrow_mut()))
}

pub fn next_std_f64_in_range(range: &Range<f64>) -> f64 {
    let value: f64 =  RND.with(|rnd_cell|
        Standard.sample((*rnd_cell.borrow_mut()).borrow_mut()));
    value.mul_add(range.end - range.start, range.start)
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
    let z = f64::sqrt(1.0 - r2);

    let phi = r1 * 2.0 * core::f64::consts::PI;
    let (sin, cos) = f64::sin_cos(phi);
    let sqrt_r2 = f64::sqrt(r2);

    let x = cos * sqrt_r2;
    let y = sin * sqrt_r2;

    let result = V3::new(x, y, z);
    result
}

pub fn rand_in_unit_disc() -> [f64; 2] {
    RND.with(|rnd_cell| UnitDisc.sample((*rnd_cell.borrow_mut()).borrow_mut()))
}

pub fn with_rnd<T, F>(op: F) -> T
    where F: FnOnce(&mut dyn RngCore) -> T {
    RND.with(|rnd_cell| op((*rnd_cell.borrow_mut()).borrow_mut()))
}
