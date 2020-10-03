use std::fmt::Debug;

pub use dielectric::*;
pub use diffuse_light::*;
pub use lambertian::*;
pub use metal::*;
pub use isotropic::*;

use crate::hittable::Hit;
use crate::ray::Ray;
use crate::texture::{Color, Texture};
use crate::vec::V3;
use crate::scatter::Scatter;

pub mod lambertian;
pub mod metal;
pub mod dielectric;
pub mod diffuse_light;
pub mod isotropic;

type PDF = f64;

#[allow(unused_variables)]
pub trait Material: Debug + Sync + Send {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<Ray> { None }
    fn emmit(&self, hit: &Hit) -> Color { Color(V3::zeros()) }

    fn scatter_with_pdf(&self, ray: &Ray, hit: &Hit) -> Option<Scatter> {
        self.scatter(ray, hit).map(|ray| Scatter::Specular(ray))
    }
    fn scattering_pdf(&self, hit: &Hit, direction: &V3) -> PDF { 0.0 }
}
