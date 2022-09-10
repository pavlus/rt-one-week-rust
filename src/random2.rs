use std::sync::atomic::{AtomicUsize, Ordering};

use rand::{Error, RngCore, SeedableRng};
use rand_xoshiro::Xoshiro256Plus;

pub type DefaultRng = RngSampler<Xoshiro256Plus>;

impl<R: RngCore> RngCore for RngSampler<R> {
    fn next_u32(&mut self) -> u32 {
        #[cfg(feature = "metrics")]
        self.1.fetch_add(4, Ordering::Relaxed);
        self.0.next_u32()
    }

    fn next_u64(&mut self) -> u64 {
        #[cfg(feature = "metrics")]
        self.1.fetch_add(8, Ordering::Relaxed);
        self.0.next_u64()
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        #[cfg(feature = "metrics")]
        self.1.fetch_add(dest.len(), Ordering::Relaxed);
        self.0.fill_bytes(dest)

    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
        #[cfg(feature = "metrics")]
        self.1.fetch_add(dest.len(), Ordering::Relaxed);
        self.0.try_fill_bytes(dest)
    }
}

#[derive(Debug)]
pub struct RngSampler<T: rand::Rng>(T, #[cfg(feature = "metrics")] pub AtomicUsize);

impl Default for RngSampler<Xoshiro256Plus> {
    fn default() -> Self {
        RngSampler(Xoshiro256Plus::seed_from_u64(0), #[cfg(feature = "metrics")] AtomicUsize::new(0))
    }
}

impl<R: RngCore + SeedableRng> SeedableRng for RngSampler<R> {
    type Seed = R::Seed;

    fn from_seed(seed: Self::Seed) -> Self {
        RngSampler(R::from_seed(seed), #[cfg(feature = "metrics")] AtomicUsize::new(0))
    }
}

