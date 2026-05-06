macro_rules! generate_polyfill_fill_bytes {
    ($fn_name:ident, $ty:ty) => {
        #[doc = "A polyfill function that uses the `next_num_fn` to generate a value of"]
        #[doc = ::core::concat!("type ", ::core::stringify!($ty), " and uses iterator")]
        #[doc = "methods to fill the destination bytes. Use this with the widest number"]
        #[doc = "generation method of your RNG to implement the optimal `next_<primitive>`"]
        #[doc = "methods automatically for your RNG."]
        #[inline]
        pub fn $fn_name<SELF, FN>(mut next_num_fn: FN) -> impl FnMut(&mut SELF, &mut [u8])
        where
            FN: FnMut(&mut SELF) -> $ty,
        {
            move |this, dest| {
                const SIZE_BYTES: usize = ::core::mem::size_of::<$ty>();
                if dest.is_empty() {
                    return;
                }

                let mut byte_iterator = dest.chunks_exact_mut(SIZE_BYTES);

                for slice in byte_iterator.by_ref() {
                    let next: $ty = next_num_fn(this);
                    let next_bytes: [u8; SIZE_BYTES] = next.to_ne_bytes();
                    slice.copy_from_slice(&next_bytes[..]);
                }

                let remainder = byte_iterator.into_remainder();
                if !remainder.is_empty() {
                    let next: $ty = next_num_fn(this);
                    let next_bytes: [u8; SIZE_BYTES] = next.to_ne_bytes();
                    remainder.copy_from_slice(&next_bytes[..remainder.len()]);
                }
            }
        }
    };
}

generate_polyfill_fill_bytes!(polyfill_fill_bytes_u8, u8);
generate_polyfill_fill_bytes!(polyfill_fill_bytes_u16, u16);
generate_polyfill_fill_bytes!(polyfill_fill_bytes_u32, u32);
generate_polyfill_fill_bytes!(polyfill_fill_bytes_u64, u64);
generate_polyfill_fill_bytes!(polyfill_fill_bytes_u128, u128);
generate_polyfill_fill_bytes!(polyfill_fill_bytes_usize, usize);

/// A polyfill function that uses the `next_u32_fn` to generate the next
/// `f32` between 0 and 1.
#[inline]
pub const fn polyfill_next_f32_from_u32<SELF, FN>(mut next_u32_fn: FN) -> impl FnMut(&mut SELF) -> f32
where
    FN: FnMut(&mut SELF) -> u32,
{
    move |this| next_u32_fn(this) as f32 / u32::MAX as f32
}

/// A polyfill function that uses the `next_u64_fn` to generate the next
/// `f32` between 0 and 1.
#[inline]
pub const fn polyfill_next_f32_from_u64<SELF, FN>(mut next_u64_fn: FN) -> impl FnMut(&mut SELF) -> f32
where
    FN: FnMut(&mut SELF) -> u64,
{
    move |this| next_u64_fn(this) as f32 / u64::MAX as f32
}

/// A polyfill function that uses the `next_u32_fn` to generate the next
/// `f64` between 0 and 1. Always calls `next_u32_fn` twice.
#[inline]
pub const fn polyfill_next_f64_from_u32<SELF, FN>(mut next_u32_fn: FN) -> impl FnMut(&mut SELF) -> f64
where
    FN: FnMut(&mut SELF) -> u32,
{
    move |this| ((u64::from(next_u32_fn(this)) << 32) | u64::from(next_u32_fn(this))) as f64 / u32::MAX as f64
}

/// A polyfill function that uses the `next_u64_fn` to generate the next
/// `f64` between 0 and 1.
#[inline]
pub const fn polyfill_next_f64_from_u64<SELF, FN>(mut next_u64_fn: FN) -> impl FnMut(&mut SELF) -> f64
where
    FN: FnMut(&mut SELF) -> u64,
{
    move |this| next_u64_fn(this) as f64 / u64::MAX as f64
}

/// Generate polyfills for floating point number generations from `u32` and `u64` generators,
/// which are already available in `RngCore`.
///
/// Usage:
/// ```rust,norun
/// impl RngCore for MyRng { /* ... */ } // for next_u32 and next_u64
///
/// impl MyRng {
///     polyfill_next_f32_next_f64_from_fn!(
///         pub fn next_f32, next_f64(Self::next_u32 = u32);
///     );
/// }
/// ```
#[doc(hidden)]
#[macro_export]
macro_rules! polyfill_next_f32_next_f64_from_fn {
    ($vis:vis fn $fn_name_1:ident, $fn_name_2:ident($input_fn_name:path = u32);) => {
        /// Generate the next `f32`.
        #[inline]
        #[must_use = "please use the generated value"]
        $vis fn $fn_name_1(&mut self) -> f32 {
            $crate::rng::polyfill::polyfill_next_f32_from_u32($input_fn_name)(self)
        }

        /// Generate the next `f64`.
        #[inline]
        #[must_use = "please use the generated value"]
        $vis fn $fn_name_2(&mut self) -> f64 {
            $crate::rng::polyfill::polyfill_next_f64_from_u32($input_fn_name)(self)
        }
    };

    ($vis:vis fn $fn_name_1:ident, $fn_name_2:ident($input_fn_name:path = u64);) => {
        /// Generate the next `f32`.
        #[inline]
        #[must_use = "please use the generated value"]
        $vis fn $fn_name_1(&mut self) -> f32 {
            $crate::rng::polyfill::polyfill_next_f32_from_u64($input_fn_name)(self)
        }

        /// Generate the next `f64`.
        #[inline]
        #[must_use = "please use the generated value"]
        $vis fn $fn_name_2(&mut self) -> f64 {
            $crate::rng::polyfill::polyfill_next_f64_from_u64($input_fn_name)(self)
        }
    };
}

#[doc(inline)]
pub use crate::polyfill_next_f32_next_f64_from_fn;
