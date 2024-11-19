use super::SplitMix64;

/// bigger cheap and dirty random numbers
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct XoRoShiRo128Plus {
    state: [u64; 2],
}

impl Default for XoRoShiRo128Plus {
    #[inline]
    fn default() -> Self {
        Self::new(0)
    }
}

impl Iterator for XoRoShiRo128Plus {
    type Item = u64;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        match self.next_u64() {
            0 => None,
            x => Some(x),
        }
    }
}

impl XoRoShiRo128Plus {
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

    /// generate a new random `u64` value
    #[inline]
    pub const fn next_u64(&mut self) -> u64 {
        let s0 = self.state[0];
        let mut s1 = self.state[1];
        let result = s0.wrapping_add(s1);

        s1 ^= s0;
        self.state[0] = s0.rotate_left(24) ^ s1 ^ (s1 << 16);
        self.state[1] = s1.rotate_left(37);

        result
    }

    /// the resulting f32 will be between `[0, 1)`
    /// (0 inclusive, 1 exclusive)
    #[inline]
    #[expect(clippy::cast_precision_loss)]
    pub const fn next_f64(&mut self) -> f64 {
        self.next_u64() as f64 / u64::MAX as f64
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
}
