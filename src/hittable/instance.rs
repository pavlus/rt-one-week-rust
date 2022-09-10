use itertools::Itertools;

use super::{AABB, Hit, Hittable, RayCtx, V3};
use crate::ray::Ray;
use nalgebra::{Rotation3, Vector3, Unit};
use crate::types::{P3, Geometry, Probability, Timespan};

pub trait FlipNormalsOp<I, O>{
    fn flip_normals(self) -> O;
}
pub trait TranslateOp<I, O>{
    fn translate(self, offset: V3) -> Translate<I>;
}

pub trait RotateYOp<I> {
    fn rotate_y(self, angle: Geometry) -> RotateY<I>;
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
    fn hit(&self, ray: &RayCtx, dist_min: Geometry, dist_max: Geometry) -> Option<Hit> {
        self.0
            .hit(ray, dist_min, dist_max)
            .map(|hit| Hit { normal: -hit.normal, ..hit })
    }

    fn bounding_box(&self, timespan: Timespan) -> Option<AABB> {
        self.0.bounding_box(timespan)
    }

    fn pdf_value(&self, origin: &P3, direction: &Unit<V3>, hit: &Hit) -> Probability {
        self.0.pdf_value(origin, direction, &hit)
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
    fn hit(&self, ray_ctx: &RayCtx, dist_min: Geometry, dist_max: Geometry) -> Option<Hit> {
        let moved_r = RayCtx {
            ray: Ray {
                origin: &ray_ctx.ray.origin - &self.offset,
                direction: ray_ctx.ray.direction,
            },
            ..*ray_ctx
        };
        self.target
            .hit(&moved_r, dist_min, dist_max)
            .map(|hit| Hit { point: hit.point + &self.offset, ..hit })
    }

    fn bounding_box(&self, timespan: Timespan) -> Option<AABB> {
        self.target
            .bounding_box(timespan)
            .map(|aabb| AABB::new(aabb.min + self.offset, aabb.max + self.offset))
    }

    fn pdf_value(&self, origin: &P3, direction: &Unit<V3>, hit: &Hit) -> Probability {
        self.target.pdf_value(&(*origin - self.offset), direction, &hit)
    }

    fn random(&self, origin: &P3) -> Unit<V3> {
        self.target.random(&(*origin - self.offset))
    }
}

#[derive(Debug,Clone)]
pub struct RotateY<T> {
    target: T,
    forward: Rotation3<Geometry>,
    aabb: Option<AABB>,
}

impl<I: Hittable> RotateYOp<I> for I {
    fn rotate_y(self, angle_degrees: Geometry) -> RotateY<I> {
        let angle = Geometry::to_radians(-angle_degrees);
        let (sin, cos) = Geometry::sin_cos(angle);

        let aabb = self.bounding_box(0.0..1.0).map(|aabb| {
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
    fn hit(&self, ray_ctx: &RayCtx, dist_min: Geometry, dist_max: Geometry) -> Option<Hit> {
        let ray = ray_ctx.ray;
        let origin = self.forward.transform_point(&ray.origin);
        let direction =  Unit::new_unchecked(self.forward.transform_vector(&ray.direction));

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

    fn bounding_box(&self, _: Timespan) -> Option<AABB> {
        self.aabb
    }

    fn pdf_value(&self, origin: &P3, direction: &Unit<V3>, hit: &Hit) -> Probability {
        let origin = self.forward.transform_point(&origin);
        let direction = Unit::new_unchecked(self.forward.transform_vector(direction));
        let hit = Hit{
            point: origin,
            normal: Unit::new_unchecked(self.forward.transform_vector(&hit.normal)),
                ..*hit
        };
        self.target.pdf_value(&origin, &direction, &hit)
    }

    // todo: check rotation, unsure, if we need backward transformation here
    fn random(&self, origin: &P3) -> Unit<V3> {
        Unit::new_unchecked(self.forward.inverse_transform_vector(&self.target.random(&self.forward.transform_point(&origin))))
    }
}

#[cfg(test)]
mod test {
    use nalgebra::Unit;
    use crate::hittable::{Sphere, RotateYOp, AABox, Hittable};
    use crate::types::{Color, Geometry, P3};
    use crate::material::NoMat;
    use crate::hittable::test::test_pdf_integration;
    use crate::random::{next_std_in_range, rand_in_unit_sphere, next_std};
    use crate::ray::RayCtx;
    use crate::V3;

    macro_rules! assert_eq_eps {
        {$left: expr, $right: expr, $epsilon: expr} => {
            let difference = ($left - $right).norm().abs();
            assert!(
                difference <= $epsilon,
                "Difference {} between {} and {} is greater than {}",
                difference, $left, $right, $epsilon);
        }
    }

    #[test]
    fn test_sphere_rotation_pdf_converges(){
        let count = 5_000;

        let radius = 4.0 * (1.0 + next_std::<Geometry>());
        let sphere = Sphere::new(P3::default(), radius, NoMat);

        let angle = next_std_in_range(&(-180.0..180.0));
        let rotated = sphere.clone()
            .rotate_y(angle);
        test_pdf_integration(rotated, count);
    }

    #[test]
    fn test_offset_aabox_rotation_pdf_converges() {
        let count = 5_000;

        let center: P3 = 5.0 * rand_in_unit_sphere();
        let h_width = 1.0 + next_std::<Geometry>();
        let h_height = 1.0 + next_std::<Geometry>();
        let h_depth = 1.0 + next_std::<Geometry>();

        let aabox = AABox::mono(
            (center.x - h_width)..(center.x + h_width),
            (center.y - h_height)..(center.y + h_height),
            (center.z - h_depth)..(center.z + h_depth),
            NoMat,
        );
        let rot_range = (-180.0)..180.0;
        for _ in 0..50 {
            let angle = next_std_in_range(&rot_range);
            test_pdf_integration(aabox.clone().rotate_y(angle), count);
        }
    }

    #[test]
    fn test_central_aabox_rotation_pdf_converges() {
        let count = 10_000;

        let h_width = 1.0 + next_std::<Geometry>();
        let h_height = 1.0 + next_std::<Geometry>();
        let h_depth = 1.0 + next_std::<Geometry>();

        let aabox = AABox::mono(
            -h_width..h_width,
            -h_height..h_height,
            -h_depth..h_depth,
            NoMat,
        );
        let rot_range = (-180.0)..180.0;
        for _ in 0..50 {
            let angle = next_std_in_range(&rot_range);
            test_pdf_integration(aabox.clone().rotate_y(angle), count);
        }
    }

    #[test]
    fn test_rotation_90() {
        let aabox = AABox::mono(
            -1.0..1.0, -1.0..1.0, -1.0..1.0,
            NoMat,
        );
        let rotated = aabox.clone().rotate_y(90.0);

        let origin = P3::new(0.0, 0.0, 5.0);
        let rot_origin = P3::new(5.0, 0.0, 0.0);
        let direction = Unit::new_unchecked(V3::new(0.0, 0.0, -1.0));
        let rot_direction = Unit::new_unchecked(V3::new(-1.0, 0.0, 0.0));
        let ray = RayCtx::new(origin, direction, Color::zeros(), 1.0, 2);
        let rotated_ray = RayCtx::new(rot_origin, rot_direction, Color::zeros(), 1.0, 2);

        let aabox_hit = aabox.hit(&ray, -100.0, 100.0).unwrap();
        let rotated_hit = rotated.hit(&ray, -100.0, 100.0).unwrap();

        let epsilon = 0.000_000_1;
        assert_eq_eps!(aabox_hit.point, rotated_hit.point, epsilon);
        assert_eq_eps!(aabox_hit.normal.into_inner(), rotated_hit.normal.into_inner(), epsilon);
        assert_eq!(aabox_hit.dist, rotated_hit.dist);
    }


}
