/// cheap and dirty random numbers
#[derive(Clone, Debug)]
pub struct XorShift32 {
    x: u32,
}

impl Default for XorShift32 {
    fn default() -> Self {
        Self { x: 0xCAFEBABE }
    }
}

impl XorShift32 {
    /// wrap a value directly into the RNG
    ///
    /// recommended to use seed_using_splitmix instead as it will handle 0 seeds
    pub const fn wrap(seed: u32) -> Self {
        Self { x: seed }
    }

    pub const fn next_u32(&mut self) -> u32 {
        self.x = Self::step(self.x);
        self.x
    }

    /// the resulting f32 will be between `[0, 1)`
    /// (0 inclusive, 1 exclusive)
    pub const fn next_f32(&mut self) -> f32 {
        self.next_u32() as f32 / u32::MAX as f32
    }

    /// This will not modify the internal state of the RNG.
    /// It will simply return the next random number in the sequence.
    pub const fn peek_next_u32(&mut self) -> u32 {
        Self::step(self.x)
    }

    pub const fn get_current_state(&self) -> u32 {
        self.x
    }

    pub const fn seed_using_splitmix(seed: u64) -> Self {
        let mut smx = SplitMix64::wrap(seed);
        Self::wrap(smx.mix() as u32)
    }

    #[inline]
    const fn step(mut x: u32) -> u32 {
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        x
    }
}

/// an RNG engine used for seeding other RNGs
#[derive(Clone, Default)]
pub struct SplitMix64 {
    x: u64,
}

impl SplitMix64 {
    pub const fn wrap(seed: u64) -> Self {
        Self { x: seed }
    }

    pub const fn mix(&mut self) -> u64 {
        self.x = self.x.wrapping_add(0x9e3779b97f4a7c15);
        let mut z = self.x;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
        z ^ (z >> 31)
    }

    pub const fn get_current_state(&self) -> u64 {
        self.x
    }
}

/// bigger cheap and dirty random numbers
#[derive(Clone)]
pub struct XoRoShiRo128Plus {
    state: [u64; 2],
}

impl XoRoShiRo128Plus {
    const SHORT_JUMP_TABLE: [u64; 2] = [0xdf900294d8f554a5, 0x170865df4b3201fc];
    const LONG_JUMP_TABLE: [u64; 2] = [0xd2a98b26625eee7b, 0xdddf9b1090aa7ac1];

    /// wrap a value directly into the RNG
    ///
    /// recommended to use seed_using_splitmix instead as it will handle 0 seeds
    pub const fn wrap(seed: [u64; 2]) -> Self {
        Self { state: seed }
    }

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

    pub const fn jump(&mut self) {
        let mut s0 = 0;
        let mut s1 = 0;

        let mut i = 0;
        while i < Self::SHORT_JUMP_TABLE.len() {
            let mut b = 0;

            while b < 64 {
                if Self::SHORT_JUMP_TABLE[i] & (1 << b) != 0 {
                    s0 ^= self.state[0];
                    s1 ^= self.state[1];
                }
                b += 1;
            }

            i += 1;
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
    // }

    pub const fn long_jump(&mut self) {
        let mut s0 = 0;
        let mut s1 = 0;

        let mut i = 0;

        while i < Self::LONG_JUMP_TABLE.len() {
            let mut b = 0;

            while b < 64 {
                if Self::LONG_JUMP_TABLE[i] & (1 << b) != 0 {
                    s0 ^= self.state[0];
                    s1 ^= self.state[1];
                }
                b += 1;
            }

            i += 1;
        }

        self.state[0] = s0;
        self.state[1] = s1;
    }

    pub const fn seed_using_splitmix(seed: u64) -> Self {
        let mut smx = SplitMix64::wrap(seed);
        let seed = [smx.mix(), smx.mix()];
        Self::wrap(seed)
    }

    pub const fn get_current_state(&self) -> [u64; 2] {
        self.state
    }
}

// pub struct Pcg32 {
//     state: u64,
//     inc: u64,
// }

// impl Pcg32 {
//     pub const fn default() -> Self {
//         Pcg32 {
//             state: 0x853c49e6748fea9bu64,
//             inc: 0xda3e39cb94b95bdbu64,
//         }
//     }

//     pub fn seed(&mut self, initstate: u64, initseq: u64) {
//         self.state = 0;
//         self.inc = (initseq << 1) | 1;
//         self.next_u32();
//         self.state = self.state.wrapping_add(initstate);
//         self.next_u32();
//     }

//     pub fn next_u32(&mut self) -> u32 {
//         let old_state = self.state;
//         self.state = (old_state.wrapping_mul(6364136223846793005u64)).wrapping_add(self.inc);
//         let xorshifted = ((old_state >> 18) ^ old_state) >> 27;
//         let rot = old_state >> 59;
//         let res = (xorshifted >> rot) | (xorshifted << ((!rot) & 31));
//         res as u32
//     }
// }

// mod test {
//     // use super::Pcg32;
//     #[allow(unused_imports)]
//     use super::*;

//     // #[test]
//     // fn test() {
//     //     let mut pcg = Pcg32::default();
//     //     for _ in 0..16 {
//     //         println!("t {}", pcg.next_u32());
//     //     }
//     // }

//     #[test]
//     fn xorshift32() {
//         let mut xsh = XorShift32::wrap(14);
//         assert_eq!(xsh.next_u32(), 0x39C1CE);
//     }

//     #[test]
//     fn splitmix64() {
//         let mut smx = SplitMix64::wrap(15);
//         assert_eq!(smx.mix(), 0x875B9307ABF55005);
//     }

//     #[test]
//     fn xoroshiro128plus() {
//         let mut xsh128p = XoRoShiRo128Plus::seed_using_splitmix(1617);
//         assert_eq!(xsh128p.next_u64(), 0x1066E27220122865);
//     }

//     #[test]
//     fn pcg32() {
//         let mut pcg = Pcg32::default();
//         let results = [
//             0x8A96A78D, 0x813E3C83, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
//         ];
//         for result in results.iter() {
//             assert_eq!(pcg.next_u32(), *result);
//         }
//     }
// }
