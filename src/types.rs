use std::ops::Range;

use nalgebra::{Point2, Point3, Unit, Vector3};

// pub type Geometry = f64;
// pub type Scale = f64;
// pub type Probability = f64;
// pub type V3 = Vector3<Geometry>;
// pub type Direction = Unit<V3>;
// pub type P3 = Point3<Geometry>;
// pub type P2 = Point2<Geometry>;
//
// pub type Time = f32;
// pub type Timespan = Range<Time>;
//
// pub type ColorComponent = f64;
// pub type Color = Vector3<ColorComponent>;

pub type Geometry = f64;
pub type Scale = f64;
pub type Probability = f32;
pub type V3 = Vector3<Geometry>;
pub type Direction = Unit<V3>;
pub type P3 = Point3<Geometry>;
pub type P2 = Point2<Geometry>;

pub type Time = f32;
pub type Timespan = Range<Time>;

pub type ColorComponent = f32;
pub type Color = Vector3<ColorComponent>;
