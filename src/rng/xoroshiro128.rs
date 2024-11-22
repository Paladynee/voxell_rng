use rand_core::RngCore;

use super::SplitMix64;

/// bigger cheap and dirty random numbers
///
/// this is the xorshiro128+ implementation
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct XoRoShiRo128 {
    state: [u64; 2],
}

impl Iterator for XoRoShiRo128 {
    type Item = u64;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        match self.step() {
            0 => None,
            x => Some(x),
        }
    }
}

impl RngCore for XoRoShiRo128 {
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
        self.step()
    }

    #[inline]
    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand_core::Error> {
        self.fill_bytes(dest);
        Ok(())
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
        Self { state: seed }
    }

    /// the resulting f32 will be between `[0, 1)`
    /// (0 inclusive, 1 exclusive)
    #[inline]
    #[expect(clippy::cast_precision_loss)]
    pub const fn next_f64(&mut self) -> f64 {
        self.step() as f64 / u64::MAX as f64
    }

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

    /// generate a new random `u64` value
    ///
    /// this function is intentionally not public,
    /// use [`rand_core::RngCore::next_u64`] instead.
    #[inline]
    const fn step(&mut self) -> u64 {
        let s0 = self.state[0];
        let mut s1 = self.state[1];
        let result = s0.wrapping_add(s1);

        s1 ^= s0;
        self.state[0] = s0.rotate_left(24) ^ s1 ^ (s1 << 16);
        self.state[1] = s1.rotate_left(37);

        result
    }
}
