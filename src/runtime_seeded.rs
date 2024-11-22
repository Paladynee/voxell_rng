use crate::rng::pcg_advanced::pcg_32::PcgInnerState32;
use crate::rng::pcg_advanced::pcg_64::PcgInnerState64;
use crate::rng::{SplitMix64, XoRoShiRo128, XorShift32};
use core::ptr;
use core::sync::atomic;
use core::sync::atomic::AtomicU64;
use rand_core::RngCore;

pub(crate) struct MagicSeed;
static MAGICXORSHIFT32_CALLED_AMOUNT: AtomicU64 = AtomicU64::new(0);

/// seed an [`XorShift32`] RNG using dark arts
pub struct MagicallySeededXorShift32;

impl MagicallySeededXorShift32 {
    /// seed using global state and this function's address
    ///
    /// # Errors
    ///
    /// returns an error if the system's random number generator fails
    #[inline]
    pub fn new_magic() -> Result<XorShift32, getrandom::Error> {
        let seed = MagicSeed::new_magic()?;
        Ok(XorShift32::new(seed as u64))
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
    ///
    /// # Errors
    ///
    /// returns an error if the system's random number generator fails
    #[inline]
    pub fn new_magic() -> Result<SplitMix64, getrandom::Error> {
        let seed = MagicSeed::new_magic()?;
        Ok(SplitMix64::new(seed as u64))
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
    ///
    /// # Errors
    ///
    /// returns an error if the system's random number generator fails
    #[inline]
    pub fn new_magic() -> Result<XoRoShiRo128, getrandom::Error> {
        let seed = MagicSeed::new_magic()?;
        Ok(XoRoShiRo128::new(seed as u64))
    }

    /// seed using the reference's address
    #[inline]
    #[must_use]
    pub fn new_with_reference<T>(reference: &T) -> XoRoShiRo128 {
        let seed = MagicSeed::new_with_reference(reference);
        XoRoShiRo128::new(seed as u64)
    }

    /// seed using a combination of multiple RNGs
    #[inline]
    #[must_use]
    pub fn mad() -> XoRoShiRo128 {
        let seed = MagicSeed::mad();
        XoRoShiRo128::new(seed as u64)
    }
}

impl MagicSeed {
    #[inline]
    #[expect(clippy::cast_possible_truncation)]
    pub fn new_magic() -> Result<usize, getrandom::Error> {
        let executed_amount = MAGICXORSHIFT32_CALLED_AMOUNT.fetch_add(0, atomic::Ordering::Acquire);
        let mut seed = [0u8; 8];
        getrandom::getrandom(&mut seed).map(|()| {
            let seed = u64::from_le_bytes(seed);
            let mixed = SplitMix64::mix(&mut SplitMix64::wrap(seed));
            MAGICXORSHIFT32_CALLED_AMOUNT.fetch_add(
                mixed.wrapping_sub(executed_amount),
                atomic::Ordering::Release,
            );
            mixed as usize
        })
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
        let mut xoroshiro = XoRoShiRo128::new(u64::from(first_seed));
        first_seed = first_seed.wrapping_add(1);
        let mut splitmix = SplitMix64::new(u64::from(first_seed));

        let results = [
            pcg32.oneseq_rxs_m_xs(),
            pcg64.mcg_xsl_rr(),
            xorshift32.next_u32(),
            xoroshiro.next_u32(),
            pcg32.unique_rxs_m_xs(),
            pcg64.unique_xsl_rr(),
            xorshift32.next_u32(),
            xoroshiro.next_u32(),
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
