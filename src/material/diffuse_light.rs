use crate::ray::RayCtx;
use crate::scatter::Scatter;
use crate::types::ColorComponent;
use super::{Color, Texture};
use super::{Hit, Material};

#[derive(Clone, Debug)]
pub struct DiffuseLight<T> {
    intensity_scale: ColorComponent,
    texture: T,
}


impl<T: Texture> DiffuseLight<T> {
    pub fn new(texture: T, scale: ColorComponent) -> DiffuseLight<T> {
        DiffuseLight { intensity_scale: scale, texture }
    }
}

impl<T: Texture> Material for DiffuseLight<T> {
    fn emmit(&self, hit: &Hit) -> Color {
        self.intensity_scale * self.texture.value(&hit.uv, &hit.point)
    }

    fn scatter_with_pdf(&self, _ray_ctx: RayCtx, _hit: &Hit) -> Option<Scatter> {
        None
    }
}

impl Default for DiffuseLight<Color> {
    fn default() -> Self {
        DiffuseLight::new(Color::from_element(1.0), 0.5)
    }
}
