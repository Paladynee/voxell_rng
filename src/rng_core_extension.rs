use crate::rng::{Pcg128, Pcg16, Pcg32, Pcg64, Pcg8, SplitMix64, XoRoShiRo128, XorShift128, XorShift32};
use core::mem;
use rand_core::RngCore;

/// Extends `RngCore` to support all primitive integer types.
pub trait RngCoreExtension: RngCore {
    /// next u8 element from the rng.
    #[must_use = "please use the generated value"]
    fn next_u8(&mut self) -> u8;
    /// next u16 element from the rng.
    #[must_use = "please use the generated value"]
    fn next_u16(&mut self) -> u16;
    /// next u32 element from the rng.
    #[must_use = "please use the generated value"]
    fn next_u32(&mut self) -> u32;
    /// next u64 element from the rng.
    #[must_use = "please use the generated value"]
    fn next_u64(&mut self) -> u64;
    /// next u128 element from the rng.
    #[must_use = "please use the generated value"]
    fn next_u128(&mut self) -> u128;
    /// next usize element from the rng.
    #[must_use = "please use the generated value"]
    fn next_usize(&mut self) -> usize;

    /// next u8 element from the rng.
    #[must_use = "please use the generated value"]
    fn next_i8(&mut self) -> i8;
    /// next u16 element from the rng.
    #[must_use = "please use the generated value"]
    fn next_i16(&mut self) -> i16;
    /// next u32 element from the rng.
    #[must_use = "please use the generated value"]
    fn next_i32(&mut self) -> i32;
    /// next u64 element from the rng.
    #[must_use = "please use the generated value"]
    fn next_i64(&mut self) -> i64;
    /// next u128 element from the rng.
    #[must_use = "please use the generated value"]
    fn next_i128(&mut self) -> i128;
    /// next usize element from the rng.
    #[must_use = "please use the generated value"]
    fn next_isize(&mut self) -> isize;
}

macro_rules! fill_native_endian_bytes {
    ($ty:ty, $bytes:expr, $rng:ident) => {
        #[allow(unused_braces)]
        {
            let mut bytes: [u8; $bytes] = [0; $bytes];
            <Self as RngCore>::fill_bytes($rng, &mut bytes);
            <$ty>::from_ne_bytes(bytes)
        }
    };
}

macro_rules! gen_next_prim_function {
    ($fn_name:ident, $final_type:ty, $byte_width:expr) => {
        #[inline]
        fn $fn_name(&mut self) -> $final_type {
            fill_native_endian_bytes!($final_type, $byte_width, self)
        }
    };
}

macro_rules! extend_rngcore_for {
    ($ty:ty) => {
        impl RngCoreExtension for $ty {
            gen_next_prim_function!(next_u8, u8, 1);
            gen_next_prim_function!(next_u16, u16, 2);
            gen_next_prim_function!(next_u32, u32, 4);
            gen_next_prim_function!(next_u64, u64, 8);
            gen_next_prim_function!(next_u128, u128, 16);
            gen_next_prim_function!(next_usize, usize, { mem::size_of::<usize>() });
            gen_next_prim_function!(next_i8, i8, 1);
            gen_next_prim_function!(next_i16, i16, 2);
            gen_next_prim_function!(next_i32, i32, 4);
            gen_next_prim_function!(next_i64, i64, 8);
            gen_next_prim_function!(next_i128, i128, 16);
            gen_next_prim_function!(next_isize, isize, { mem::size_of::<isize>() });
        }
    };
}

extend_rngcore_for!(XorShift128);
extend_rngcore_for!(XorShift32);
extend_rngcore_for!(Pcg128);
extend_rngcore_for!(Pcg64);
extend_rngcore_for!(Pcg32);
extend_rngcore_for!(Pcg16);
extend_rngcore_for!(Pcg8);
extend_rngcore_for!(SplitMix64);
extend_rngcore_for!(XoRoShiRo128);
