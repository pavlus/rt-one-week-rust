use crate::material::Isotropic;
use crate::random::next;
use crate::texture::Texture;

use super::{AABB, Hit, Hittable, Material, RayCtx, V3};
use crate::types::{P3, Geometry, Scale, Probability, Timespan, Direction};
use nalgebra::Unit;
use crate::random;

#[derive(Debug)]
pub struct ConstantMedium<B, M> {
    boundary: B,
    density: Scale,
    phase_function: M,
}

// todo: inject material
impl<B: Hittable, T: Texture> ConstantMedium<B, Isotropic<T>> {
    pub fn new(boundary: B,
               density: Scale,
               texture: T,
    ) -> ConstantMedium<B, Isotropic<T>> {
        ConstantMedium {
            boundary,
            density,
            phase_function: Isotropic::new(texture),
        }
    }
}

impl<B:Hittable, M: Material> Hittable for ConstantMedium<B,M> {
    fn hit(&self, ray_ctx: &RayCtx, dist_min: Geometry, dist_max: Geometry) -> Option<Hit> {
        let enter_hit = self.boundary.hit(ray_ctx, Geometry::MIN, Geometry::MAX)?;
        let exit_hit = self.boundary.hit(ray_ctx, enter_hit.dist + 0.001, Geometry::MAX)?;
        let enter_dist = Geometry::max(dist_min, enter_hit.dist);
        let exit_dist = Geometry::min(exit_hit.dist, dist_max);
        if enter_dist >= exit_dist {
            return None
        }

        // random walk which follows Poisson point process, using exponential distribution
        let hit_dist: Geometry = next::<Geometry, rand_distr::Exp1>(rand_distr::Exp1) / self.density as Geometry;
        let inner_travel_distance = exit_dist - enter_dist;
        if hit_dist < inner_travel_distance {
            let dist = enter_dist + hit_dist;
            let ray = &ray_ctx.ray;
            Some(Hit::new(
                dist,
                ray.point_at(dist),
                Unit::new_unchecked(random::rand_in_unit_sphere().coords),
                &self.phase_function,
                enter_hit.uv
            ))
        } else {
            None
        }
    }

    #[inline]
    fn bounding_box(&self, timespan: Timespan) -> Option<AABB> {
        self.boundary.bounding_box(timespan)
    }

    #[inline]
    fn pdf_value(&self, origin: &P3, direction: &Direction, hit: &Hit) -> Probability {
        self.boundary.pdf_value(origin, direction, hit)
    }

    #[inline]
    fn random(&self, origin: &P3) -> Direction {
        self.boundary.random(origin)
    }
}

impl<B: Clone, M: Clone> Clone for ConstantMedium<B, M> {
    fn clone(&self) -> Self {
        ConstantMedium {
            boundary: self.boundary.clone(),
            density: self.density,
            phase_function: self.phase_function.clone(),
        }
    }
}
