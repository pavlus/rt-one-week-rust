use std::f64::consts::PI;

use itertools::Itertools;

use super::{AABB, Hit, Hittable, RayCtx, V3};
use crate::ray::Ray;

pub trait FlipNormalsOp<I, O>{
    fn flip_normals(self) -> O;
}
pub trait TranslateOp<I, O>{
    fn translate(self, offset: V3) -> Translate<I>;
}

pub trait RotateYOp<I> {
    fn rotate_y(self, angle: f64) -> RotateY<I>;
}


#[derive(Debug, Clone)]
pub struct FlipNormals<T>(T);
impl<I: Hittable + Sized> FlipNormalsOp<I, FlipNormals<I>> for I {
    fn flip_normals(self) -> FlipNormals<I> {
        FlipNormals(self)
    }
}
impl<I: Hittable> FlipNormalsOp<FlipNormals<I>, I> for FlipNormals<I> {
    fn flip_normals(self) -> I {
        self.0
    }
}

impl<T: Hittable> Hittable for FlipNormals<T> {
    fn hit(&self, ray: &RayCtx, dist_min: f64, dist_max: f64) -> Option<Hit> {
        self.0
            .hit(ray, dist_min, dist_max)
            .map(|hit| Hit { normal: -hit.normal, ..hit })
    }

    fn bounding_box(&self, t_min: f32, t_max: f32) -> Option<AABB> {
        self.0.bounding_box(t_min, t_max)
    }

    fn pdf_value(&self, origin: &V3, direction: &V3, hit: &Hit) -> f64 {
        self.0.pdf_value(origin, direction, hit)
    }

    fn random(&self, origin: &V3) -> V3 {
        self.0.random(origin)
    }
}


#[derive(Debug, Clone)]
pub struct Translate<T> {
    target: T,
    offset: V3,
}

impl<T: Hittable + Sized> TranslateOp<T, Translate<T>> for T {
    fn translate(self, offset: V3) -> Translate<T> {
        Translate {
            target: self,
            offset,
        }
    }
}

impl<T: Hittable + Sized> TranslateOp<T, Translate<T>> for Translate<T> {
    fn translate(self, offset: V3) -> Translate<T> {
        Translate {
            target: self.target,
            offset: self.offset + offset,
        }
    }
}

impl<T: Hittable> Hittable for Translate<T> {
    fn hit(&self, ray_ctx: &RayCtx, dist_min: f64, dist_max: f64) -> Option<Hit> {
        let moved_r = RayCtx {
            ray: Ray {
                origin: ray_ctx.ray.origin - self.offset,
                direction: ray_ctx.ray.direction,
            },
            ..*ray_ctx
        };
        self.target
            .hit(&moved_r, dist_min, dist_max)
            .map(|hit| Hit { point: hit.point + self.offset, ..hit })
    }

    fn bounding_box(&self, t_min: f32, t_max: f32) -> Option<AABB> {
        self.target
            .bounding_box(t_min, t_max)
            .map(|aabb| AABB::new(aabb.min + self.offset, aabb.max + self.offset))
    }

    fn pdf_value(&self, origin: &V3, direction: &V3, hit: &Hit) -> f64 {
        self.target.pdf_value(&(*origin - self.offset), direction, hit)
    }

    fn random(&self, origin: &V3) -> V3 {
        self.target.random(&(*origin - self.offset))
    }
}

#[derive(Debug,Clone)]
pub struct RotateY<T> {
    target: T,
    sin: f64,
    cos: f64,
    aabb: Option<AABB>,
}

impl<I: Hittable + Sized> RotateYOp<I> for I {
    fn rotate_y(self, angle: f64) -> RotateY<I> {
        let (sin, cos) = f64::sin_cos((PI / 180.0) * angle);

        let aabb = self.bounding_box(0.0, 1.0).map(|aabb| {
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
        RotateY { target: self, sin, cos, aabb }
    }
}

impl<T: Hittable> Hittable for RotateY<T> {
    fn hit(&self, ray_ctx: &RayCtx, dist_min: f64, dist_max: f64) -> Option<Hit> {
        let ray = ray_ctx.ray;
        let origin = self.forward_transform(ray.origin);
        let direction = self.forward_transform(ray.direction);

        let rotated_ray = RayCtx { ray: Ray { origin, direction }, ..*ray_ctx };
        self.target.hit(&rotated_ray, dist_min, dist_max)
            .map(|hit| {
                let point = self.backward_transform(hit.point);
                let normal = self.backward_transform(hit.normal);

                Hit {
                    point,
                    normal,
                    ..hit
                }
            })
    }

    fn bounding_box(&self, _: f32, _: f32) -> Option<AABB> {
        self.aabb
    }

    fn pdf_value(&self, origin: &V3, direction: &V3, hit: &Hit) -> f64 {
        let origin = self.forward_transform(*origin);
        let direction = self.forward_transform(*direction);
        self.target.pdf_value(&origin, &direction, hit)
    }

    // todo: check rotation, unsure, if we need backward transformation here
    fn random(&self, origin: &V3) -> V3 {
        self.backward_transform(self.target.random(&self.forward_transform(*origin)))
    }
}

impl<I> RotateY<I> {
    #[inline]
    fn forward_transform(&self, vec: V3) -> V3 {
        let sin = self.sin;
        let cos = self.cos;
        mul_by_matrix!(vec,
            cos, 0.0, -sin,
            0.0, 1.0, 0.0,
            sin, 0.0, cos
        )
    }
    #[inline]
    fn backward_transform(&self, vec: V3) -> V3 {
        let sin = self.sin;
        let cos = self.cos;
        mul_by_matrix!(vec,
            cos, 0.0, sin,
            0.0, 1.0, 0.0,
           -sin, 0.0, cos
        )
    }
}

#[cfg(test)]
mod test {
    use crate::hittable::{Sphere, Hittable, RotateYOp, Hit, AABox};
    use crate::texture::Color;
    use crate::vec::V3;
    use crate::material::{Dielectric, Lambertian};
    use std::sync::Arc;
    use crate::hittable::test::test_pdf_integration;
    use crate::random::{next_std_f64_in_range, rand_in_unit_sphere, next_std_f64};

    #[test]
    fn test_rotation_pdf_sphere(){
        let center = 6.0 * rand_in_unit_sphere();
        let radius = 1.0 + next_std_f64();
        let sphere = Sphere::new(center, radius, Dielectric::new(1.5));

        let rot_range = (-180.0)..180.0;
        for _ in 0..100 {
            let angle = next_std_f64_in_range(&rot_range);
            let rotated = sphere.clone()
                .rotate_y(angle);
            test_pdf_integration(rotated, 1000);
        };
    }
    #[test]
    fn test_rotation_pdf_aabox() {
        let count = 10_000;

        let center: V3 = 5.0 * rand_in_unit_sphere();
        let h_width = 1.0 + next_std_f64();
        let h_height = 1.0 + next_std_f64();
        let h_depth = 1.0 + next_std_f64();

        let aabox = AABox::mono(
            (center.x - h_width)..(center.x + h_width),
            (center.y - h_height)..(center.y + h_height),
            (center.z - h_depth)..(center.z + h_depth),
            Arc::new(Lambertian::new(Color(V3::ones()))),
        );
        let rot_range = (-180.0)..180.0;
        for _ in 0..100 {
            let angle = next_std_f64_in_range(&rot_range);
            test_pdf_integration(aabox.clone().rotate_y(angle), count);
        }
    }
}
