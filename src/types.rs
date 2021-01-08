use std::ops::Range;

use nalgebra::{Point2, Point3, Vector2, Vector3};

pub type Distance = f64;
pub type Angle = f64;
pub type Scale = f64;
pub type V3 = Vector3<Distance>;
pub type V2 = Vector2<Distance>;
pub type P3 = Point3<Distance>;
pub type P2 = Point2<Distance>;

pub type Time = f32;
pub type Timespan = Range<Time>;

pub type ColorComponent = f64;
pub type Color = Vector3<ColorComponent>;
