use super::{Hittable, Ray, Renderer, V3};

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
