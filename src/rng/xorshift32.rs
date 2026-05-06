use super::polyfill::polyfill_fill_bytes_u32;
use super::SplitMix64;
use crate::polyfill_next_f32_next_f64_from_fn;
use rand_core::RngCore;

/// cheap and dirty random numbers
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct XorShift32 {
    x: u32,
}

impl XorShift32 {
    /// seed the RNG using a `SplitMix64` RNG
    #[inline]
    #[must_use]
    pub const fn new(seed: u64) -> Self {
        let mut smx = SplitMix64::wrap(seed);
        Self::wrap(smx.mix() as u32)
    }

    /// wrap a value directly into the RNG
    ///
    /// recommended to use `seed_using_splitmix` instead as it will handle 0 seeds
    #[inline]
    #[must_use]
    #[track_caller]
    pub const fn wrap(seed: u32) -> Self {
        assert!(seed != 0, "XorShift32 cannot be seeded with 0");
        Self { x: seed }
    }

    polyfill_next_f32_next_f64_from_fn!(
        pub fn next_f32, next_f64(Self::advance = u32);
    );

    /// This will not modify the internal state of the RNG.
    /// It will simply return the next random number in the sequence.
    #[inline]
    pub const fn peek_next_u32(&mut self) -> u32 {
        xorshift32_step(self.x)
    }

    /// get the internal state of the RNG without mutating it
    #[inline]
    #[must_use]
    pub const fn get_current_state(&self) -> u32 {
        self.x
    }

    #[inline]
    const fn advance(&mut self) -> u32 {
        self.x = xorshift32_step(self.x);
        self.x
    }
}

#[inline]
pub const fn xorshift32_step(mut x: u32) -> u32 {
    x ^= x << 13;
    x ^= x >> 17;
    x ^= x << 5;
    x
}

impl Default for XorShift32 {
    /// # Panics
    ///
    /// This will panic if the OS RNG fails to generate a seed
    #[inline]
    #[track_caller]
    fn default() -> Self {
        let mut seed = SplitMix64::default();
        Self::wrap(seed.mix() as u32)
    }
}

impl RngCore for XorShift32 {
    /// Fill `dest` with random data.
    #[inline]
    fn fill_bytes(&mut self, dest: &mut [u8]) {
        polyfill_fill_bytes_u32(Self::next_u32)(self, dest);
    }

    /// Return the next random `u32`.
    #[inline]
    fn next_u32(&mut self) -> u32 {
        self.advance()
    }

    /// Return the next random `u64`.
    #[inline]
    fn next_u64(&mut self) -> u64 {
        ((u64::from(self.advance())) << 32) | u64::from(self.advance())
    }
}

impl Iterator for XorShift32 {
    type Item = u32;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        match self.next_u32() {
            0 => None,
            x => Some(x),
        }
    }
}
