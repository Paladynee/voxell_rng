//! Thread local random number generators (`voxell_rng::rng`), optionally
//! seeded with the system time (`voxell_rng::time_seeded_rng`).
//!
//! If you need to share RNGs between threads, you should use the `BranchRng` trait
//! to branch RNGs.
//!
//! This trait is implemented for all RNGs in this crate.
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

    /// pcg library for n00bz
    pub mod pcg_easy {
        pub use super::pcg_128::Pcg128;
        pub use super::pcg_16::Pcg16;
        pub use super::pcg_32::Pcg32;
        pub use super::pcg_64::Pcg64;
        pub use super::pcg_8::Pcg8;
    }

    /// pcg library for hax0rs
    #[expect(missing_docs)]
    pub mod pcg_advanced {
        pub mod pcg_16 {
            pub use super::super::pcg_16::*;
        }
        pub mod pcg_32 {
            pub use super::super::pcg_32::*;
        }
        pub mod pcg_64 {
            pub use super::super::pcg_64::*;
        }
        pub mod pcg_8 {
            pub use super::super::pcg_8::*;
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
/// Seed RNGs with the system time
pub mod time_seeded_rng;
