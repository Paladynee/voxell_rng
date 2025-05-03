use rand_core::RngCore;

use crate::getrandom::GetRandom;

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
        if dest.is_empty() {
            return;
        }

        let mut byte_iterator = dest.chunks_exact_mut(8);
        for slice in byte_iterator.by_ref() {
            let next = self.next_u64();
            let next_bytes = next.to_le_bytes();
            slice.copy_from_slice(next_bytes.as_slice());
        }

        let remainder = byte_iterator.into_remainder();
        if !remainder.is_empty() {
            let next = self.next_u64();
            let next_bytes = next.to_le_bytes();
            remainder.copy_from_slice(&next_bytes[..remainder.len()]);
        }
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
        self.x = self.x.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut z = self.x;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        z ^ (z >> 31)
    }

    /// get the internal state of the RNG without mutating it
    #[inline]
    #[must_use]
    pub const fn get_current_state(&self) -> u64 {
        self.x
    }
}
