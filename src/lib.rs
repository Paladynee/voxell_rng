#![doc = include_str!("../README.md")]
#![warn(missing_docs)]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;

extern crate getrandom;
extern crate preinterpret;
extern crate rand_core;

/// RNG branching mechanics for parallel random numbers
pub mod branch_rng;

/// Seed RNGs in runtime using OS entropy
pub mod genrandom;

/// RNG engines
pub mod rng;

/// Methods on slices that require randomness
pub mod slice_methods;

/// Seed RNGs using the system time
#[cfg(feature = "std")]
pub mod time_seeded;

/// prelude
pub mod prelude;

/// extension methods for [`RngCore`]
///
/// covers all the basic integer types
pub mod rng_core_extension;
