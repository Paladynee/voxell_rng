use rand_core::RngCore;

use crate::{genrandom::GenRandom, polyfill_next_f32_next_f64_from_fn};

use super::{polyfill::polyfill_fill_bytes_u64, SplitMix64};

/// bigger cheap and dirty random numbers
///
/// this is the xorshiro128+ implementation
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct XoRoShiRo128 {
    state: [u64; 2],
}

impl Default for XoRoShiRo128 {
    /// # Panics
    ///
    /// This will panic if the OS RNG fails to generate a seed
    #[inline]
    #[track_caller]
    fn default() -> Self {
        let seed = [u64::get_random().unwrap(), u64::get_random().unwrap()];
        Self::wrap(seed)
    }
}

impl RngCore for XoRoShiRo128 {
    /// Fill `dest` with random data.
    #[inline]
    fn fill_bytes(&mut self, dest: &mut [u8]) {
        polyfill_fill_bytes_u64(Self::next_u64)(self, dest);
    }

    /// Return the next random `u32`.
    #[inline]
    fn next_u32(&mut self) -> u32 {
        self.next_u64() as u32
    }

    /// Return the next random `u64`.
    #[inline]
    fn next_u64(&mut self) -> u64 {
        self.advance()
    }
}

impl XoRoShiRo128 {
    const SHORT_JUMP_TABLE: [u64; 2] = [0xdf90_0294_d8f5_54a5, 0x1708_65df_4b32_01fc];
    const LONG_JUMP_TABLE: [u64; 2] = [0xd2a9_8b26_625e_ee7b, 0xdddf_9b10_90aa_7ac1];

    /// seed the RNG using a `SplitMix64` RNG
    #[inline]
    #[must_use]
    pub const fn new(seed: u64) -> Self {
        let mut smx = SplitMix64::wrap(seed);
        let seed = [smx.mix(), smx.mix()];
        Self::wrap(seed)
    }

    /// wrap a value directly into the RNG
    ///
    /// recommended to use `seed_using_splitmix` instead as it will handle 0 seeds
    #[inline]
    #[must_use]
    pub const fn wrap(seed: [u64; 2]) -> Self {
        assert!(seed[0] != 0, "XoRoShiRo128 cannot be seeded with 0");
        Self { state: seed }
    }

    polyfill_next_f32_next_f64_from_fn!(
        pub fn next_f32, next_f64(Self::advance = u64);
    );

    // pub fn jump(&mut self) {
    //     let mut s0 = 0;
    //     let mut s1 = 0;

    //     for i in Self::SHORT_JUMP_TABLE {
    //         for b in 0..64 {
    //             if i & (1 << b) != 0 {
    //                 s0 ^= self.state[0];
    //                 s1 ^= self.state[1];
    //             }
    //             self.next_u64();
    //         }
    //     }

    //     self.state[0] = s0;
    //     self.state[1] = s1;
    // }

    /// jump the RNG forward by a some amount based on the jump table
    ///
    /// used by the `BranchRng` trait.
    #[inline]
    pub const fn jump(&mut self) {
        let mut s0 = 0;
        let mut s1 = 0;

        let mut i: usize = 0;
        while i < Self::SHORT_JUMP_TABLE.len() {
            let mut b: usize = 0;

            while b < 64 {
                if Self::SHORT_JUMP_TABLE[i] & (1 << b) != 0 {
                    s0 ^= self.state[0];
                    s1 ^= self.state[1];
                }
                b = b.wrapping_add(1);
            }

            i = i.wrapping_add(1);
        }

        self.state[0] = s0;
        self.state[1] = s1;
    }

    // pub fn long_jump(&mut self) {
    //     let mut s0 = 0;
    //     let mut s1 = 0;
    //     for i in Self::LONG_JUMP_TABLE {
    //         for b in 0..64 {
    //             if i & (1 << b) != 0 {
    //                 s0 ^= self.state[0];
    //                 s1 ^= self.state[1];
    //             }
    //             self.next_u64();
    //         }
    //     }

    //     self.state[0] = s0;
    //     self.state[1] = s1;
    // }4

    /// jump the RNG forward by a large amount based on the jump table
    ///
    /// used by the `BranchRng` trait.
    #[inline]
    pub const fn long_jump(&mut self) {
        let mut s0 = 0;
        let mut s1 = 0;

        let mut i: usize = 0;

        while i < Self::LONG_JUMP_TABLE.len() {
            let mut b: usize = 0;

            while b < 64 {
                if Self::LONG_JUMP_TABLE[i] & (1 << b) != 0 {
                    s0 ^= self.state[0];
                    s1 ^= self.state[1];
                }
                b = b.wrapping_add(1);
            }

            i = i.wrapping_add(1);
        }

        self.state[0] = s0;
        self.state[1] = s1;
    }

    /// get the internal state of the RNG without mutating it
    #[inline]
    #[must_use]
    pub const fn get_current_state(&self) -> [u64; 2] {
        self.state
    }

    #[inline]
    const fn advance(&mut self) -> u64 {
        xoroshiro128_step(&mut self.state)
    }
}
/// generate a new random `u64` value
///
/// this function is intentionally not public,
#[inline]
pub const fn xoroshiro128_step(state: &mut [u64; 2]) -> u64 {
    let s0 = state[0];
    let mut s1 = state[1];
    let result = s0.wrapping_add(s1);

    s1 ^= s0;
    state[0] = s0.rotate_left(24) ^ s1 ^ (s1 << 16);
    state[1] = s1.rotate_left(37);

    result
}

impl Iterator for XoRoShiRo128 {
    type Item = u64;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        match self.advance() {
            0 => None,
            x => Some(x),
        }
    }
}
