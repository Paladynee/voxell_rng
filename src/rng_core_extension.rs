use crate::rng::{Pcg128, Pcg16, Pcg32, Pcg64, Pcg8, SplitMix64, XoRoShiRo128, XorShift128, XorShift32};
use core::mem;
use rand_core::RngCore;

/// Extends `RngCore` to support all primitive integer types.
pub trait RngCoreExtension: RngCore {
    /// next u8 element from the rng.
    fn next_u8(&mut self) -> u8;
    /// next u16 element from the rng.
    fn next_u16(&mut self) -> u16;
    /// next u32 element from the rng.
    fn next_u32(&mut self) -> u32;
    /// next u64 element from the rng.
    fn next_u64(&mut self) -> u64;
    /// next u128 element from the rng.
    fn next_u128(&mut self) -> u128;
    /// next usize element from the rng.
    fn next_usize(&mut self) -> usize;

    /// next u8 element from the rng.
    fn next_i8(&mut self) -> i8;
    /// next u16 element from the rng.
    fn next_i16(&mut self) -> i16;
    /// next u32 element from the rng.
    fn next_i32(&mut self) -> i32;
    /// next u64 element from the rng.
    fn next_i64(&mut self) -> i64;
    /// next u128 element from the rng.
    fn next_i128(&mut self) -> i128;
    /// next usize element from the rng.
    fn next_isize(&mut self) -> isize;
}

macro_rules! fill_native_endian_bytes {
    ($ty:ty, $bytes:tt, $rng:tt) => {
        #[allow(unused_braces)]
        {
            let mut bytes: [u8; $bytes] = [0; $bytes];
            $rng.fill_bytes(&mut bytes);
            <$ty>::from_ne_bytes(bytes)
        }
    };
}

macro_rules! impl_for_rng {
    ($ty:ty) => {
        impl RngCoreExtension for $ty {
            #[inline]
            fn next_u8(&mut self) -> u8 {
                fill_native_endian_bytes!(u8, 1, self)
            }

            #[inline]
            fn next_u16(&mut self) -> u16 {
                fill_native_endian_bytes!(u16, 2, self)
            }

            #[inline]
            fn next_u32(&mut self) -> u32 {
                fill_native_endian_bytes!(u32, 4, self)
            }

            #[inline]
            fn next_u64(&mut self) -> u64 {
                fill_native_endian_bytes!(u64, 8, self)
            }

            #[inline]
            fn next_u128(&mut self) -> u128 {
                fill_native_endian_bytes!(u128, 16, self)
            }

            #[inline]
            fn next_usize(&mut self) -> usize {
                fill_native_endian_bytes!(usize, { mem::size_of::<usize>() }, self)
            }

            #[inline]
            fn next_i8(&mut self) -> i8 {
                fill_native_endian_bytes!(i8, 1, self)
            }

            #[inline]
            fn next_i16(&mut self) -> i16 {
                fill_native_endian_bytes!(i16, 2, self)
            }

            #[inline]
            fn next_i32(&mut self) -> i32 {
                fill_native_endian_bytes!(i32, 4, self)
            }

            #[inline]
            fn next_i64(&mut self) -> i64 {
                fill_native_endian_bytes!(i64, 8, self)
            }

            #[inline]
            fn next_i128(&mut self) -> i128 {
                fill_native_endian_bytes!(i128, 16, self)
            }

            #[inline]
            fn next_isize(&mut self) -> isize {
                fill_native_endian_bytes!(isize, { mem::size_of::<isize>() }, self)
            }
        }
    };
}

impl_for_rng!(XorShift128);
impl_for_rng!(XorShift32);
impl_for_rng!(Pcg128);
impl_for_rng!(Pcg64);
impl_for_rng!(Pcg32);
impl_for_rng!(Pcg16);
impl_for_rng!(Pcg8);
impl_for_rng!(SplitMix64);
impl_for_rng!(XoRoShiRo128);
