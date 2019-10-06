use crate::ray::Ray;
use crate::vec::V3;
use crate::material::Material;
use crate::aabb::AABB;
use std::fmt::Debug;

#[derive(Copy, Clone)]
pub struct Hit<'a> {
    dist: f64,
    p: V3,
    normal: V3,
    material: &'a Box<dyn Material>,
}

//impl Eq for Hit {}

impl<'a> Hit<'a> {
    pub fn new(dist: f64, p: V3, n: V3, material: &'a Box<dyn Material>) -> Hit<'a> {
        return Hit { dist, p, normal: n, material };
    }
    pub fn point(self) -> V3 {
        self.p
    }
    pub fn normal(self) -> V3 {
        self.normal
    }
    pub fn dist(self) -> f64 {
        self.dist
    }
    pub fn material(self) -> &'a Box<dyn Material> {
        &self.material
    }
}

pub trait Hittable: Debug {
    fn hit(&self, ray: &Ray, dist_min: f64, dist_max: f64) -> Option<Hit>;
    fn bounding_box(&self, t_min: f32, t_max: f32) -> Option<AABB> { None }
}


#[derive(Debug)]
pub struct Sphere {
    center: V3,
    radius: f64,
    material: Box<dyn Material>,
}

impl Sphere {
    pub fn new(center: V3, radius: f64, material: Box<dyn Material>) -> Sphere {
        Sphere { center, radius, material }
    }
    fn center(&self, time: f32) -> V3 {
        self.center
    }
    fn radius(&self) -> f64 { self.radius }
    fn material<'a>(&'a self) -> &'a Box<dyn Material> { &self.material }

    fn aabb(&self, t0: f32, t1: f32) -> AABB {
        AABB::new(self.center - self.radius, self.center + self.radius)
    }
}

#[derive(Debug)]
pub struct MovingSphere {
    center_t0: V3,
    center_t1: V3,
    time0: f32,
    duration: f32,
    radius: f64,
    material: Box<dyn Material>,
}

impl MovingSphere {
    pub fn new(center_t0: V3, center_t1: V3, time0: f32, time1: f32, radius: f64, material: Box<dyn Material>) -> MovingSphere {
        MovingSphere {
            center_t0,
            center_t1,
            time0,
            duration: time1 - time0,
            radius,
            material,
        }
    }
    fn center(&self, time: f32) -> V3 {
        let scale = (time - self.time0) / self.duration;
        self.center_t0 + scale * (self.center_t1 - self.center_t0)
    }
    fn radius(&self) -> f64 { self.radius }
    fn material<'a>(&'a self) -> &'a Box<dyn Material> { &self.material }
    fn aabb(&self, t: f32) -> AABB {
        AABB::new(self.center(t) - self.radius(), self.center(t) + self.radius())
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, dist_min: f64, dist_max: f64) -> Option<Hit> {
        let oc = ray.origin() - self.center(ray.time());
        let a = ray.direction().sqr_length();
        let b = oc.dot(ray.direction());
        let c = oc.sqr_length() - (self.radius() * self.radius()) as f64;
        let discr_sqr = b * b - a * c;

        let get_hit = |ray: &Ray, dist: f64| -> Hit {
            let p = ray.point_at(dist);
            let n = (p - self.center(ray.time())) / self.radius();
            return Hit::new(dist, p, n, &self.material());
        };

        if discr_sqr > 0.0 {
            let tmp = (b * b - a * c).sqrt();
            let x1 = (-b - tmp) / a;
            if (dist_min..dist_max).contains(&x1) {
                return Option::Some(get_hit(ray, x1));
            }
            let x2 = (-b + tmp) / a;
            if (dist_min..dist_max).contains(&x2) {
                return Option::Some(get_hit(ray, x2));
            }
            return None;
        } else {
            None
        }
    }

    fn bounding_box(&self, t_min: f32, t_max: f32) -> Option<AABB> {
        Some(self.aabb(t_min, t_max))
    }
}

impl Hittable for MovingSphere {
    fn hit(&self, ray: &Ray, dist_min: f64, dist_max: f64) -> Option<Hit> {
//        if !self.bounding_box(ray.time(), ray.time())
//            .unwrap().hit(ray, dist_min, dist_max) { return None; }

        let oc = ray.origin() - self.center(ray.time());
        let a = ray.direction().sqr_length();
        let b = oc.dot(ray.direction());
        let c = oc.sqr_length() - (self.radius() * self.radius()) as f64;
        let discr_sqr = b * b - a * c;

        let get_hit = |ray: &Ray, dist: f64| -> Hit {
            let p = ray.point_at(dist);
            let n = (p - self.center(ray.time())) / self.radius();
            return Hit::new(dist, p, n, &self.material());
        };

        if discr_sqr > 0.0 {
            let tmp = (b * b - a * c).sqrt();
            let x1 = (-b - tmp) / a;
            if (dist_min..dist_max).contains(&x1) {
                return Option::Some(get_hit(ray, x1));
            }
            let x2 = (-b + tmp) / a;
            if (dist_min..dist_max).contains(&x2) {
                return Option::Some(get_hit(ray, x2));
            }
            return None;
        } else {
            None
        }
    }

    fn bounding_box(&self, t_min: f32, t_max: f32) -> Option<AABB> {
        Some(self.aabb(t_min) + self.aabb(t_max))
    }
}

#[derive(Debug)]
pub struct Stage {
    objects: Vec<Box<dyn Hittable>>,
    aabb: AABB,
}

impl Stage {
    pub fn new(objects: Vec<Box<dyn Hittable>>) -> Stage {
        let aabb = match objects.len() {
            0 => None,
            1 => objects.first().and_then(|h| h.bounding_box(0.0, 1.0)),
            _ => {
                let mut aabbs = objects.iter().map(|o| o.bounding_box(0.0, 1.0));
                let first = aabbs.next().unwrap();
                aabbs.fold(first, |a, b| a.and_then(|a| b.map(|b| a + b)).or(b))
            }
        };

        Stage { objects, aabb: aabb.unwrap() }
    }
}

impl Hittable for Stage {
    fn hit(&self, ray: &Ray, dist_min: f64, dist_max: f64) -> Option<Hit> {
        self.objects
            .iter()
            // todo[performance]: try enabling again after implementing heavier object
//            .filter(|h| h.bounding_box(ray.time(), ray.time())
//                .map(|aabb| aabb.hit(ray, dist_min, dist_max))
//                .unwrap_or(true)
//            )
            .map(|h| h.hit(ray, dist_min, dist_max))
            .filter(Option::is_some)
            .map(Option::unwrap)
            .min_by(|s, o| s.dist().partial_cmp(&o.dist()).unwrap())
    }

    fn bounding_box(&self, t_min: f32, t_max: f32) -> Option<AABB> {
        Some(self.aabb)
    }
}
