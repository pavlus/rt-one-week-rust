use std::fmt::Debug;

pub use aabox::*;
pub use aarect::*;
pub use constant_medium::*;
pub use instance::*;
pub use list::*;
pub use sphere::*;

use crate::aabb::AABB;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec::V3;

mod sphere;
mod aarect;
mod list;
mod aabox;
mod constant_medium;
mod instance;

#[derive(Copy, Clone)]
pub struct Hit<'a> {
    pub point: V3,
    pub normal: V3,
    pub u: f64,
    pub v: f64,
    pub material: &'a dyn Material,
    pub dist: f64,
}

impl<'a> Hit<'a> {
    pub fn new(dist: f64, p: V3, n: V3, material: &'a dyn Material, u: f64, v: f64) -> Hit<'a> {
        return Hit { dist, point: p, normal: n, material, u, v };
    }
}

pub trait Hittable: Debug + Sync {
    fn hit(&self, ray: &Ray, dist_min: f64, dist_max: f64) -> Option<Hit>;
    fn bounding_box(&self, t_min: f32, t_max: f32) -> Option<AABB> { None }
}
