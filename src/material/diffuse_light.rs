use super::{Color, Texture};
use super::{Hit, Material};

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
        self.intensity_scale * self.texture.value(hit.u, hit.v, &hit.point)
    }
}

impl Default for DiffuseLight{
    fn default() -> Self {
        DiffuseLight::new(Box::new(Color::from_element(1.0)), 0.5)
    }
}
