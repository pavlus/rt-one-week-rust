
use crate::ray::RayCtx;
use crate::texture::Color;
use crate::pdf::PDF;

pub enum Scatter {
    Specular(RayCtx),
    Diffuse(Box<dyn PDF>, Color)
}
