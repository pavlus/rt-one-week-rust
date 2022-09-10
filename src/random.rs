use std::borrow::BorrowMut;
use std::cell::RefCell;

use rand::SeedableRng;
use rand::distributions::{Distribution, Standard};
use rand_distr::UnitSphere;
use rand_xoshiro::Xoshiro256Plus;

use crate::types::P3;

thread_local! {
    static RND: RefCell<Xoshiro256Plus> =
            RefCell::new(Xoshiro256Plus::seed_from_u64(0))
}

#[deprecated]
pub fn next_std_f64() -> f64 {
    RND.with(|rnd_cell|
        Standard.sample((*rnd_cell.borrow_mut()).borrow_mut()))
}

#[deprecated]
pub fn next_std<T>() -> T where Standard: Distribution<T>{
    RND.with(|rnd_cell|
        Standard.sample((*rnd_cell.borrow_mut()).borrow_mut()))
}

#[deprecated]
pub fn next<T, D: Distribution<T>>(d: D) -> T {
    RND.with(|rnd_cell|
        d.sample((*rnd_cell.borrow_mut()).borrow_mut()))
}


#[deprecated]
pub fn rand_in_unit_sphere() -> P3 {
    RND.with(|rnd_cell| UnitSphere.sample((*rnd_cell.borrow_mut()).borrow_mut())).into()
}

