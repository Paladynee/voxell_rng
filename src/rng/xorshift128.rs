use super::SplitMix64;
use rand_core::RngCore;

/// cheap and dirty random numbers
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct XorShift128 {
    state: [u64; 2],
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

impl RngCore for XorShift128 {
    #[inline]
    fn fill_bytes(&mut self, dest: &mut [u8]) {
        let mut chunksmut = dest.chunks_exact_mut(8);
        for chunk in chunksmut.by_ref() {
            let next = self.next_u64();
            let bytes = next.to_le_bytes();
            chunk.copy_from_slice(&bytes);
        }
        let a = chunksmut.into_remainder();
        if !a.is_empty() {
            let next = self.next_u64();
            let bytes = next.to_le_bytes();
            a.copy_from_slice(&bytes[0..a.len()]);
        }
    }

    #[inline]
    #[expect(clippy::cast_possible_truncation)]
    fn next_u32(&mut self) -> u32 {
        self.next_u64() as u32
    }

    #[inline]
    fn next_u64(&mut self) -> u64 {
        Self::step(&mut self.state)
    }

    #[inline]
    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand_core::Error> {
        self.fill_bytes(dest);
        Ok(())
    }
}

impl From<[u64; 2]> for XorShift128 {
    /// warning! this will [`wrap`] the value
    ///
    /// [`wrap`]: XorShift128::wrap
    ///
    /// wrapping a 0 value will cause the RNG to always yield 0!
    #[inline]
    fn from(seed: [u64; 2]) -> Self {
        Self::wrap(seed)
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

    /// the resulting f32 will be between `[0, 1)`
    /// (0 inclusive, 1 exclusive)
    #[inline]
    #[expect(clippy::cast_precision_loss)]
    pub fn next_f32(&mut self) -> f32 {
        self.next_u32() as f32 / u32::MAX as f32
    }

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
    const fn step(x: &mut [u64; 2]) -> u64 {
        let mut t = x[0];
        let s = x[1];
        x[0] = s;
        t ^= t << 23;
        t ^= t >> 18;
        t ^= s ^ (s >> 5);
        x[1] = t;
        t.wrapping_add(s)
    }
}
