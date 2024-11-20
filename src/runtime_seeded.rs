use crate::rng::pcg_advanced::pcg_32::PcgInnerState32;
use crate::rng::pcg_advanced::pcg_64::PcgInnerState64;
use crate::rng::{SplitMix64, XoRoShiRo128Plus, XorShift32};
use core::ptr;
use core::sync::atomic;
use core::sync::atomic::AtomicU64;

pub(crate) struct MagicSeed;
static MAGICXORSHIFT32_CALLED_AMOUNT: AtomicU64 = AtomicU64::new(0);

/// seed an [`XorShift32`] RNG using dark arts
pub struct MagicallySeededXorShift32;

impl MagicallySeededXorShift32 {
    /// seed using global state and this function's address
    #[inline]
    #[must_use]
    pub fn new_magic() -> XorShift32 {
        let seed = MagicSeed::new_magic();
        XorShift32::new(seed as u64)
    }

    /// seed using the reference's address
    #[inline]
    #[must_use]
    pub fn new_with_reference<T>(reference: &T) -> XorShift32 {
        let seed = MagicSeed::new_with_reference(reference);
        XorShift32::new(seed as u64)
    }

    /// seed using a combination of multiple RNGs
    #[inline]
    #[must_use]
    pub fn mad() -> XorShift32 {
        let seed = MagicSeed::mad();
        XorShift32::new(seed as u64)
    }
}

/// seed a [`SplitMix64`] RNG using dark arts
pub struct MagicallySeededSplitMix64;

impl MagicallySeededSplitMix64 {
    /// seed using global state and this function's address
    #[inline]
    #[must_use]
    pub fn new_magic() -> SplitMix64 {
        let seed = MagicSeed::new_magic();
        SplitMix64::new(seed as u64)
    }

    /// seed using the reference's address
    #[inline]
    #[must_use]
    pub fn new_with_reference<T>(reference: &T) -> SplitMix64 {
        let seed = MagicSeed::new_with_reference(reference);
        SplitMix64::new(seed as u64)
    }

    /// seed using a combination of multiple RNGs
    #[inline]
    #[must_use]
    pub fn mad() -> SplitMix64 {
        let seed = MagicSeed::mad();
        SplitMix64::new(seed as u64)
    }
}

/// seed a [`XoRoShiRo128Plus`] RNG using dark arts
pub struct MagicallySeededXoRoShiRo128Plus;

impl MagicallySeededXoRoShiRo128Plus {
    /// seed using global state and this function's address
    #[inline]
    #[must_use]
    pub fn new_magic() -> XoRoShiRo128Plus {
        let seed = MagicSeed::new_magic();
        XoRoShiRo128Plus::new(seed as u64)
    }

    /// seed using the reference's address
    #[inline]
    #[must_use]
    pub fn new_with_reference<T>(reference: &T) -> XoRoShiRo128Plus {
        let seed = MagicSeed::new_with_reference(reference);
        XoRoShiRo128Plus::new(seed as u64)
    }

    /// seed using a combination of multiple RNGs
    #[inline]
    #[must_use]
    pub fn mad() -> XoRoShiRo128Plus {
        let seed = MagicSeed::mad();
        XoRoShiRo128Plus::new(seed as u64)
    }
}

impl MagicSeed {
    #[inline]
    #[expect(clippy::cast_possible_truncation)]
    pub fn new_magic() -> usize {
        let executed_amount = MAGICXORSHIFT32_CALLED_AMOUNT.fetch_add(1, atomic::Ordering::Relaxed);
        (Self::new_magic as fn() -> usize as usize).wrapping_add(executed_amount as usize)
    }

    #[inline]
    pub fn new_with_reference<T>(reference: &T) -> usize {
        ptr::from_ref::<T>(reference) as usize
    }

    #[inline]
    #[must_use]
    #[expect(clippy::cast_possible_truncation)]
    pub fn mad() -> usize {
        let mut first_seed = Self::mad as fn() -> usize as usize as u32;
        let mut pcg32 = PcgInnerState32::oneseq_seeded(first_seed);
        first_seed = first_seed.wrapping_add(1);
        let mut pcg64 = PcgInnerState64::oneseq_seeded(u64::from(first_seed));
        first_seed = first_seed.wrapping_add(1);
        let mut xorshift32 = XorShift32::new(u64::from(first_seed));
        first_seed = first_seed.wrapping_add(1);
        let mut xoroshiro = XoRoShiRo128Plus::new(u64::from(first_seed));
        first_seed = first_seed.wrapping_add(1);
        let mut splitmix = SplitMix64::new(u64::from(first_seed));

        let results = [
            pcg32.oneseq_rxs_m_xs(),
            pcg64.mcg_xsl_rr(),
            xorshift32.next_u32(),
            xoroshiro.next_u64() as u32,
            pcg32.unique_rxs_m_xs(),
            pcg64.unique_xsl_rr(),
            xorshift32.next_u32(),
            xoroshiro.next_u64() as u32,
            splitmix.mix() as u32,
        ];

        (results[0]
            ^ results[1]
            ^ results[2]
            ^ results[3]
            ^ results[4]
            ^ results[5]
            ^ results[6]
            ^ results[7]
            ^ results[8]) as usize
    }
}
