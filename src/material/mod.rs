use std::fmt::Debug;
pub use dielectric::*;
pub use diffuse_light::*;
pub use lambertian::*;
pub use metal::*;
pub use isotropic::*;

use crate::hittable::Hit;
use crate::ray::RayCtx;
use crate::texture::Texture;
use crate::types::{V3, Color};
use crate::scatter::Scatter;

pub mod lambertian;
pub mod metal;
pub mod dielectric;
pub mod diffuse_light;
pub mod isotropic;

type PDF = f64;

#[allow(unused_variables)]
pub trait Material: Sync + Send + Debug {
    fn scatter(&self, ray: &RayCtx, hit: &Hit) -> Option<RayCtx> { None }
    fn emmit(&self, hit: &Hit) -> Color { Color::from_element(0.0) }

    fn scatter_with_pdf(&self, ray_ctx: &RayCtx, hit: &Hit) -> Option<Scatter> {
        self.scatter(ray_ctx, hit).map(|ray| Scatter::Specular(ray))
    }
}
