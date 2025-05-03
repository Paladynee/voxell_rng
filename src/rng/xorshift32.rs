use super::SplitMix64;
use crate::getrandom::GetRandom;
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
    pub const fn wrap(seed: u32) -> Self {
        debug_assert!(seed != 0, "XorShift32 cannot be seeded with 0");
        Self { x: seed }
    }

    /// the resulting f32 will be between `[0, 1)`
    /// (0 inclusive, 1 exclusive)
    #[inline]
    pub fn next_f32(&mut self) -> f32 {
        self.next_u32() as f32 / u32::MAX as f32
    }

    /// This will not modify the internal state of the RNG.
    /// It will simply return the next random number in the sequence.
    #[inline]
    pub const fn peek_next_u32(&mut self) -> u32 {
        Self::step(self.x)
    }

    /// get the internal state of the RNG without mutating it
    #[inline]
    #[must_use]
    pub const fn get_current_state(&self) -> u32 {
        self.x
    }

    #[inline]
    const fn step(mut x: u32) -> u32 {
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        x
    }
}

impl From<u32> for XorShift32 {
    #[inline]
    fn from(seed: u32) -> Self {
        Self::wrap(seed)
    }
}

impl Default for XorShift32 {
    /// # Panics
    ///
    /// This will panic if the OS RNG fails to generate a seed
    #[inline]
    #[track_caller]
    fn default() -> Self {
        let seed = u32::get_random().unwrap();
        Self::wrap(seed)
    }
}

impl RngCore for XorShift32 {
    /// Fill `dest` with random data.
    #[inline]
    fn fill_bytes(&mut self, dest: &mut [u8]) {
        if dest.is_empty() {
            return;
        }

        let mut byte_iterator = dest.chunks_exact_mut(4);

        for slice in byte_iterator.by_ref() {
            let next = self.next_u32();
            let next_bytes = next.to_le_bytes();
            slice.copy_from_slice(next_bytes.as_slice());
        }

        let remainder = byte_iterator.into_remainder();
        if !remainder.is_empty() {
            let next = self.next_u32();
            let next_bytes = next.to_le_bytes();
            remainder.copy_from_slice(&next_bytes[..remainder.len()]);
        }
    }

    /// Return the next random `u32`.
    #[inline]
    fn next_u32(&mut self) -> u32 {
        self.x = Self::step(self.x);
        self.x
    }

    /// Return the next random `u64`.
    #[inline]
    fn next_u64(&mut self) -> u64 {
        ((u64::from(self.next_u32())) << 32) | u64::from(self.next_u32())
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
