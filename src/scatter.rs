use crate::ray::RayCtx;
use crate::pdf::PDF;
use crate::types::Color;

pub enum Scatter {
    Specular(RayCtx, Color),
    Diffuse(Box<dyn PDF>, Color),
}
