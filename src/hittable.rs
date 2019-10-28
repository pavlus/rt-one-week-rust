use crate::ray::Ray;
use crate::vec::V3;
use crate::material::Material;
use crate::aabb::AABB;
use std::fmt::Debug;
use std::f64::consts;
use std::ops::Range;

#[derive(Copy, Clone)]
pub struct Hit<'a> {
    pub point: V3,
    pub normal: V3,
    pub u: f64,
    pub v: f64,
    pub material: &'a Box<dyn Material>,
    pub dist: f64,
}

//impl Eq for Hit {}

impl<'a> Hit<'a> {
    pub fn new(dist: f64, p: V3, n: V3, material: &'a Box<dyn Material>, u: f64, v: f64) -> Hit<'a> {
        return Hit { dist, point: p, normal: n, material, u, v };
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
    fn material(&self) -> &Box<dyn Material> { &self.material }

    fn aabb(&self, t0: f32, t1: f32) -> AABB {
        AABB::new(self.center - self.radius, self.center + self.radius)
    }

    fn uv(unit_point: V3) -> (f64, f64) {
        let phi = f64::atan2(unit_point.z, unit_point.x);
        let theta = unit_point.y.asin();

        let u = 1.0 - (phi + consts::PI) / (2.0 * consts::PI);
        let v = (theta + consts::FRAC_PI_2) / consts::PI;
        (u, v)
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
        let oc = ray.origin - self.center(ray.time);
        let a = ray.direction.sqr_length();
        let b = oc.dot(ray.direction);
        let c = oc.sqr_length() - (self.radius() * self.radius()) as f64;
        let discr_sqr = b * b - a * c;

        let get_hit = |ray: &Ray, dist: f64| -> Hit {
            let p = ray.point_at(dist);
            let n = (p - self.center(ray.time)) / self.radius();
            let (u, v) = Sphere::uv(n);
            return Hit::new(dist, p, n, &self.material(), u, v);
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

        let oc = ray.origin - self.center(ray.time);
        let a = ray.direction.sqr_length();
        let b = oc.dot(ray.direction);
        let c = oc.sqr_length() - (self.radius * self.radius) as f64;
        let discr_sqr = b * b - a * c;

        let get_hit = |ray: &Ray, dist: f64| -> Hit {
            let p = ray.point_at(dist);
            let n = (p - self.center(ray.time)) / self.radius;
            let (u, v) = Sphere::uv(n);
            return Hit::new(dist, p, n, &self.material, u, v);
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

macro_rules! aarect_aabb {
    {$slf:ident, $a:tt, $b:tt, $off:expr} => {
        AABB::new(
            aarect_aabb!($slf, start, $a, $b, $off - 0.001),
            aarect_aabb!($slf, end  , $a, $b, $off + 0.001)
        )
    };
    {$slf:ident, $bound:ident, x, y, $off:expr} => {V3::new($slf.x.$bound, $slf.y.$bound, $off)};
    {$slf:ident, $bound:ident, x, z, $off:expr} => {V3::new($slf.x.$bound, $off, $slf.z.$bound)};
    {$slf:ident, $bound:ident, y, z, $off:expr} => {V3::new($off, $slf.y.$bound, $slf.z.$bound)};
}

macro_rules! norm_vec {
    {x, y} => {V3::new(0.0,0.0,1.0)};
    {x, z} => {V3::new(0.0,1.0,0.0)};
    {y, z} => {V3::new(1.0,0.0,0.0)};
}
macro_rules! aarect {
    {$name:tt, $a:tt, $b:tt, normal: $k:tt} =>{
        #[derive(Debug)]
        pub struct $name {
            $a: Range<f64>,
            $b: Range<f64>,
            k: f64,
            material: Box<dyn Material>
        }
        impl $name {
            pub fn new($a: Range<f64>, $b:Range<f64>, k:f64, material: Box<dyn Material>) -> $name {
                $name { $a, $b, k, material }
            }

            fn uv(&self, $a:f64, $b: f64) -> (f64, f64) {
                let u = ($a - self.$a.start)/(self.$a.end-self.$a.start);
                let v = ($b - self.$b.start)/(self.$b.end-self.$b.start);
                (u, v)
            }
        }

        impl Hittable for $name {
            fn hit(&self, ray: &Ray, dist_min: f64, dist_max: f64) -> Option<Hit> {
                let dist = (self.k - ray.origin.$k) / ray.direction.$k;
                if !(dist_min..dist_max).contains(&dist) { return None; };

                let $a = ray.origin.$a + dist * ray.direction.$a;
                let $b = ray.origin.$b + dist * ray.direction.$b;

                if !(self.$a.contains(&$a) && self.$b.contains(&$b)) {
                    return None;
                };

                let (u, v) = self.uv($a, $b);
                Some(Hit::new(dist, ray.point_at(dist), norm_vec!($a, $b), &self.material, u, v))
            }

            fn bounding_box(&self, t_min: f32, t_max: f32) -> Option<AABB> {
                Some(aarect_aabb!(self, $a, $b, self.k))
            }
        }

    };
}

aarect!(XYRect, x, y, normal: z);
aarect!(XZRect, x, z, normal: y);
aarect!(YZRect, y, z, normal: x);


#[derive(Debug)]
pub struct Stage {
    objects: Vec<Box<dyn Hittable>>,
    aabb: AABB,
}

impl Stage {
    pub fn new(objects: Vec<Box<dyn Hittable>>) -> Stage {
        let aabb = (|| {
            let mut aabbs = objects.iter().flat_map(|o| o.bounding_box(0.0, 1.0));
            let first = aabbs.next()?;
            Some(aabbs.fold(first, |a, b| a + b))
        })();
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
            .filter_map(std::convert::identity)
            .min_by(|s, o| s.dist.partial_cmp(&o.dist).unwrap())
    }

    fn bounding_box(&self, t_min: f32, t_max: f32) -> Option<AABB> {
        Some(self.aabb)
    }
}

#[derive(Debug)]
pub struct FlipNormals {
    hittable: Box<dyn Hittable>,
}

impl FlipNormals {
    pub fn new(hittable: Box<dyn Hittable>) -> Box<Self> {
        Box::new(FlipNormals { hittable })
    }
}


impl Hittable for FlipNormals {
    fn hit(&self, ray: &Ray, dist_min: f64, dist_max: f64) -> Option<Hit> {
        match self.hittable.hit(ray, dist_min, dist_max) {
            Some(h) => Some(Hit {
                normal: -h.normal,
                ..h
            }),
            None => None
        }
    }

    fn bounding_box(&self, t_min: f32, t_max: f32) -> Option<AABB> {
        self.hittable.bounding_box(t_min, t_max)
    }
}