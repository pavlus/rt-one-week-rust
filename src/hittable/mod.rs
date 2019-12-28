use std::borrow::{Borrow, BorrowMut};
use std::f64::{MAX, MIN};
use std::f64::consts;
use std::f64::consts::PI;
use std::fmt::{Debug, Error, Formatter};
use std::ops::Range;
use std::sync::Arc;

use itertools::Itertools;
use rand::distributions::Uniform;
use rand::distributions::uniform::UniformFloat;
use rand::RngCore;

pub use aarect::*;
pub use list::*;
pub use sphere::*;

use crate::aabb::AABB;
use crate::material::{Lambertian, Material};
use crate::random::{next_f64, next_std_f64, rand_in_unit_sphere};
use crate::ray::Ray;
use crate::texture::{Color, Texture};
use crate::vec::{Axis, V3};

mod sphere;
mod aarect;
mod list;

#[derive(Copy, Clone)]
pub struct Hit<'a> {
    pub point: V3,
    pub normal: V3,
    pub u: f64,
    pub v: f64,
    pub material: &'a dyn Material,
    pub dist: f64,
}

//impl Eq for Hit {}

impl<'a> Hit<'a> {
    pub fn new(dist: f64, p: V3, n: V3, material: &'a dyn Material, u: f64, v: f64) -> Hit<'a> {
        return Hit { dist, point: p, normal: n, material, u, v };
    }
}

pub trait Hittable: Debug + Sync {
    fn hit(&self, ray: &Ray, dist_min: f64, dist_max: f64) -> Option<Hit>;
    fn bounding_box(&self, t_min: f32, t_max: f32) -> Option<AABB> { None }
}

pub trait Instance {
    fn flip_normals(self) -> Box<dyn Hittable>;
    fn translate(self, offset: V3) -> Box<dyn Hittable>;
    fn rotate_y(self, angle: f64) -> Box<dyn Hittable>;
}

impl Instance for Box<dyn Hittable> {
    fn flip_normals(self) -> Box<dyn Hittable> { Box::new(FlipNormals(self)) }

    fn translate(self, offset: V3) -> Box<dyn Hittable> {
        Box::new(Translate {
            target: self,
            offset,
        })
    }

    fn rotate_y(self, angle: f64) -> Box<dyn Hittable> {
        RotateY::new(self, angle)
    }
}

impl<T: Hittable + 'static> Instance for T {
    fn flip_normals(self) -> Box<dyn Hittable> { Box::new(FlipNormals(Box::new(self))) }

    fn translate(self, offset: V3) -> Box<dyn Hittable> {
        Box::new(Translate {
            target: Box::new(self),
            offset,
        })
    }

    fn rotate_y(self, angle: f64) -> Box<dyn Hittable> {
        RotateY::new(Box::new(self), angle)
    }
}

#[derive(Debug)]
struct FlipNormals(Box<dyn Hittable>);

impl Hittable for FlipNormals {
    fn hit(&self, ray: &Ray, dist_min: f64, dist_max: f64) -> Option<Hit> {
        self.0
            .hit(ray, dist_min, dist_max)
            .map(|hit| Hit { normal: -hit.normal, ..hit })
    }

    fn bounding_box(&self, t_min: f32, t_max: f32) -> Option<AABB> {
        self.0.bounding_box(t_min, t_max)
    }
}


#[derive(Debug)]
struct Translate {
    target: Box<dyn Hittable>,
    offset: V3,
}

impl Hittable for Translate {
    fn hit(&self, ray: &Ray, dist_min: f64, dist_max: f64) -> Option<Hit> {
        let moved_r = Ray { origin: ray.origin - self.offset, ..*ray };
        self.target
            .hit(&moved_r, dist_min, dist_max)
            .map(|hit| Hit { point: hit.point + self.offset, ..hit })
    }

    fn bounding_box(&self, t_min: f32, t_max: f32) -> Option<AABB> {
        self.target
            .bounding_box(t_min, t_max)
            .map(|aabb| AABB::new(aabb.min + self.offset, aabb.max + self.offset))
    }
}

#[derive(Debug)]
struct RotateY {
    target: Box<dyn Hittable>,
    sin: f64,
    cos: f64,
    aabb: Option<AABB>,
}

impl RotateY {
    fn new(target: Box<dyn Hittable>, angle: f64) -> Box<dyn Hittable> {
        Box::new(RotateY::_new2(target, angle))
    }

    fn _new1(target: Box<dyn Hittable>, angle: f64) -> RotateY {
        let (sin, cos) = f64::sin_cos((PI / 180.0) * angle);

        let aabb = target.bounding_box(0.0, 1.0).map(|aabb| {
            macro_rules! nx { ($x:expr, $z: expr) => (cos * $x + sin * $z) }
            macro_rules! nz { ($x:expr, $z: expr) => (-sin * $x + cos * $z) }
            macro_rules! reduce { ($comp: path, $rot:tt) => ($comp(
                $comp($rot!(aabb.min.x, aabb.min.z), $rot!(aabb.max.x, aabb.max.z)),
                $comp($rot!(aabb.min.x, aabb.max.z), $rot!(aabb.max.x, aabb.min.z)),
            )) }

            AABB::new(
                V3::new(reduce!(f64::min, nx), aabb.min.y, reduce!(f64::min, nz)),
                V3::new(reduce!(f64::max, nx), aabb.max.y, reduce!(f64::max, nz)),
            )
        });
        RotateY { target, sin, cos, aabb }
    }

    fn _new2(target: Box<dyn Hittable>, angle: f64) -> RotateY {
        let (sin, cos) = f64::sin_cos((PI / 180.0) * angle);

        let aabb = target.bounding_box(0.0, 1.0).map(|aabb| {
            let min_max = [aabb.min, aabb.max, aabb.min, aabb.max];
            let (min_x, max_x) = min_max.iter()
                .tuple_combinations()
                .map(|(a, b)| cos * a.x + sin * b.z)
                .minmax().into_option().unwrap();

            let (min_z, max_z) = min_max.iter()
                .tuple_combinations()
                .map(|(a, b)| -sin * a.x + cos * b.z)
                .minmax().into_option().unwrap();

            AABB::new(
                V3::new(min_x, aabb.min.y, min_z),
                V3::new(max_x, aabb.max.y, max_z),
            )
        });
        RotateY { target, sin, cos, aabb }
    }
}

// todo: implementation for X and Z + benchmark to compute instead of saving for hardcoded time
#[test]
fn test_rot_y() {
    let h = || AABox::mono(
        0.0..1.0, 0.0..2.0, 0.0..3.0,
        Arc::new(Lambertian::new(V3::new(0.0, 0.0, 0.0))),
    );
    for a in 0..3600 {
        let aabb1 = RotateY::_new1(Box::new(h()), a as f64 / 10.0).aabb;
        let aabb2 = RotateY::_new2(Box::new(h()), a as f64 / 10.0).aabb;
        assert_eq!(aabb1, aabb2);
    }
}

impl Hittable for RotateY {
    fn hit(&self, ray: &Ray, dist_min: f64, dist_max: f64) -> Option<Hit> {
        let sin = self.sin;
        let cos = self.cos;

        let origin = mul_by_matrix!(ray.origin,
            cos, 0.0, -sin,
            0.0, 1.0, 0.0,
            sin, 0.0, cos
        );
        let direction = mul_by_matrix!(ray.direction,
            cos, 0.0, -sin,
            0.0, 1.0, 0.0,
            sin, 0.0, cos
        );

        let rotated_ray = Ray { origin, direction, ..*ray };
        self.target.hit(&rotated_ray, dist_min, dist_max)
            .map(|hit| {
                let point = mul_by_matrix!(hit.point,
                     cos, 0.0, sin,
                     0.0, 1.0, 0.0,
                    -sin, 0.0, cos
                );
                let normal = mul_by_matrix!(hit.normal,
                     cos, 0.0, sin,
                     0.0, 1.0, 0.0,
                    -sin, 0.0, cos
                );

                Hit {
                    point,
                    normal,
                    ..hit
                }
            })
    }

    fn bounding_box(&self, t_min: f32, t_max: f32) -> Option<AABB> {
        self.aabb
    }
}

#[derive(Debug)]
pub struct AABox {
    faces: HittableList,
    aabb: AABB,
}

impl AABox {
    pub fn new(
        x: Range<f64>,
        y: Range<f64>,
        z: Range<f64>,
        top: Arc<dyn Material>,
        bottom: Arc<dyn Material>,
        front: Arc<dyn Material>,
        left: Arc<dyn Material>,
        back: Arc<dyn Material>,
        right: Arc<dyn Material>,
    ) -> AABox {
        let faces: Vec<Box<dyn Hittable>> = vec![
            XYRect::new(x.clone(), y.clone(), z.start, back).flip_normals(),
            Box::new(XYRect::new(x.clone(), y.clone(), z.end, front)),
            XZRect::new(x.clone(), z.clone(), y.start, bottom).flip_normals(),
            Box::new(XZRect::new(x.clone(), z.clone(), y.end, top)),
            YZRect::new(y.clone(), z.clone(), x.start, right).flip_normals(),
            Box::new(YZRect::new(y.clone(), z.clone(), x.end, left)),
        ];

        AABox {
            faces: HittableList::new(faces),
            aabb: AABB::new(V3::new(x.start, y.start, z.start),
                            V3::new(x.end, y.end, z.end)),
        }
    }
    pub fn mono(
        x: Range<f64>,
        y: Range<f64>,
        z: Range<f64>,
        material: Arc<dyn Material>,
    ) -> AABox {
        AABox::new(x, y, z,
                   Arc::clone(&material),
                   Arc::clone(&material),
                   Arc::clone(&material),
                   Arc::clone(&material),
                   Arc::clone(&material),
                   Arc::clone(&material),
        )
    }
}

impl Hittable for AABox {
    fn hit(&self, ray: &Ray, dist_min: f64, dist_max: f64) -> Option<Hit> {
        self.faces.hit(ray, dist_min, dist_max)
    }

    fn bounding_box(&self, t_min: f32, t_max: f32) -> Option<AABB> {
        Some(self.aabb)
    }
}

#[derive(Debug)]
pub struct ConstantMedium {
    boundary: Box<dyn Hittable>,
    density: f64,
    phase_function: Box<dyn Material>,
}

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
        self.boundary.hit(ray, 0.0, MAX).and_then(|enter_hit| {
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
                            V3::new(1.0, 0.0, 0.0),
                            self.phase_function.borrow(),
                            0.0, 0.0,
                        ))
                    } else { None }
                } else { None }
            })
        })
    }

    fn bounding_box(&self, t_min: f32, t_max: f32) -> Option<AABB> {
        self.boundary.bounding_box(t_min, t_max)
    }
}

#[derive(Debug)]
pub struct Isotropic {
    albedo: Box<dyn Texture>
}

impl Isotropic {
    pub fn new(albedo: Box<dyn Texture>) -> Isotropic {
        Isotropic { albedo }
    }
}

impl Material for Isotropic {
    /// Isotropic material has a unit-sphere BSDF,
    /// this means that amount of reflected light
    /// is equal to the amount of transmitted light
    /// probability of reflection in any direction is the same
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<Ray> {
        Some(
            ray.produce(
                hit.point,
                rand_in_unit_sphere(),
                self.albedo.value(hit.u, hit.v, hit.point).0)
        )
    }
}
