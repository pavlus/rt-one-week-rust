use nalgebra::Unit;

use crate::V3;

#[derive(Debug, Copy, Clone)]
pub struct ONB {
    pub u: Unit<V3>,
    pub v: Unit<V3>,
    pub w: Unit<V3>,
}

impl ONB {
    pub fn from_w(w: Unit<V3>) -> ONB {
        let a = if w.x.abs() > 0.9 { V3::new(0., 1., 0.) } else { V3::new(1., 0., 0.) };
        let v = Unit::new_normalize(w.cross(&a));
        let u = Unit::new_unchecked(w.cross(&v));
        ONB {
            u,
            v,
            w,
        }
    }
    pub fn from_up_w(up: Unit<V3>, w: Unit<V3>) -> ONB {
        // cross-product of upwards vector and w will give us normal to plane they are in.
        // it's also normal to both of them, being normal to upwards direction makes it horizontal
        let u = Unit::new_normalize(up.cross(&w));
        // given that we have u and w is normal to plane of viewport -- v is their cross-product
        let v = Unit::new_unchecked(w.cross(&u));
        ONB {
            u,
            v,
            w,
        }
    }

    pub fn local(&self, a: &V3) -> V3 {
        a.x * self.u.as_ref() + a.y * self.v.as_ref() + a.z * self.w.as_ref()
    }
}

#[cfg(test)]
mod test {
    use nalgebra::Unit;
    use rand::distributions::uniform::SampleRange;
    use rand::prelude::Distribution;
    use rand_distr::UnitSphere;

    use crate::V3;
    use crate::onb::ONB;
    use crate::random2::DefaultRng;
    use crate::types::Geometry;

    const EPSILON: f64 = 1e-9;

    #[test]
    fn test_preserves_unit_length() {
        let mut rng = DefaultRng::default();
        let w = Unit::new_normalize(V3::from(UnitSphere.sample(&mut rng)));
        let onb = ONB::from_w(w);
        let test = Unit::new_normalize(V3::from(UnitSphere.sample(&mut rng)));
        let value = onb.local(&test.into_inner()).norm_squared();
        let test = test.norm_squared();
        assert!((test - value).abs() < EPSILON, "test: {}, value: {}", test, value);
    }

    //#[test]
    fn bench_local() {
        let count = 1000_000;
        let mut rng = DefaultRng::default();
        let mut blackhole = Vec::<V3>::with_capacity(count);
        let mut sum: Geometry = 0.0;

        let w = Unit::new_unchecked(V3::from(UnitSphere.sample(&mut rng)));
        let onb = ONB::from_w(w);
        for _ in 0..count {
            let scale = (0.5..5.0).sample_single(&mut rng);
            sum += scale;
            let test = V3::from(UnitSphere.sample(&mut rng)) * scale;
            let value = onb.local(&test);
            blackhole.push(value);
        }
        let test: Geometry = blackhole.into_iter().map(|v| v.norm()).sum::<Geometry>();
        assert!((test - sum).abs() < EPSILON, "test: {}", test);
    }
}
