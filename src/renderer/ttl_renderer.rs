use super::{Hittable, RayCtx, Renderer};
use crate::types::{Color, ColorComponent};

#[allow(dead_code)]
pub struct TtlRenderer{
    pub hittable: Box<dyn Hittable>,
    pub ttl: i32
}
impl Renderer for TtlRenderer {
    fn color(&self, ray_ctx: &RayCtx) -> Color {
        match self.hittable.hit(&ray_ctx, 0.0001, 99999.0) {
            Some(hit) => {
                return match hit
                    .material
                    .scatter(ray_ctx, &hit)
                    .and_then(RayCtx::validate) {
                    Some(scattered) => {
                        ttl_color(scattered.ttl, self.ttl).component_mul(&self.color(&scattered))
                    }
                    None => ttl_color(ray_ctx.ttl, self.ttl)
                };
            }
            None => {
                return ttl_color(ray_ctx.ttl, self.ttl)
            }
        };
    }
}

fn ttl_color(ray_ttl: i32, max_ttl:i32) -> Color {
    (ray_ttl as ColorComponent / max_ttl as ColorComponent) * Color::from_element(1.0)
}
