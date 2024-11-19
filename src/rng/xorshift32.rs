use super::SplitMix64;

/// cheap and dirty random numbers
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct XorShift32 {
    x: u32,
}

#[cfg(test)]
#[test]
fn test() {
    let rng = XorShift32::new(14123);
    let _: Vec<_> = rng.take(1034).collect();
}

impl Default for XorShift32 {
    #[inline]
    fn default() -> Self {
        Self { x: 0xCAFE_BABE }
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

impl From<u32> for XorShift32 {
    #[inline]
    fn from(seed: u32) -> Self {
        Self::wrap(seed)
    }
}

impl XorShift32 {
    /// seed the RNG using a `SplitMix64` RNG
    #[inline]
    #[must_use]
    #[expect(clippy::cast_possible_truncation)]
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

    /// generate a new random `u32` value
    #[inline]
    pub const fn next_u32(&mut self) -> u32 {
        self.x = Self::step(self.x);
        self.x
    }

    /// the resulting f32 will be between `[0, 1)`
    /// (0 inclusive, 1 exclusive)
    #[inline]
    #[expect(clippy::cast_precision_loss)]
    pub const fn next_f32(&mut self) -> f32 {
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
