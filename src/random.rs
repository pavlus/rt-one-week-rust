#[allow(dead_code)]
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::ops::Range;

use rand::{RngCore, SeedableRng};
use rand::distributions::{Distribution, Standard};
use rand::seq::SliceRandom;
use rand_distr::{UnitDisc, UnitSphere};
use rand_xoshiro::Xoshiro256Plus;

use crate::types::{V3, P3, Geometry, Color, ColorComponent, Direction, Probability};
use crate::consts::{TAU};
use nalgebra::Unit;

thread_local! {
    static RND: RefCell<Xoshiro256Plus> =
            RefCell::new(Xoshiro256Plus::seed_from_u64(0))
}

pub fn next_std_f64() -> f64 {

    RND.with(|rnd_cell|
        Standard.sample((*rnd_cell.borrow_mut()).borrow_mut()))
}

pub fn next_std<T>() -> T where Standard: Distribution<T>{
    RND.with(|rnd_cell|
        Standard.sample((*rnd_cell.borrow_mut()).borrow_mut()))
}

pub fn next_std_distance() -> Geometry {
    next_std()
}

pub fn next_std_color_comp() -> ColorComponent {
    next_std()
}

pub fn next_std_i32() -> i32 {
    RND.with(|rnd_cell|
        Standard.sample((*rnd_cell.borrow_mut()).borrow_mut()))
}

pub fn next_f64<D: Distribution<f64>>(d: D) -> f64 {
    RND.with(|rnd_cell|
        d.sample((*rnd_cell.borrow_mut()).borrow_mut()))
}

pub fn next<T, D: Distribution<T>>(d: D) -> T {
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

pub fn next_color() -> Color {
    Color::new(next_std(), next_std(), next_std())
}

pub fn random_axis() -> usize {
    RND.with(|rnd_cell|
        ((*rnd_cell.borrow_mut()).borrow_mut().next_u32() % 3) as usize)
}

pub fn random_item<T>(from: &[T]) -> Option<&T> {
    RND.with(|rnd_cell|
        from.choose((*rnd_cell.borrow_mut()).borrow_mut()))
}

pub fn next_std_f64_in_range(range: &Range<f64>) -> f64 {
    let value: f64 = RND.with(|rnd_cell|
        Standard.sample((*rnd_cell.borrow_mut()).borrow_mut()));
    value.mul_add(range.end - range.start, range.start)
}

pub fn next_std_in_range(range: &Range<Geometry>) -> Geometry {
    let value: Geometry = RND.with(|rnd_cell|
        Standard.sample((*rnd_cell.borrow_mut()).borrow_mut()));
    value.mul_add(range.end - range.start, range.start)
}

pub fn rand_in_unit_sphere() -> P3 {
    RND.with(|rnd_cell| UnitSphere.sample((*rnd_cell.borrow_mut()).borrow_mut())).into()
}

pub fn rand_in_unit_hemisphere(normal: &V3) -> P3 {
    let result = rand_in_unit_sphere();
    if result.coords.dot(normal) > 0.0 { result } else { -result }
}

pub fn rand_cosine_direction() -> Direction {
    let r1: Geometry = next_std();
    let r2 = next_std();
    let z = Geometry::sqrt(1.0 - r2);

    let phi = r1 * TAU as Geometry;
    let (sin, cos) = Geometry::sin_cos(phi);
    let sqrt_r2 = Geometry::sqrt(r2);

    let x = cos * sqrt_r2;
    let y = sin * sqrt_r2;

    let result = V3::new(x, y, z);
    Unit::new_unchecked(result)
}

pub fn rand_in_unit_disc() -> V3 {
    RND.with(|rnd_cell| {
        let [x, y] = UnitDisc.sample((*rnd_cell.borrow_mut()).borrow_mut());
        V3::new(x, y, 0.0)
    })
}

pub fn with_rnd<T, F>(op: F) -> T
    where F: FnOnce(&mut dyn RngCore) -> T {
    RND.with(|rnd_cell| op((*rnd_cell.borrow_mut()).borrow_mut()))
}
