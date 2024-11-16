use crate::rng::{SplitMix64, XoRoShiRo128Plus, XorShift32};
use core::mem;

/// Trait for branching RNGs.
pub trait BranchRng<T> {
    /// This functionality diverges the `self` random number generator into a
    /// new RNG that won't produce the same sequence of random numbers as the
    /// original RNG.
    fn branch_rng(&mut self) -> T;
}

impl BranchRng<SplitMix64> for SplitMix64 {
    fn branch_rng(&mut self) -> SplitMix64 {
        let seed = self.mix().wrapping_add(1);
        SplitMix64::wrap(seed)
    }
}

impl BranchRng<XoRoShiRo128Plus> for XoRoShiRo128Plus {
    fn branch_rng(&mut self) -> XoRoShiRo128Plus {
        let mut other = self.clone();
        other.long_jump();
        mem::swap(self, &mut other);
        other
    }
}

impl BranchRng<XorShift32> for XorShift32 {
    fn branch_rng(&mut self) -> XorShift32 {
        let seed = self.next_u32().wrapping_add(1);
        XorShift32::wrap(seed)
    }
}
