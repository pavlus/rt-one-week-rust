use crate::V3;

pub struct ONB{
    pub u: V3,
    pub v: V3,
    pub w: V3,
}

impl ONB{
    pub fn from_w(w: &V3) -> ONB {
        let w = w.unit();
        let a = if w.x.abs() > 0.9 { V3::new(0., 1., 0.) } else { V3::new(1., 0., 0.) };
        let v = w.cross(a).unit();
        let u = w.cross(v);
        ONB { u, v, w }
    }

    pub fn local(&self, a: V3) -> V3 {
        a.x * self.u + a.y * self.v + a.z * self.w
    }
}
