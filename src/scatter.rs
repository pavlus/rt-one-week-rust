
use crate::ray::Ray;
use crate::texture::Color;
use crate::pdf::PDF;

pub enum Scatter {
    Specular(Ray),
    Diffuse(Box<dyn PDF>, Color)
}
