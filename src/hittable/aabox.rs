use std::ops::Range;
use std::sync::Arc;

use super::{AABB, Hit, Hittable, Material, Ray, V3};
use std::borrow::Borrow;

#[derive(Debug, Clone)]
pub struct AABoxMono {
    x: Range<f64>,
    y: Range<f64>,
    z: Range<f64>,
    aabb: AABB,
    material: Arc<dyn Material>,
}

#[derive(Debug, Clone)]
pub struct AABoxHetero {
    x: Range<f64>,
    y: Range<f64>,
    z: Range<f64>,
    aabb: AABB,
    front: Arc<dyn Material>,
    back: Arc<dyn Material>,
    top: Arc<dyn Material>,
    bottom: Arc<dyn Material>,
    left: Arc<dyn Material>,
    right: Arc<dyn Material>,
}


impl AABoxMono {
    pub fn new(
        x: Range<f64>,
        y: Range<f64>,
        z: Range<f64>,
        material: Arc<dyn Material>,
    ) -> AABoxMono {
        let Range { start: x_start, end: x_end } = x;
        let Range { start: y_start, end: y_end } = y;
        let Range { start: z_start, end: z_end } = z;
        AABoxMono {
            x,
            y,
            z,
            aabb: AABB::new(V3::new(x_start, y_start, z_start),
                            V3::new(x_end, y_end, z_end)),
            material,
        }
    }
}

impl AABoxHetero {
    pub fn new(
        x: Range<f64>,
        y: Range<f64>,
        z: Range<f64>,
        front: Arc<dyn Material>,
        back: Arc<dyn Material>,
        top: Arc<dyn Material>,
        bottom: Arc<dyn Material>,
        left: Arc<dyn Material>,
        right: Arc<dyn Material>,
    ) -> AABoxHetero {
        let Range { start: x_start, end: x_end } = x;
        let Range { start: y_start, end: y_end } = y;
        let Range { start: z_start, end: z_end } = z;
        AABoxHetero {
            x,
            y,
            z,
            front,
            back,
            top,
            bottom,
            left,
            right,
            aabb: AABB::new(V3::new(x_start, y_start, z_start),
                            V3::new(x_end, y_end, z_end)),
        }
    }
}

impl Hittable for AABoxMono {
    fn hit(&self, ray: &Ray, dist_min: f64, dist_max: f64) -> Option<Hit> {

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
        let mut dist: f64 = dist_max;
        if self.x.contains(&x_front) && self.y.contains(&y_front) && dist_min < dist_front && dist_front < dist {
            result = Some(Hit::new(dist_front, ray.point_at(dist_front), V3::new(0., 0., 1.), self.material.borrow(), u_front, v_front));
            dist = dist_front;
        };
        if self.x.contains(&x_back) && self.y.contains(&y_back) && dist_min < dist_back && dist_back < dist {
            result = Some(Hit::new(dist_back, ray.point_at(dist_back), V3::new(0., 0., -1.), self.material.borrow(), u_back, v_back));
            dist = dist_back;
        }
        if self.x.contains(&x_top) && self.z.contains(&z_top) && dist_min < dist_top && dist_top < dist {
            result = Some(Hit::new(dist_top, ray.point_at(dist_top), V3::new(0., 1., 0.), self.material.borrow(), u_top, v_top));
            dist = dist_top;
        }
        if self.x.contains(&x_bottom) && self.z.contains(&z_bottom) && dist_min < dist_bottom && dist_bottom < dist {
            result = Some(Hit::new(dist_bottom, ray.point_at(dist_bottom), V3::new(0., -1., 0.), self.material.borrow(), u_bottom, v_bottom));
            dist = dist_bottom;
        }
        if self.y.contains(&y_left) && self.z.contains(&z_left) && dist_min < dist_left && dist_left < dist {
            result = Some(Hit::new(dist_left, ray.point_at(dist_left), V3::new(1., 0., 0.), self.material.borrow(), u_left, v_left));
            dist = dist_left;
        }
        if self.y.contains(&y_right) && self.z.contains(&z_right) && dist_min < dist_right && dist_right < dist {
            result = Some(Hit::new(dist_right, ray.point_at(dist_right), V3::new(-1., 0., 0.), self.material.borrow(), u_right, v_right));
            dist = dist_right;
        }
        result
    }

    fn bounding_box(&self, _: f32, _: f32) -> Option<AABB> {
        Some(self.aabb)
    }
}


impl Hittable for AABoxHetero {
    fn hit(&self, ray: &Ray, dist_min: f64, dist_max: f64) -> Option<Hit> {

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
        let mut dist: f64 = dist_max;
        if self.x.contains(&x_front) && self.y.contains(&y_front) && dist_min < dist_front && dist_front < dist {
            result = Some(Hit::new(dist_front, ray.point_at(dist_front), V3::new(0., 0., 1.), self.front.borrow(), u_front, v_front));
            dist = dist_front;
        };
        if self.x.contains(&x_back) && self.y.contains(&y_back) && dist_min < dist_back && dist_back < dist {
            result = Some(Hit::new(dist_back, ray.point_at(dist_back), V3::new(0., 0., -1.), self.back.borrow(), u_back, v_back));
            dist = dist_back;
        }
        if self.x.contains(&x_top) && self.z.contains(&z_top) && dist_min < dist_top && dist_top < dist {
            result = Some(Hit::new(dist_top, ray.point_at(dist_top), V3::new(0., 1., 0.), self.top.borrow(), u_top, v_top));
            dist = dist_top;
        }
        if self.x.contains(&x_bottom) && self.z.contains(&z_bottom) && dist_min < dist_bottom && dist_bottom < dist {
            result = Some(Hit::new(dist_bottom, ray.point_at(dist_bottom), V3::new(0., -1., 0.), self.bottom.borrow(), u_bottom, v_bottom));
            dist = dist_bottom;
        }
        if self.y.contains(&y_left) && self.z.contains(&z_left) && dist_min < dist_left && dist_left < dist {
            result = Some(Hit::new(dist_left, ray.point_at(dist_left), V3::new(1., 0., 0.), self.left.borrow(), u_left, v_left));
            dist = dist_left;
        }
        if self.y.contains(&y_right) && self.z.contains(&z_right) && dist_min < dist_right && dist_right < dist {
            result = Some(Hit::new(dist_right, ray.point_at(dist_right), V3::new(-1., 0., 0.), self.right.borrow(), u_right, v_right));
            dist = dist_right;
        }
        result
    }

    fn bounding_box(&self, _: f32, _: f32) -> Option<AABB> {
        Some(self.aabb)
    }

    fn pdf_value(&self, _: &V3, direction: &V3, hit: &Hit) -> f64 {
        let width = self.x.end - self.x.start;
        let height = self.y.end - self.y.start;
        let depth = self.z.end - self.z.start;
        let area = width * height + height * depth + width * depth;
        let sqr_dist = (hit.dist * hit.dist) * direction.sqr_length();
        let cosine = f64::abs(direction.dot(hit.normal) / direction.length());
        sqr_dist / (cosine * area)
    }

    fn random(&self, origin: &V3) -> V3 {
        let o: [f64;3] = (*origin).into();
        let k = crate::random::next_std_i32() as usize;
        let a = k + 1;
        let b = k + 2;
        let (k, a, b) = (k % 3, a % 3, b % 3);
        let axes = [&self.x, &self.y, &self.z];
        let mut tmp: [f64; 3] = [0., 0., 0.];
        tmp[a] = crate::random::next_std_f64_in_range(axes[a]);
        tmp[b] = crate::random::next_std_f64_in_range(axes[b]);
        tmp[k] = f64::min(axes[k].start - o[k], axes[k].end - o[k]);
        tmp.into()
    }
}


pub struct AABox;

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
    ) -> AABoxHetero {
        AABoxHetero::new(x, y, z, front, back, top, bottom, left, right)
    }

    pub fn mono(
        x: Range<f64>,
        y: Range<f64>,
        z: Range<f64>,
        material: Arc<dyn Material>,
    ) -> AABoxMono {
        AABoxMono::new(x, y, z, Arc::clone(&material))
    }
}

