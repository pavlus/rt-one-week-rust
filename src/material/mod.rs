use std::fmt::Debug;

pub use lambertian::*;
pub use metal::*;
pub use dielectric::*;

use crate::hittable::Hit;
use crate::random;
use crate::ray::Ray;
use crate::texture::{Color, Texture};
use crate::vec::V3;

pub mod lambertian;
pub mod metal;
pub mod dielectric;

pub trait Material: Debug + Sync + Send {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<Ray> { None }
    fn emmit(&self, hit: &Hit) -> Color { Color(V3::zeros()) }
}


#[derive(Debug)]
pub struct DiffuseLight {
    texture: Box<dyn Texture>,
    intensity_scale: f64,
}

impl DiffuseLight {
    pub fn new(texture: Box<dyn Texture>, scale: f64) -> DiffuseLight {
        DiffuseLight { texture, intensity_scale: scale }
    }
}

impl Material for DiffuseLight {
    fn emmit(&self, hit: &Hit) -> Color {
        Color(self.intensity_scale * self.texture.value(hit.u, hit.v, hit.point).0)
    }
}
