#[doc = include_str!("../README.md")]
#[warn(missing_docs)]
/// RNG branching mechanics for parallel random numbers
pub mod branch_rng;

/// Seed RNGs in runtime using OS entropy
pub mod getrandom;

/// RNG engines
pub mod rng;

/// Methods on slices that require randomness
pub mod slice_methods;

/// Seed RNGs using the system time
pub mod time_seeded;

/// prelude
pub mod prelude;

/// extension methods for [`RngCore`]
///
/// covers all the basic integer types
pub mod rng_core_extension;
