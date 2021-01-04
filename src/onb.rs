use crate::V3;
use nalgebra::Unit;

#[derive(Debug, Copy, Clone)]
pub struct ONB {
    pub u: Unit<V3>,
    pub v: Unit<V3>,
    pub w: Unit<V3>,
}

impl ONB{
    pub fn from_w(w: &V3) -> ONB {
        let w= Unit::new_normalize(*w);
        let a = if w.x.abs() > 0.9 { V3::new(0., 1., 0.) } else { V3::new(1., 0., 0.) };
        let v = Unit::new_normalize(w.cross(&a));
        let u = Unit::new_normalize(w.cross(&v));
        ONB { u, v, w }
    }

    pub fn local(&self, a: &V3) -> V3 {
        a.x * self.u.as_ref() + a.y * self.v.as_ref() + a.z * self.w.as_ref()
    }
}
