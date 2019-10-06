use std::borrow::BorrowMut;
use std::cell::RefCell;

use rand::seq::SliceRandom;
use rand::distributions::{Distribution, Standard};
use rand::SeedableRng;
use rand_xoshiro::Xoshiro256Plus;
use crate::vec::V3;
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

pub fn random_axis() -> &'static dyn (Fn(V3) -> f64) {
    RND.with(|rnd_cell|
        [V3::x, V3::y, V3::z].choose((*rnd_cell.borrow_mut()).borrow_mut())).unwrap()
}

pub fn rand_in_unit_sphere() -> V3 {
    RND.with(|rnd_cell| {
        let arr = UnitSphere.sample((*rnd_cell.borrow_mut()).borrow_mut());
        V3 { x: arr[0], y: arr[1], z: arr[2] }
    }
    )
}