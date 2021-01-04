use itertools::Itertools;

use super::{AABB, Hit, Hittable, RayCtx, V3};
use crate::ray::Ray;
use nalgebra::{Rotation3, Vector3, Unit};
use crate::types::{P3, Distance, Time, Angle, Scale};
use crate::consts::PI;

pub trait FlipNormalsOp<I, O>{
    fn flip_normals(self) -> O;
}
pub trait TranslateOp<I, O>{
    fn translate(self, offset: V3) -> Translate<I>;
}

pub trait RotateYOp<I> {
    fn rotate_y(self, angle: Angle) -> RotateY<I>;
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
    fn hit(&self, ray: &RayCtx, dist_min: Distance, dist_max: Distance) -> Option<Hit> {
        self.0
            .hit(ray, dist_min, dist_max)
            .map(|hit| Hit { normal: -hit.normal, ..hit })
    }

    fn bounding_box(&self, t_min: Time, t_max: Time) -> Option<AABB> {
        self.0.bounding_box(t_min, t_max)
    }

    fn pdf_value(&self, origin: &P3, direction: &Unit<V3>, hit: &Hit) -> f64 {
        self.0.pdf_value(origin, direction, hit)
    }

    fn random(&self, origin: &P3) -> Unit<V3> {
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
    fn hit(&self, ray_ctx: &RayCtx, dist_min: Distance, dist_max: Distance) -> Option<Hit> {
        let moved_r = RayCtx {
            ray: Ray {
                origin: &ray_ctx.ray.origin - &self.offset,
                direction: ray_ctx.ray.direction,
            },
            attenuation: ray_ctx.attenuation,
            time: ray_ctx.time,
            ttl: ray_ctx.ttl,
        };
        self.target
            .hit(&moved_r, dist_min, dist_max)
            .map(|hit| Hit { point: hit.point + self.offset, ..hit })
    }

    fn bounding_box(&self, t_min: Time, t_max: Time) -> Option<AABB> {
        self.target
            .bounding_box(t_min, t_max)
            .map(|aabb| AABB::new(aabb.min + self.offset, aabb.max + self.offset))
    }

    fn pdf_value(&self, origin: &P3, direction: &Unit<V3>, hit: &Hit) -> f64 {
        self.target.pdf_value(&(*origin - self.offset), direction, hit)
    }

    fn random(&self, origin: &P3) -> Unit<V3> {
        self.target.random(&(*origin - self.offset))
    }
}

#[derive(Debug,Clone)]
pub struct RotateY<T> {
    target: T,
    forward: Rotation3<Angle>,
    aabb: Option<AABB>,
}

impl<I: Hittable + Sized> RotateYOp<I> for I {
    fn rotate_y(self, angle_degrees: Angle) -> RotateY<I> {
        let angle = Angle::to_radians(-angle_degrees);
        let (sin, cos) = Angle::sin_cos(angle);

        let aabb = self.bounding_box(0.0, 1.0).map(|aabb| {
            let min_max = [&aabb.min, &aabb.max, &aabb.min, &aabb.max];
            let (min_x, max_x) = min_max.iter()
                .tuple_combinations()
                .map(|(a, b)| cos * a.x + sin * b.z)
                .minmax().into_option().unwrap();

            let (min_z, max_z) = min_max.iter()
                .tuple_combinations()
                .map(|(a, b)| -sin * a.x + cos * b.z)
                .minmax().into_option().unwrap();

            AABB::new(
                P3::new(min_x, aabb.min.y, min_z),
                P3::new(max_x, aabb.max.y, max_z),
            )
        });
        RotateY { target: self, forward : Rotation3::from_axis_angle(&Vector3::y_axis(), angle), aabb }
    }
}

impl<T: Hittable> Hittable for RotateY<T> {
    fn hit(&self, ray_ctx: &RayCtx, dist_min: Distance, dist_max: Distance) -> Option<Hit> {
        let ray = ray_ctx.ray;
        let origin = self.forward.transform_point(&ray.origin);
        let direction =  &self.forward * &ray.direction;

        let rotated_ray = RayCtx { ray: Ray { origin, direction }, ..*ray_ctx };
        self.target.hit(&rotated_ray, dist_min, dist_max)
            .map(|hit| {
                let point =  self.forward.inverse_transform_point(&hit.point);
                let normal = Unit::new_unchecked(self.forward.inverse_transform_vector(&hit.normal));

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

    fn pdf_value(&self, origin: &P3, direction: &Unit<V3>, hit: &Hit) -> f64 {
        let origin = self.forward.transform_point(&origin);
        let direction = self.forward.inverse_transform_vector(direction);
        self.target.pdf_value(&origin, &Unit::new_unchecked(direction), hit)
    }

    // todo: check rotation, unsure, if we need backward transformation here
    fn random(&self, origin: &P3) -> Unit<V3> {
        Unit::new_unchecked(self.forward.inverse_transform_vector(&self.target.random(&self.forward.transform_point(&origin))))
    }
}

#[cfg(test)]
mod test {
    use crate::hittable::{Sphere, Hittable, RotateYOp, Hit, AABox};
    use crate::texture::Color;
    use crate::types::{V3, P3};
    use crate::material::{Dielectric, Lambertian};
    use std::sync::Arc;
    use crate::hittable::test::test_pdf_integration;
    use crate::random::{next_std_in_range, rand_in_unit_sphere, next_std};

    #[test]
    fn test_rotation_pdf_sphere(){
        let center: P3 = 6.0 * rand_in_unit_sphere();
        let radius = 1.0 + next_std();
        let sphere = Sphere::new(center, radius, Dielectric::new(1.5));

        let rot_range = (-180.0)..180.0;
        for _ in 0..100 {
            let angle = next_std_in_range(&rot_range);
            let rotated = sphere.clone()
                .rotate_y(angle);
            test_pdf_integration(rotated, 1000);
        };
    }
    #[test]
    fn test_rotation_pdf_aabox() {
        let count = 10_000;

        let center: P3 = 5.0 * rand_in_unit_sphere();
        let h_width = 1.0 + next_std();
        let h_height = 1.0 + next_std();
        let h_depth = 1.0 + next_std();

        let aabox = AABox::mono(
            (center.x - h_width)..(center.x + h_width),
            (center.y - h_height)..(center.y + h_height),
            (center.z - h_depth)..(center.z + h_depth),
            Arc::new(Lambertian::new(Color(V3::from_element(1.0)))),
        );
        let rot_range = (-180.0)..180.0;
        for _ in 0..100 {
            let angle = next_std_in_range(&rot_range);
            test_pdf_integration(aabox.clone().rotate_y(angle), count);
        }
    }
}
