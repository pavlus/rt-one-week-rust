use crate::ray::Ray;
use crate::vec::V3;
use crate::hittable::Hittable;

pub trait Renderer {
    fn color(&self, r: &Ray) -> V3;
}

pub struct RgbRenderer {
    pub hittable: Box<dyn Hittable>
}

impl Renderer for RgbRenderer {
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
//                let unit_direction = r.direction.unit();
//                let t: f64 = 0.5 * (unit_direction.y + 1.0);
                return V3::new(0.05088, 0.05088, 0.05088);
//                return (1.0 - t) * V3::ones() + t * V3::new(0.5, 0.7, 1.0);
            }
        };
    }
}

pub struct TtlRenderer{
    hittable: Box<dyn Hittable>,
    ttl: i32
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
