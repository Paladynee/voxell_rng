#![doc = include_str!("../README.md")]
#![warn(missing_docs)]

/// RNG branching mechanics for parallel random numbers
pub mod branch_rng;
/// RNG engines
pub mod rng {
    mod pcg_128;
    mod pcg_16;
    mod pcg_32;
    mod pcg_64;
    mod pcg_8;

    pub use pcg_128::Pcg128;
    pub use pcg_16::Pcg16;
    pub use pcg_32::Pcg32;
    pub use pcg_64::Pcg64;
    pub use pcg_8::Pcg8;

    /// pcg library for hardc0re hax0rs
    ///
    /// available in 8, 16, 32, 64, and 128 bit variants
    ///
    /// available in oneseq, unique, setseq, and mcg variants
    ///
    /// output available in many "permuted functions on tuples" variants (`xsh_rs`, `xsh_rr`, `rxsh_rs`, `rxsh_rr`, etc)
    #[expect(missing_docs)]
    pub mod pcg_advanced {
        pub mod pcg_8 {
            pub use super::super::pcg_8::*;
        }
        pub mod pcg_16 {
            pub use super::super::pcg_16::*;
        }
        pub mod pcg_32 {
            pub use super::super::pcg_32::*;
        }
        pub mod pcg_64 {
            pub use super::super::pcg_64::*;
        }
        pub mod pcg_128 {
            pub use super::super::pcg_128::*;
        }
    }

    mod splitmix64;
    mod xoroshiro128plus;
    mod xorshift32;

    pub use splitmix64::SplitMix64;
    pub use xoroshiro128plus::XoRoShiRo128Plus;
    pub use xorshift32::XorShift32;
}
/// Seed RNGs in runtime using dark arts
pub mod runtime_seeded;
/// Seed RNGs using the system time
pub mod time_seeded;
