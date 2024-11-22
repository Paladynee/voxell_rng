use crate::rng::{SplitMix64, XoRoShiRo128, XorShift32};
use core::mem;
use rand_core::RngCore;

/// Trait for branching RNGs.
pub trait BranchRng<T> {
    /// This functionality diverges the `self` random number generator into a
    /// new RNG that won't produce the same sequence of random numbers as the
    /// original RNG.
    fn branch_rng(&mut self) -> T;
}

impl BranchRng<Self> for SplitMix64 {
    #[inline]
    fn branch_rng(&mut self) -> Self {
        let seed = self.mix().wrapping_add(1);
        Self::wrap(seed)
    }
}

impl BranchRng<Self> for XoRoShiRo128 {
    #[inline]
    fn branch_rng(&mut self) -> Self {
        let mut other = self.clone();
        other.long_jump();
        mem::swap(self, &mut other);
        other
    }
}

impl BranchRng<Self> for XorShift32 {
    #[inline]
    fn branch_rng(&mut self) -> Self {
        let seed = self.next_u32().wrapping_add(1);
        if seed == 0 {
            Self::wrap(1)
        } else {
            Self::wrap(seed)
        }
    }
}
