use std::ops::Range;
use std::sync::Arc;

use super::{AABB, Hit, Hittable, Material, RayCtx, P3, V3};
use std::borrow::Borrow;
use crate::random::next_std_in_range;
use crate::types::Distance;
use nalgebra::Unit;

#[derive(Clone)]
pub struct AABox {
    x: Range<Distance>,
    y: Range<Distance>,
    z: Range<Distance>,
    aabb: AABB,
    front: Arc<dyn Material>,
    back: Arc<dyn Material>,
    top: Arc<dyn Material>,
    bottom: Arc<dyn Material>,
    left: Arc<dyn Material>,
    right: Arc<dyn Material>,
}

impl AABox {
    pub fn new(
        x: Range<Distance>,
        y: Range<Distance>,
        z: Range<Distance>,
        front: Arc<dyn Material>,
        back: Arc<dyn Material>,
        top: Arc<dyn Material>,
        bottom: Arc<dyn Material>,
        left: Arc<dyn Material>,
        right: Arc<dyn Material>,
    ) -> AABox {
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

    pub fn mono(
        x: Range<Distance>,
        y: Range<Distance>,
        z: Range<Distance>,
        material: Arc<dyn Material>,
    ) -> AABox {
        let Range { start: x_start, end: x_end } = x;
        let Range { start: y_start, end: y_end } = y;
        let Range { start: z_start, end: z_end } = z;
        AABox {
            x,
            y,
            z,
            front: Arc::clone(&material),
            back: Arc::clone(&material),
            top: Arc::clone(&material),
            bottom: Arc::clone(&material),
            left: Arc::clone(&material),
            right: material,
            aabb: AABB::new(P3::new(x_start, y_start, z_start),
                            P3::new(x_end, y_end, z_end)),
        }
    }
}


impl Hittable for AABox {
    fn hit(&self, ray_ctx: &RayCtx, dist_min: Distance, dist_max: Distance) -> Option<Hit> {
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
        let mut dist: Distance = dist_max;
        if self.x.contains(&x_front) && self.y.contains(&y_front) && dist_min < dist_front && dist_front < dist {
            result = Some(Hit::new(dist_front, ray.point_at(dist_front), Unit::new_unchecked(V3::new(0., 0., 1.)), self.front.borrow(), u_front, v_front));
            dist = dist_front;
        };
        if self.x.contains(&x_back) && self.y.contains(&y_back) && dist_min < dist_back && dist_back < dist {
            result = Some(Hit::new(dist_back, ray.point_at(dist_back), Unit::new_unchecked(V3::new(0., 0., -1.)), self.back.borrow(), u_back, v_back));
            dist = dist_back;
        }
        if self.x.contains(&x_top) && self.z.contains(&z_top) && dist_min < dist_top && dist_top < dist {
            result = Some(Hit::new(dist_top, ray.point_at(dist_top), Unit::new_unchecked(V3::new(0., 1., 0.)), self.top.borrow(), u_top, v_top));
            dist = dist_top;
        }
        if self.x.contains(&x_bottom) && self.z.contains(&z_bottom) && dist_min < dist_bottom && dist_bottom < dist {
            result = Some(Hit::new(dist_bottom, ray.point_at(dist_bottom), Unit::new_unchecked(V3::new(0., -1., 0.)), self.bottom.borrow(), u_bottom, v_bottom));
            dist = dist_bottom;
        }
        if self.y.contains(&y_left) && self.z.contains(&z_left) && dist_min < dist_left && dist_left < dist {
            result = Some(Hit::new(dist_left, ray.point_at(dist_left), Unit::new_unchecked(V3::new(1., 0., 0.)), self.left.borrow(), u_left, v_left));
            dist = dist_left;
        }
        if self.y.contains(&y_right) && self.z.contains(&z_right) && dist_min < dist_right && dist_right < dist {
            result = Some(Hit::new(dist_right, ray.point_at(dist_right), Unit::new_unchecked(V3::new(-1., 0., 0.)), self.right.borrow(), u_right, v_right));
            // dist = dist_right;
        }
        result
    }

    fn bounding_box(&self, _: f32, _: f32) -> Option<AABB> {
        Some(self.aabb)
    }

    fn pdf_value(&self, _: &P3, direction: &Unit<V3>, hit: &Hit) -> f64 {
        let width = self.x.end - self.x.start;
        let height = self.y.end - self.y.start;
        let depth = self.z.end - self.z.start;

        let dir_unit = direction;
        let area_xy = Distance::abs(width * height * dir_unit.z);
        let area_xz = Distance::abs(depth * width * dir_unit.y);
        let area_yz = Distance::abs(height * depth * dir_unit.x);
        let sqr_dist = hit.dist * hit.dist;
        let total_area = area_xy + area_yz + area_xz;
        sqr_dist as f64 / total_area as f64
    }

    fn random(&self, origin: &P3) -> Unit<V3> {
        let x = next_std_in_range(&self.x);
        let y = next_std_in_range(&self.y);
        let z = next_std_in_range(&self.z);
        Unit::new_normalize(V3::new(x, y, z) - &origin.coords)
    }
}


#[cfg(test)]
mod test{
    use crate::hittable::{AABox, XYRect, Hit, Hittable};
    use std::sync::Arc;
    use crate::material::Lambertian;
    use crate::V3;
    use nalgebra::Unit;
    use crate::random::{rand_in_unit_sphere, next_std};
    use crate::hittable::test::test_pdf_integration;
    use crate::types::{Color, Distance, P3};

    #[test]
    fn test_box_rect_pdf(){
        let mat = Arc::new(Lambertian::color(V3::new(1.0, 1.0, 1.0)));
        let aabox = AABox::mono(-1.0..1.0, -1.0..1.0, -1.0..1.0, mat.clone());
        let aarect = XYRect::new(-1.0..1.0, -1.0..1.0, -1.0, mat.clone());
        let origin = P3::new(0.0, 0.0, -2.0);
        let direction = Unit::new_unchecked(V3::new(0.0, 0.0, 1.0));
        let hit = Hit::new(1.0,
                           P3::new(0.0, 0.0, -1.0),
                           Unit::new_unchecked(V3::new(0.0, 0.0, -1.0)), &*mat, 0.0, 0.0);
        assert_eq!(
            aabox.pdf_value(&origin, &direction, &hit),
            aarect.pdf_value(&origin, &direction, &hit)
        );
    }

    #[test]
    fn test_pdf() {
        for _ in 0..100 {
            let count = 10_000;

            let center: P3 = 5.0 * rand_in_unit_sphere();
            let h_width:Distance = 1.0 + next_std();
            let h_height:Distance = 1.0 + next_std();
            let h_depth:Distance = 1.0 + next_std();

            let aabox = AABox::mono(
                (center.x - h_width)..(center.x + h_width),
                (center.y - h_height)..(center.y + h_height),
                (center.z - h_depth)..(center.z + h_depth),
                Arc::new(Lambertian::new(Color::from_element(1.0))),
            );
            test_pdf_integration(aabox, count);
        }
    }
}

