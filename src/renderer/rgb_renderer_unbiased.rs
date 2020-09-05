use super::{Hittable, Ray, Renderer, V3};
use std::borrow::Borrow;

pub struct RgbRendererUnbiased {
    pub hittable: Box<dyn Hittable>,
    pub miss_shader: fn(&Ray) -> V3,
}

impl Renderer for RgbRendererUnbiased {
    fn color(&self, r: &Ray) -> V3 {
        match self.hittable.hit(&r, 0.0001, 99999.0) {
            Some(hit) => {
                let emitted = hit.material.emmit(&hit);
                return match hit
                    .material
                    .scatter(r, &hit)
                    .and_then(Ray::validate) {
                    Some(scattered) => { emitted.0 + scattered.attenuation * self.color(&scattered) }
                    None => emitted.0
                };
            }
            None => {
                return self.miss_shader.borrow()(r);
            }
        };
    }
}
