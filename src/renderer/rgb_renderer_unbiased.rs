use super::{Hittable, RayCtx, Renderer, V3};
use std::borrow::Borrow;
use crate::ray::Ray;

pub struct RgbRendererUnbiased {
    pub hittable: Box<dyn Hittable>,
    pub miss_shader: fn(&Ray) -> V3,
}

impl Renderer for RgbRendererUnbiased {
    fn color(&self, ray_ctx: &RayCtx) -> V3 {
        match self.hittable.hit(&ray_ctx, 0.0001, 99999.0) {
            Some(hit) => {
                let emitted = hit.material.emmit(&hit);
                return match hit
                    .material
                    .scatter(ray_ctx, &hit)
                    .and_then(RayCtx::validate) {
                    Some(scattered) => { emitted.0 + scattered.attenuation * self.color(&scattered) }
                    None => emitted.0
                };
            }
            None => {
                return self.miss_shader.borrow()(&ray_ctx.ray);
            }
        };
    }
}
