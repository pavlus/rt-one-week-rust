use crate::random;

use super::{Hit, Material, RayCtx, V3};
use nalgebra::{Reflection, Unit};
use crate::scatter::Scatter;
use crate::types::{Scale, Color, Direction};

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct GlossyMetal {
    albedo: Color,
    fuzz: Scale,
}

impl GlossyMetal {
    pub fn new(albedo: Color, fuzz_factor: Scale) -> GlossyMetal {
        GlossyMetal { albedo, fuzz: if fuzz_factor < 1.0 { fuzz_factor } else { 1.0 } }
    }

    fn fuzz(self, vector: &V3) -> V3 {
        self.fuzz * random::rand_in_unit_sphere().coords + vector
    }
}

impl Material for GlossyMetal {
    fn scatter_with_pdf(&self, ray_ctx: RayCtx, hit: &Hit) -> Option<Scatter> {
        let mut reflected = ray_ctx.ray.direction.normalize();
        Reflection::new(hit.normal, 0.0).reflect(&mut reflected);
        if reflected.dot(&hit.normal) > 0.0 {
            Some(Scatter::Specular(ray_ctx.produce(hit.point, Unit::new_normalize(self.fuzz(&reflected))), self.albedo))
        } else {
            None
        }
    }
}


#[derive(PartialEq, Copy, Clone, Debug)]
pub struct PolishedMetal {
    albedo: Color,
}


impl PolishedMetal {
    pub fn new(albedo: Color) -> PolishedMetal { PolishedMetal { albedo } }
}


impl Material for PolishedMetal {
    fn scatter_with_pdf(&self, ray_ctx: RayCtx, hit: &Hit) -> Option<Scatter> {
        let mut reflected = ray_ctx.ray.direction.into_inner();
        Reflection::new(hit.normal, 0.0).reflect(&mut reflected);
        if reflected.dot(&hit.normal) > 0.0 {
            let reflected = Direction::new_unchecked(reflected);
            Some(Scatter::Specular(ray_ctx.produce(hit.point, reflected), self.albedo))
        } else {
            None
        }
    }
}
