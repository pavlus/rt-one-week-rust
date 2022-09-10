use super::{Hittable, RayCtx, Renderer};
use std::borrow::Borrow;
use crate::ray::Ray;
use crate::types::Color;

pub struct RgbRendererUnbiased {
    pub hittable: Box<dyn Hittable>,
    pub miss_shader: fn(&Ray) -> Color,
}

impl Renderer for RgbRendererUnbiased {
    fn color(&self, ray_ctx: &RayCtx) -> Color {
        match self.hittable.hit(&ray_ctx, 0.000_001, 99999.0) {
            Some(hit) => {
                let emitted = if hit.normal.dot(&ray_ctx.ray.direction.normalize()) < 0.0 {
                    hit.material.emmit(&hit)
                } else {
                    Color::from_element(0.0)
                };
                return match hit
                    .material
                    .scatter(ray_ctx, &hit)
                    .and_then(RayCtx::validate) {
                    Some(scattered) => { emitted + scattered.attenuation.component_mul(&self.color(&scattered)) }
                    None => emitted
                };
            }
            None => {
                return self.miss_shader.borrow()(&ray_ctx.ray);
            }
        };
    }
}
