//! Thread local random number generators (`voxell_rng::rng`), optionally
//! seeded with the system time (`voxell_rng::time_seeded_rng`).
//!
//! If you need to share RNGs between threads, you should use the `BranchRng` trait
//! to branch RNGs.
//!
//! This trait is implemented for all RNGs in this crate.

pub mod branch_rng;
pub mod rng;
pub mod time_seeded_rng;
