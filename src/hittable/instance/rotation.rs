use core::option::Option;
use crate::aabb::AABB;
use crate::hittable::{Hit, Hittable};
use crate::ray::{Ray, RayCtx};
use crate::types::{Direction, Geometry, P3, Probability, Timespan, V3};
use super::*;

pub trait RotateOp<I, O> {
    fn rotate_x(self, angle_degrees: Geometry) -> O;
    fn rotate_y(self, angle_degrees: Geometry) -> O;
    fn rotate_z(self, angle_degrees: Geometry) -> O;
    fn apply_rotation(self, rotation: Rotation3<Geometry>) -> O;
}

#[derive(Debug,Clone)]
pub struct Rotate<T> {
    pub(super) target: T,
    pub(super) transform: Rotation3<Geometry>,
    pub(super) aabb: Option<AABB>,
}

impl<T> Rotate<T> {
    pub fn into_inner(self) -> T {
        self.target
    }
}


impl<I: Hittable> RotateOp<I, Rotate<I>> for I {
    fn rotate_x(self, angle_degrees: Geometry) -> Rotate<I> {
        RotateOp::apply_rotation(self, Rotation3::from_scaled_axis(V3::x() * Geometry::to_radians(angle_degrees)))
    }

    fn rotate_y(self, angle_degrees: Geometry) -> Rotate<I> {
        RotateOp::apply_rotation(self, Rotation3::from_scaled_axis(V3::y() * Geometry::to_radians(angle_degrees)))
    }

    fn rotate_z(self, angle_degrees: Geometry) -> Rotate<I> {
        RotateOp::apply_rotation(self, Rotation3::from_scaled_axis(V3::z() * Geometry::to_radians(angle_degrees)))
    }

    fn apply_rotation(self, rotation: Rotation3<Geometry>) -> Rotate<I> {
        let aabb = self.bounding_box(0.0..1.0)
            .map(|aabb| aabb.by_rotation(&rotation));
        Rotate { target: self, transform: rotation, aabb }
    }
}

impl<T: Hittable + Orientable> RotateOp<T, T> for T {
    fn rotate_x(self, angle_degrees: Geometry) -> T {
        let angle = Geometry::to_radians(angle_degrees);
        self.by_scaled_axis(V3::x() * angle)
    }

    fn rotate_y(self, angle_degrees: Geometry) -> T {
        let angle = Geometry::to_radians(angle_degrees);
        self.by_scaled_axis(V3::y() * angle)
    }

    fn rotate_z(self, angle_degrees: Geometry) -> T {
        let angle = Geometry::to_radians(angle_degrees);
        self.by_scaled_axis(V3::z() * angle)
    }

    fn apply_rotation(self, rotation: Rotation3<Geometry>) -> T {
        self.by_rotation(&rotation)
    }
}

impl<T: Hittable> Hittable for Rotate<T> {
    fn hit(&self, ray_ctx: &RayCtx, dist_min: Geometry, dist_max: Geometry) -> Option<Hit> {
        let ray = ray_ctx.ray;
        let origin = self.transform.inverse_transform_point(&ray.origin);
        let direction =  self.transform.inverse_transform_unit_vector(&ray.direction);
        let rotated_ray = RayCtx { ray: Ray { origin, direction }, ..*ray_ctx };

        self.target.hit(&rotated_ray, dist_min, dist_max)
            .map(|hit| {
                let point =  self.transform * hit.point;
                let normal = self.transform * &hit.normal;

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

    fn pdf_value(&self, origin: &P3, direction: &Direction, hit: &Hit) -> Probability {
        let origin = self.transform.inverse_transform_point(origin);
        let direction = self.transform.inverse_transform_unit_vector(direction);
        let hit = Hit{
            point: origin,
                ..*hit
        };
        self.target.pdf_value(&origin, &direction, &hit)
    }

    fn random(&self, origin: &P3) -> Direction {
            self.transform *
                &self.target.random(
                    &self.transform.inverse_transform_point(&origin))
    }
}

#[cfg(test)]
mod test {
    use nalgebra::Unit;
    use crate::hittable::{AABox, Hittable, RotateOp, Sphere};
    use crate::types::{Color, Geometry, P3};
    use crate::material::NoMat;
    use crate::hittable::test::test_pdf_integration;
    use crate::random::{next_std, next_std_in_range, rand_in_unit_sphere};
    use crate::ray::RayCtx;
    use crate::types::V3;

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
