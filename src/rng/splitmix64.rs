use crate::runtime_seeded::MagicSeed;

/// an RNG engine used for seeding other RNGs
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct SplitMix64 {
    x: u64,
}

impl Default for SplitMix64 {
    #[inline]
    fn default() -> Self {
        Self::new(MagicSeed::new_magic() as u64)
    }
}

impl Iterator for SplitMix64 {
    type Item = u64;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        Some(self.mix())
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
