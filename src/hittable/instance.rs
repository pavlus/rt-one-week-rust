use std::f64::consts::PI;

use itertools::Itertools;

use super::{AABB, Hit, Hittable, Ray, V3};

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
    fn hit(&self, ray: &Ray, dist_min: f64, dist_max: f64) -> Option<Hit> {
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

    fn pdf_value(&self, origin: &V3, direction: &V3, hit: &Hit) -> f64 {
        // self.target.pdf_value(&(*origin - self.offset), direction, hit)
        self.target.pdf_value(origin, direction, hit)
    }

    fn random(&self, origin: &V3) -> V3 {
        // self.target.random(&(*origin - self.offset))
        self.target.random(origin)
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
    fn hit(&self, ray: &Ray, dist_min: f64, dist_max: f64) -> Option<Hit> {
        let origin = self.forward_transform(ray.origin);
        let direction = self.forward_transform(ray.direction);

        let rotated_ray = Ray { origin, direction, ..*ray };
        let sin = self.sin;
        let cos = self.cos;
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

    fn bounding_box(&self, _: f32, _: f32) -> Option<AABB> {
        self.aabb
    }

    fn pdf_value(&self, origin: &V3, direction: &V3, hit: &Hit) -> f64 {
        let origin = self.forward_transform(*origin);
        let direction = self.forward_transform(*direction);
        self.target.pdf_value(&origin, &direction, hit)
    }

    fn random(&self, origin: &V3) -> V3 {
        // self.target.random(&self.forward_transform(*origin))
        self.target.random(origin)
    }
}

impl<I> RotateY<I> {
    #[inline]
    fn forward_transform(&self, vec: V3) -> V3 {
        mul_by_matrix!(vec,
            self.cos, 0.0, -self.sin,
            0.0, 1.0, 0.0,
            self.sin, 0.0, self.cos
        )
    }
}
