use std::ops::Range;
use std::sync::Arc;

use super::{AABB, Hit, Hittable, HittableList, Instance, Material, Ray, V3, XYRect, XZRect, YZRect};

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
