use nalgebra::{Reflection, Unit};

use crate::random;
use crate::types::{Color, Direction, Scale};

use super::{Hit, Material, RayCtx, V3};

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Dielectric {
    albedo: Color,
    ref_idx: Scale,
}

impl Dielectric {
    pub fn new(ref_idx: Scale) -> Dielectric { Dielectric { albedo: Color::from_element(1.0), ref_idx } }
    pub fn new_colored(albedo: Color, ref_idx: Scale) -> Dielectric {
        Dielectric { albedo, ref_idx }
    }
    fn schlick(self, cosine: Scale) -> Scale {
        let mut r0 = (1.0 - self.ref_idx) / (1.0 + self.ref_idx);
        r0 *= r0;
        return r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0);
    }


    fn refract(v: &Direction, normal: &Direction, ni_over_nt: Scale) -> Option<Direction> {
        let unit = v.as_ref();
        let dt = unit.dot(normal);
        let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
        return if discriminant > 0.0 {
            Some(Unit::new_normalize(ni_over_nt * (v.as_ref() - dt * normal.as_ref()) - discriminant.sqrt() * normal.as_ref()))
        } else { None };
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray_ctx: &RayCtx, &hit: &Hit) -> Option<RayCtx> {
        let unit_direction = ray_ctx.ray.direction;

        let cosine: Scale;
        let outward_normal: Direction;
        let ni_over_nt: Scale;

        let vector_cosine = unit_direction.dot(&hit.normal);
        if vector_cosine > 0.0 {
            outward_normal = -hit.normal;
            ni_over_nt = self.ref_idx;
            cosine = (1.0 - self.ref_idx * self.ref_idx * (1.0 - vector_cosine * vector_cosine)).sqrt();
        } else {
            outward_normal = hit.normal;
            ni_over_nt = 1.0 / self.ref_idx;
            cosine = -vector_cosine;
        }

        Dielectric::refract(&unit_direction, &outward_normal, ni_over_nt)
            .filter(|_| self.schlick(cosine) < random::next_std())
            .map(|refracted| ray_ctx.produce(hit.point, refracted, self.albedo))
            .or_else(|| {
                let mut reflected = ray_ctx.ray.direction.clone_owned();
                Reflection::new(outward_normal, 0.0)
                    .reflect(&mut reflected);
                Some(ray_ctx.produce(hit.point, Unit::new_unchecked(reflected), Color::from_element(1.0)))
            })
    }
}
