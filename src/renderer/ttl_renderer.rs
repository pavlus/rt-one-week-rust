use super::{Hittable, Ray, Renderer, V3};

#[allow(dead_code)]
pub struct TtlRenderer{
    pub hittable: Box<dyn Hittable>,
    pub ttl: i32
}
impl Renderer for TtlRenderer {
    fn color(&self, r: &Ray) -> V3 {
        match self.hittable.hit(&r, 0.0001, 99999.0) {
            Some(hit) => {
                return match hit
                    .material
                    .scatter(r, &hit)
                    .and_then(Ray::validate) {
                    Some(scattered) => {
                        ttl_color(scattered.ttl, self.ttl) * self.color(&scattered)
                    }
                    None => ttl_color(r.ttl, self.ttl)
                };
            }
            None => {
                return ttl_color(r.ttl, self.ttl)
            }
        };
    }
}

fn ttl_color(ray_ttl: i32, max_ttl:i32) -> V3{
    (ray_ttl as f64 / max_ttl as f64)* V3::ones()
}
