use std::borrow::Borrow;

use crate::material::Isotropic;
use crate::random::{rand_in_unit_sphere, next};
use crate::texture::Texture;

use super::{AABB, Hit, Hittable, Material, RayCtx, V3};
use crate::types::{P3, Distance, Time, Scale};
use nalgebra::Unit;

#[derive(Debug)]
pub struct ConstantMedium<B,M> {
    boundary: B,
    density: Scale,
    phase_function: M,
}
// todo: inject material
impl<B:Hittable, T: Texture> ConstantMedium<B, Isotropic<T>> {
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
    fn hit(&self, ray_ctx: &RayCtx, dist_min: Distance, dist_max: Distance) -> Option<Hit> {
        self.boundary.hit(ray_ctx, Distance::MIN, Distance::MAX).and_then(|enter_hit| {
            self.boundary.hit(ray_ctx, enter_hit.dist + 0.001, Distance::MAX).and_then(|exit_hit| {
                let enter_dist = Distance::max(dist_min, enter_hit.dist);
                let exit_dist = Distance::min(exit_hit.dist, dist_max);
                if enter_dist >= exit_dist {
                    return None
                }

                let ray = ray_ctx.ray;
                // TODO: describe why such distribution?
                //  isotropic scattering follows Poisson point process?
                let hit_dist: Distance = next::<Distance, rand_distr::Exp1>(rand_distr::Exp1) / self.density;
                let dir_norm = ray.direction.norm();
                let inner_travel_distance = (exit_dist - enter_dist) * dir_norm;
                if hit_dist >= inner_travel_distance {
                    return None
                }

                let dist = enter_dist + hit_dist / dir_norm;
                Some(Hit::new(
                    dist,
                    ray.point_at(dist),
                    Unit::new_unchecked(rand_in_unit_sphere().coords),
                    &self.phase_function,
                    enter_hit.u, enter_hit.v,
                ))
            })
        })
    }

    #[inline]
    fn bounding_box(&self, t_min: Time, t_max: Time) -> Option<AABB> {
        self.boundary.bounding_box(t_min, t_max)
    }

    #[inline]
    fn pdf_value(&self, origin: &P3, direction: &Unit<V3>, hit: &Hit) -> f64 {
        self.boundary.pdf_value(origin, direction, hit)
    }

    #[inline]
    fn random(&self, origin: &P3) -> Unit<V3> {
        self.boundary.random(origin)
    }

}

impl<B: Clone, M: Clone> Clone for ConstantMedium<B, M>{
    fn clone(&self) -> Self {
        ConstantMedium{
            boundary: self.boundary.clone(),
            density: self.density,
            phase_function: self.phase_function.clone(),
        }
    }
}
