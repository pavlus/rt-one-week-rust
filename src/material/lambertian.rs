use crate::pdf::CosinePDF;
use crate::scatter::Scatter;

use super::{Color, Texture};
use super::{Hit, Material, RayCtx};

#[derive(Debug, Clone)]
pub struct Lambertian<T> {
    pub(crate) texture: T,
}

impl <T: Texture> Lambertian<T> {
    pub fn new(albedo: Color) -> Lambertian<Color> { Lambertian { texture: albedo } }
    pub fn texture(texture: T) -> Lambertian<T> { Lambertian { texture } }
}

impl <T: Texture> Material for Lambertian<T> {

    fn scatter_with_pdf(&self, _: RayCtx, hit: &Hit) -> Option<Scatter> {
        let albedo = self.texture.value(&hit.uv, &hit.point);
        Some(Scatter::Diffuse(Box::new(CosinePDF::from_w(hit.normal)), albedo))
    }
}

impl Default for Lambertian<Color> {
    fn default() -> Lambertian<Color> {
        Lambertian::<Color>::new(Color::from_element(0.5))
    }
}

impl<T: PartialEq> PartialEq<Self> for Lambertian<T> {
    fn eq(&self, other: &Self) -> bool {
        self.texture == other.texture
    }
}
