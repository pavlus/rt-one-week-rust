use super::{Color, Texture};
use super::{Hit, Material};
use crate::vec::V3;

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

impl Default for DiffuseLight{
    fn default() -> Self {
        DiffuseLight::new(Box::new(Color(V3::ones())), 0.5)
    }
}
