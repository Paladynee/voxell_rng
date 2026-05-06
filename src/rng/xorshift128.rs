use crate::polyfill_next_f32_next_f64_from_fn;

use super::{polyfill::polyfill_fill_bytes_u64, SplitMix64};
use rand_core::RngCore;

/// cheap and dirty random numbers
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct XorShift128 {
    state: [u64; 2],
}

impl Default for XorShift128 {
    /// # Panics
    ///
    /// This will panic if the OS RNG fails to generate a seed
    #[inline]
    #[track_caller]
    fn default() -> Self {
        let mut rand = SplitMix64::default();
        let seed = [rand.mix(), rand.mix()];
        Self::wrap(seed)
    }
}

impl RngCore for XorShift128 {
    /// Fill `dest` with random data.
    #[inline]
    fn fill_bytes(&mut self, dest: &mut [u8]) {
        polyfill_fill_bytes_u64(Self::next_u64)(self, dest);
    }

    /// Return the next random `u32`.
    #[inline]
    fn next_u32(&mut self) -> u32 {
        self.next_u64() as u32
    }

    /// Return the next random `u64`.
    #[inline]
    fn next_u64(&mut self) -> u64 {
        self.advance()
    }
}

impl XorShift128 {
    /// seed the RNG using a `SplitMix64` RNG
    #[inline]
    #[must_use]
    pub const fn new(seed: u64) -> Self {
        let mut smx = SplitMix64::wrap(seed);
        Self::wrap([smx.mix(), smx.mix()])
    }

    /// wrap a value directly into the RNG
    ///
    /// recommended to use `seed_using_splitmix` instead as it will handle 0 seeds
    #[inline]
    #[must_use]
    pub const fn wrap(seed: [u64; 2]) -> Self {
        Self { state: seed }
    }

    polyfill_next_f32_next_f64_from_fn!(
        pub fn next_f32, next_f64(Self::advance = u64);
    );

    /// get the internal state of the RNG without mutating it
    #[inline]
    #[must_use]
    pub const fn get_current_state(&self) -> [u64; 2] {
        self.state
    }

    /// This will not modify the internal state of the RNG.
    /// It will simply return the next random number in the sequence.
    #[inline]
    #[must_use]
    pub const fn peek_next_u64(&self) -> u64 {
        let mut t = self.state[0];
        let s = self.state[1];
        t ^= t << 23;
        t ^= t >> 18;
        t ^= s ^ (s >> 5);
        t.wrapping_add(s)
    }

    #[inline]
    const fn advance(&mut self) -> u64 {
        xorshift128_step(&mut self.state)
    }
}

#[inline]
pub const fn xorshift128_step(x: &mut [u64; 2]) -> u64 {
    let mut t = x[0];
    let s = x[1];
    x[0] = s;
    t ^= t << 23;
    t ^= t >> 18;
    t ^= s ^ (s >> 5);
    x[1] = t;
    t.wrapping_add(s)
}

impl Iterator for XorShift128 {
    type Item = u32;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        match self.next_u32() {
            0 => None,
            x => Some(x),
        }
    }
}
