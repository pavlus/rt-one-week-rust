use std::borrow::Borrow;
use std::f64::{MAX, MIN};

use crate::material::Isotropic;
use crate::random::{next_f64, rand_in_unit_sphere};
use crate::texture::Texture;

use super::{AABB, Hit, Hittable, Material, Ray, V3};

#[derive(Debug)]
pub struct ConstantMedium {
    boundary: Box<dyn Hittable>,
    density: f64,
    phase_function: Box<dyn Material>,
}
// todo: inject material
impl ConstantMedium {
    pub fn new(boundary: Box<dyn Hittable>,
               density: f64,
               texture: Box<dyn Texture>,
    ) -> ConstantMedium {
        ConstantMedium {
            boundary,
            density,
            phase_function: Box::new(Isotropic::new(texture)),
        }
    }
}

impl Hittable for ConstantMedium {
    fn hit(&self, ray: &Ray, dist_min: f64, dist_max: f64) -> Option<Hit> {
        self.boundary.hit(ray, MIN, MAX).and_then(|enter_hit| {
            self.boundary.hit(ray, enter_hit.dist + 0.001, MAX).and_then(|exit_hit| {
                let enter_dist = f64::max(dist_min, enter_hit.dist);
                let exit_dist = f64::min(exit_hit.dist, dist_max);
                if enter_dist < exit_dist {
                    // TODO: describe why such distribution?
                    //  isotropic scattering follows Poisson point process?
                    let hit_dist = next_f64(rand_distr::Exp1) / self.density;
                    let inner_travel_distance = (exit_dist - enter_dist) * ray.direction.length();
                    if hit_dist < inner_travel_distance {
                        let dist = enter_dist + hit_dist / ray.direction.length();
                        Some(Hit::new(
                            dist,
                            ray.point_at(dist),
                            rand_in_unit_sphere(),
                            self.phase_function.borrow(),
                            enter_hit.u, enter_hit.v,
                        ))
                    } else { None }
                } else { None }
            })
        })
    }

    #[inline]
    fn bounding_box(&self, t_min: f32, t_max: f32) -> Option<AABB> {
        self.boundary.bounding_box(t_min, t_max)
    }

    #[inline]
    fn pdf_value(&self, origin: &V3, direction: &V3, hit: &Hit) -> f64 {
        self.boundary.pdf_value(origin, direction, hit)
    }

    #[inline]
    fn random(&self, origin: &V3) -> V3 {
        self.boundary.random(origin)
    }

}

