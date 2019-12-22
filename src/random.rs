use std::borrow::BorrowMut;
use std::cell::RefCell;

use rand::seq::SliceRandom;
use rand::distributions::{Distribution, Standard};
use rand::{SeedableRng, Rng, RngCore};
use rand_xoshiro::Xoshiro256Plus;
use crate::vec::{V3, Axis};
use rand_distr::UnitSphere;

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

pub fn with_rnd<T, F>(op: F) -> T
    where F: FnOnce(&mut RngCore) -> T {
    RND.with(|rnd_cell| op((*rnd_cell.borrow_mut()).borrow_mut()))
}