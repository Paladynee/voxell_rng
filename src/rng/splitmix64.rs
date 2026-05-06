use super::polyfill::polyfill_fill_bytes_u64;
use crate::{genrandom::GenRandom, polyfill_next_f32_next_f64_from_fn};
use rand_core::RngCore;

/// an RNG engine used for seeding other RNGs
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct SplitMix64 {
    x: u64,
}

impl Default for SplitMix64 {
    #[inline]
    fn default() -> Self {
        let seed = u64::get_random().unwrap();
        Self::wrap(seed)
    }
}

impl Iterator for SplitMix64 {
    type Item = u64;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        Some(self.mix())
    }
}

impl RngCore for SplitMix64 {
    /// Fill `dest` with random data.
    #[inline]
    fn fill_bytes(&mut self, dest: &mut [u8]) {
        polyfill_fill_bytes_u64(Self::mix)(self, dest);
    }

    /// Return the next random `u32`.
    #[inline]
    fn next_u32(&mut self) -> u32 {
        self.next_u64() as u32
    }

    /// Return the next random `u64`.
    #[inline]
    fn next_u64(&mut self) -> u64 {
        self.mix()
    }
}

impl SplitMix64 {
    /// seed the RNG
    ///
    /// no special handling for 0 seeds since `SplitMix64` is designed
    /// to be used as a seed generator
    #[inline]
    #[must_use]
    pub const fn new(seed: u64) -> Self {
        Self::wrap(seed)
    }

    /// encase a value directly into the RNG
    #[inline]
    #[must_use]
    pub const fn wrap(seed: u64) -> Self {
        Self { x: seed }
    }

    /// generate a new random `u64` value
    #[inline]
    pub const fn mix(&mut self) -> u64 {
        splitmix64_step(&mut self.x)
    }

    polyfill_next_f32_next_f64_from_fn!(
        pub fn next_f32, next_f64(Self::mix = u64);
    );

    /// get the internal state of the RNG without mutating it
    #[inline]
    #[must_use]
    pub const fn get_current_state(&self) -> u64 {
        self.x
    }
}

/// generate a new random `u64` value
#[inline]
pub const fn splitmix64_step(x: &mut u64) -> u64 {
    *x = x.wrapping_add(0x9e37_79b9_7f4a_7c15);
    let mut z = *x;
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    z ^ (z >> 31)
}
