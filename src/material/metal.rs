use crate::random;

use super::{Hit, Material, RayCtx, V3};

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Metal {
    albedo: V3,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: V3) -> Metal { Metal { albedo, fuzz: 0.0 } }
    pub fn new_fuzzed(albedo: V3, fuzz_factor: f64) -> Metal {
        Metal { albedo, fuzz: if fuzz_factor < 1.0 { fuzz_factor } else { 1.0 } }
    }

    fn fuzz(self, vector: V3) -> V3 {
        self.fuzz * random::rand_in_unit_sphere() + vector
    }
}

impl Material for Metal {
    fn scatter(&self, ray_ctx: &RayCtx, &hit: &Hit) -> Option<RayCtx> {
        let unit_direction = ray_ctx.ray.direction.unit();
        let reflected = unit_direction.reflect(hit.normal);
        if reflected.dot(hit.normal) > 0.0 {
            Some(ray_ctx.produce(hit.point, self.fuzz(reflected), self.albedo))
        } else {
            None
        }
    }

    #[allow(unused_variables)]
    fn scattering_pdf(&self, hit: &Hit, direction: &V3) -> f64 {
        0.0
    }

}
