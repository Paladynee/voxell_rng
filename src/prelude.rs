pub use crate::branch_rng::BranchRng;
// we don't prelude `RngCore` so our `next_u32` and `next_u64` methods
// don't clash with the respective `RngCore` methods.
pub use crate::rng_core_extension::RngCoreExtension;
pub use crate::slice_methods::Shuffle;
