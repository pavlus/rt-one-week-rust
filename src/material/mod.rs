use std::fmt::Debug;

pub use lambertian::*;
pub use metal::*;
pub use dielectric::*;
pub use diffuse_light::*;

use crate::hittable::Hit;
use crate::ray::Ray;
use crate::texture::{Color, Texture};
use crate::vec::V3;

pub mod lambertian;
pub mod metal;
pub mod dielectric;
pub mod diffuse_light;

pub trait Material: Debug + Sync + Send {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<Ray> { None }
    fn emmit(&self, hit: &Hit) -> Color { Color(V3::zeros()) }
}

