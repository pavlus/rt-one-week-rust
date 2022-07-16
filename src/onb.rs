use crate::V3;
use nalgebra::{Matrix3, Unit};
use crate::types::Geometry;

#[derive(Debug, Copy, Clone)]
pub struct ONB {
    pub u: Unit<V3>,
    pub v: Unit<V3>,
    pub w: Unit<V3>,
}

impl ONB{
    pub fn from_w(w: Unit<V3>) -> ONB {
        let a = if w.x.abs() > 0.9 { V3::new(0., 1., 0.) } else { V3::new(1., 0., 0.) };
        let v = Unit::new_normalize(w.cross(&a));
        let u = Unit::new_unchecked(w.cross(&v));
        ONB {
            u, v, w,
        }
    }
    pub fn from_up_w(up: Unit<V3>, w: Unit<V3>) -> ONB {
        // cross-product of upwards vector and w will give us normal to plane they are in.
        // it's also normal to both of them, being normal to upwards direction makes it horizontal
        let u = Unit::new_normalize(up.cross(&w));
        // given that we have u and w is normal to plane of viewport -- v is their cross-product
        let v = Unit::new_normalize(w.cross(&u)); // fixme: this normalize looks strange
        ONB {
            u, v, w,
        }
    }

    pub fn local(&self, a: &V3) -> V3 {
        a.x * self.u.as_ref() + a.y * self.v.as_ref() + a.z * self.w.as_ref()
    }
}

#[cfg(test)]
mod test {
    use nalgebra::Unit;
    use crate::onb::ONB;
    use crate::{random, V3};
    use crate::types::Geometry;

    const EPSILON: f64 = 1e-9;

    #[test]
    fn test_preserves_unit_length(){
        let w = Unit::new_normalize(random::rand_in_unit_sphere().coords);
        let onb = ONB::from_w(w);
        let test = Unit::new_normalize(random::rand_in_unit_sphere().coords);
        let value = onb.local(&test.into_inner()).norm();
        let test = test.norm();
        assert!((test - value).abs() < EPSILON, "test: {}, value: {}", test, value);
    }

    //#[test]
    fn bench_local(){
        let count = 1000_000;
        let mut blackhole = Vec::<V3>::with_capacity(count);
        let mut sum: Geometry = 0.0;

        let w = Unit::new_normalize(random::rand_in_unit_sphere().coords);
        let onb = ONB::from_w(w);
        for _ in 0..count{
            let scale = random::next_std_f64_in_range(&(0.5..5.0));
            sum +=scale;
            let test = random::rand_in_unit_sphere().coords * scale;
            let value = onb.local(&test);
            blackhole.push(value);
        }
        let test: Geometry = blackhole.into_iter().map(|v| v.norm()).sum::<Geometry>();
        assert!((test - sum).abs() < EPSILON, "test: {}", test);
    }



}
