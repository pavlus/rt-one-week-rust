use crate::random;

use super::{Hit, Material, RayCtx, V3};
use nalgebra::{Reflection, Unit};
use crate::types::{Scale, Color};

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Metal {
    albedo: Color,
    fuzz: Scale,
}

impl Metal {
    pub fn new(albedo: Color) -> Metal { Metal { albedo, fuzz: 0.0 } }
    pub fn new_fuzzed(albedo: Color, fuzz_factor: Scale) -> Metal {
        Metal { albedo, fuzz: if fuzz_factor < 1.0 { fuzz_factor } else { 1.0 } }
    }

    fn fuzz(self, vector: &V3) -> V3 {
        self.fuzz * random::rand_in_unit_sphere().coords + vector
    }
}

impl Material for Metal {
    fn scatter(&self, ray_ctx: &RayCtx, &hit: &Hit) -> Option<RayCtx> {
        let mut reflected = ray_ctx.ray.direction.normalize();
        Reflection::new(hit.normal, 0.0).reflect(&mut reflected);
        if reflected.dot(&hit.normal) > 0.0 {
            Some(ray_ctx.produce(hit.point, Unit::new_normalize(self.fuzz(&reflected)), self.albedo))
        } else {
            None
        }
    }

}
