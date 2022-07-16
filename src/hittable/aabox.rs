use std::ops::Range;

use super::{AABB, Hit, Hittable, Material, RayCtx};
use crate::random::next_std_in_range;
use crate::types::{Direction, Geometry, P2, P3, Probability, Timespan, V3};
use nalgebra::Unit;
use crate::hittable::Positionable;

#[derive(Clone, Debug)]
pub struct AABox<
    Front: Material,
    Back: Material,
    Top: Material,
    Bottom: Material,
    Left: Material,
    Right: Material> {
    x: Range<Geometry>,
    y: Range<Geometry>,
    z: Range<Geometry>,
    aabb: AABB,
    front: Front,
    back: Back,
    top: Top,
    bottom: Bottom,
    left: Left,
    right: Right,
}

pub type AABoxMono<M: Material> = AABox<M, M, M, M, M, M>;

impl<'a, M: Material + Clone> AABoxMono<M>
{
    pub fn mono(
        x: Range<Geometry>,
        y: Range<Geometry>,
        z: Range<Geometry>,
        material: M,
    ) -> Self {
        let Range { start: x_start, end: x_end } = x;
        let Range { start: y_start, end: y_end } = y;
        let Range { start: z_start, end: z_end } = z;

        AABox {
            x,
            y,
            z,
            front: material.clone(),
            back: material.clone(),
            top: material.clone(),
            bottom: material.clone(),
            left: material.clone(),
            right: material,
            aabb: AABB::new(P3::new(x_start, y_start, z_start),
                            P3::new(x_end, y_end, z_end)),
        }
    }
}

impl<Front: Material, Back: Material, Top: Material, Bottom: Material, Left: Material, Right: Material>
AABox<Front, Back, Top, Bottom, Left, Right> {
    pub fn new(
        x: Range<Geometry>,
        y: Range<Geometry>,
        z: Range<Geometry>,
        front: Front,
        back: Back,
        top: Top,
        bottom: Bottom,
        left: Left,
        right: Right,
    ) -> Self {
        let Range { start: x_start, end: x_end } = x;
        let Range { start: y_start, end: y_end } = y;
        let Range { start: z_start, end: z_end } = z;
        AABox {
            x,
            y,
            z,
            front,
            back,
            top,
            bottom,
            left,
            right,
            aabb: AABB::new(P3::new(x_start, y_start, z_start),
                            P3::new(x_end, y_end, z_end)),
        }
    }
}


impl<Front: Material, Back: Material, Top: Material, Bottom: Material, Left: Material, Right: Material>
Hittable for AABox<Front, Back, Top, Bottom, Left, Right> {
    fn hit(&self, ray_ctx: &RayCtx, dist_min: Geometry, dist_max: Geometry) -> Option<Hit> {
        let ray = &ray_ctx.ray;
        let dist_front =  (self.z.end - ray.origin.z) / ray.direction.z;
        let dist_back =   (self.z.start - ray.origin.z) / ray.direction.z;
        let dist_top =    (self.y.end - ray.origin.y) / ray.direction.y;
        let dist_bottom = (self.y.start - ray.origin.y) / ray.direction.y;
        let dist_left =   (self.x.end - ray.origin.x) / ray.direction.x;
        let dist_right =  (self.x.start - ray.origin.x) / ray.direction.x;

        let x_front =  ray.origin.x + dist_front  * ray.direction.x;
        let x_back =   ray.origin.x + dist_back   * ray.direction.x;
        let x_top =    ray.origin.x + dist_top    * ray.direction.x;
        let x_bottom = ray.origin.x + dist_bottom * ray.direction.x;

        let y_front =  ray.origin.y + dist_front  * ray.direction.y;
        let y_back =   ray.origin.y + dist_back   * ray.direction.y;
        let y_left =   ray.origin.y + dist_left   * ray.direction.y;
        let y_right =  ray.origin.y + dist_right  * ray.direction.y;

        let z_top =    ray.origin.z + dist_top    * ray.direction.z;
        let z_bottom = ray.origin.z + dist_bottom * ray.direction.z;
        let z_left =   ray.origin.z + dist_left   * ray.direction.z;
        let z_right =  ray.origin.z + dist_right  * ray.direction.z;


        let u_front =  (x_front  - self.x.start)/(self.x.end-self.x.start);
        let u_back =   (x_back   - self.x.start)/(self.x.end-self.x.start);
        let u_top =    (x_top    - self.x.start)/(self.x.end-self.x.start);
        let u_bottom = (x_bottom - self.x.start)/(self.x.end-self.x.start);

        let v_front =  (y_front  - self.y.start)/(self.y.end-self.y.start);
        let v_back =   (y_back   - self.y.start)/(self.y.end-self.y.start);
        let u_left =   (y_left   - self.y.start)/(self.y.end-self.y.start);
        let u_right =  (y_right  - self.y.start)/(self.y.end-self.y.start);

        let v_top =    (z_top    - self.z.start)/(self.z.end-self.z.start);
        let v_bottom = (z_bottom - self.z.start)/(self.z.end-self.z.start);
        let v_left =   (z_left   - self.z.start)/(self.z.end-self.z.start);
        let v_right =  (z_right  - self.z.start)/(self.z.end-self.z.start);

        let mut result: Option<Hit> = None;
        let mut dist: Geometry = dist_max;
        if self.x.contains(&x_front) && self.y.contains(&y_front) && dist_min <= dist_front && dist_front <= dist {
            result = Some(Hit::new(dist_front, ray.point_at(dist_front), Unit::new_unchecked(V3::new(0., 0., 1.)), &self.front, P2::new(u_front, v_front)));
            dist = dist_front;
        };
        if self.x.contains(&x_back) && self.y.contains(&y_back) && dist_min <= dist_back && dist_back <= dist {
            result = Some(Hit::new(dist_back, ray.point_at(dist_back), Unit::new_unchecked(V3::new(0., 0., -1.)), &self.back, P2::new(u_back, v_back)));
            dist = dist_back;
        }
        if self.x.contains(&x_top) && self.z.contains(&z_top) && dist_min <= dist_top && dist_top <= dist {
            result = Some(Hit::new(dist_top, ray.point_at(dist_top), Unit::new_unchecked(V3::new(0., 1., 0.)), &self.top, P2::new(u_top, v_top)));
            dist = dist_top;
        }
        if self.x.contains(&x_bottom) && self.z.contains(&z_bottom) && dist_min <= dist_bottom && dist_bottom <= dist {
            result = Some(Hit::new(dist_bottom, ray.point_at(dist_bottom), Unit::new_unchecked(V3::new(0., -1., 0.)), &self.bottom, P2::new(u_bottom, v_bottom)));
            dist = dist_bottom;
        }
        if self.y.contains(&y_left) && self.z.contains(&z_left) && dist_min <= dist_left && dist_left <= dist {
            result = Some(Hit::new(dist_left, ray.point_at(dist_left), Unit::new_unchecked(V3::new(1., 0., 0.)), &self.left, P2::new(u_left, v_left)));
            dist = dist_left;
        }
        if self.y.contains(&y_right) && self.z.contains(&z_right) && dist_min <= dist_right && dist_right <= dist {
            result = Some(Hit::new(dist_right, ray.point_at(dist_right), Unit::new_unchecked(V3::new(-1., 0., 0.)), &self.right, P2::new(u_right, v_right)));
            // dist = dist_right;
        }
        result
    }

    fn bounding_box(&self, _: Timespan) -> Option<AABB> {
        Some(self.aabb)
    }

    fn pdf_value(&self, origin: &P3, direction: &Direction, hit: &Hit) -> Probability {
        // it's horribly wrong :(
        let width = self.x.end - self.x.start;
        let height = self.y.end - self.y.start;
        let depth = self.z.end - self.z.start;

        let cosine = direction.abs();
        let area = V3::new(height * depth, depth * width, width * height);
        let total_area = area.dot(&cosine);

        let sqr_dist = hit.dist * hit.dist;

        if false && cfg!(test) {
            let cos_area = area.component_mul(&cosine);
            let total_area = cos_area.sum();
            eprintln!("&hit.normal: {:?}", &hit.normal);
            eprintln!("&hit.point: {:?}", &hit.point);
            eprintln!("direction: {:?}", direction);
            eprintln!("cosine: {:?}", cosine);
            eprintln!("area: {}", area);
            eprintln!("cos_area: {}", cos_area);
            eprintln!("sqr_dist: {}", sqr_dist);
            eprintln!("total_area: {}", total_area);
            eprintln!("----------------------------");
        }
        sqr_dist as Probability / total_area as Probability
    }

    fn random(&self, origin: &P3) -> Direction {
        let x = next_std_in_range(&self.x);
        let y = next_std_in_range(&self.y);
        let z = next_std_in_range(&self.z);
        Unit::new_normalize(V3::new(x, y, z) - &origin.coords)
    }
}

impl<M: Material + Clone> From<(M, AABB)> for AABoxMono<M> {
    fn from((material, aabb): (M, AABB)) -> Self {
        AABox::mono(aabb.min.x..aabb.max.x, aabb.min.y..aabb.max.y, aabb.min.z..aabb.max.z, material)
    }
}

impl<Front: Material, Back: Material, Top: Material, Bottom: Material, Left: Material, Right: Material>
Positionable for AABox<Front, Back, Top, Bottom, Left, Right> {
    fn move_by(&mut self, offset: &V3) {
        self.x = (self.x.start + offset.x)..(self.x.end + offset.x);
        self.y = (self.y.start + offset.y)..(self.y.end + offset.y);
        self.z = (self.z.start + offset.z)..(self.z.end + offset.z);
        self.aabb.move_by(offset);
    }

    fn moved_by(self, offset: &V3) -> Self {
        AABox {
            x: (self.x.start + offset.x)..(self.x.end + offset.x),
            y: (self.y.start + offset.y)..(self.y.end + offset.y),
            z: (self.z.start + offset.z)..(self.z.end + offset.z),
            aabb: self.aabb.moved_by(offset),
            ..self
        }
    }

}

#[cfg(test)]
mod test {
    use crate::hittable::{AABox, XYRect, Hit, Hittable, FlipNormalsOp};
    use crate::material::{Lambertian, NoMat};
    use crate::V3;
    use nalgebra::Unit;
    use crate::random::next_std;
    use crate::hittable::test::test_pdf_integration;
    use crate::ray::RayCtx;
    use crate::types::{Color, Geometry, P2, P3};

    #[test]
    fn test_box_rect_pdf_one_side_visible_eq_to_rect() {
        let mat = Lambertian::<Color>::new(Color::new(1.0, 1.0, 1.0));
        let aabox = AABox::mono(-1.0..1.0, -1.0..1.0, -1.0..1.0, mat.clone());
        let aarect = XYRect::new(-1.0..1.0, -1.0..1.0, -1.0, mat.clone());
        let origin = P3::new(0.0, 0.0, -2.0);
        let direction = Unit::new_unchecked(V3::new(0.0, 0.0, 1.0));
        let hit = Hit::new(1.0,
                           P3::new(0.0, 0.0, -1.0),
                           Unit::new_unchecked(V3::new(0.0, 0.0, -1.0)), &mat, P2::new(0.0, 0.0));
        assert_eq!(
            aabox.pdf_value(&origin, &direction, &hit),
            aarect.pdf_value(&origin, &direction, &hit)
        );
    }

    #[test]
    fn test_box_rect_pdf_smaller_than_rect_if_more_than_one_side_is_visible() {
        let mat = Lambertian::<Color>::new(Color::new(1.0, 1.0, 1.0));
        let aabox = AABox::mono(-1.0..1.0, -1.0..1.0, -1.0..1.0, mat.clone());
        let aarect = XYRect::new(-1.0..1.0, -1.0..1.0, -1.0, mat.clone())
            .flip_normals();
        let origin = P3::new(0.0, -1.5, -3.0);
        let direction = Unit::new_normalize(V3::new(0.0, 0.3, 0.9));
        let ray = RayCtx::new(origin, direction, Color::zeros(), 1.0, 2);
        let aabox_hit = aabox.hit(&ray, -10.0, 10.0).unwrap();
        let aarect_hit = aarect.hit(&ray, -10.0, 10.0).unwrap();
        assert_eq!(aabox_hit.point, aarect_hit.point);
        assert_eq!(aabox_hit.normal, aarect_hit.normal);
        assert_eq!(aabox_hit.dist, aarect_hit.dist);
        let boxpdf = aabox.pdf_value(&origin, &direction, &aabox_hit);
        let rectpdf = aarect.pdf_value(&origin, &direction, &aarect_hit);
        assert!(boxpdf < rectpdf, "{boxpdf} was not less than {rectpdf}");
    }

    #[test]
    fn test_pdf_converges() {
        let count = 1000_000;

        let h_width = 1.0 + next_std::<Geometry>();
        let h_height = 1.0 + next_std::<Geometry>();
        let h_depth = 1.0 + next_std::<Geometry>();

        let aabox = AABox::mono(
            -h_width..h_width,
            -h_height..h_height,
            -h_depth..h_depth,
            NoMat,
        );
        test_pdf_integration(aabox, count);
    }
}

