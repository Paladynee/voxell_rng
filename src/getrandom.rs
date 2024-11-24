/// This module provides a simple way to generate random numbers using the `getrandom` crate.
/// I will call this Magic since the entropy comes from the underlying OS in runtime.
pub struct MagicSeed;

macro_rules! impl_new_for_primitive_types {
    ($($t:ident),*) => {
        $(
            #[doc = r"Create a new "]
            #[doc = stringify!($t)]
            #[doc = r" magically using runtime entropy."]
            #[doc = r"# Errors"]
            #[doc = r"Returns an error if the underlying [`getrandom::getrandom`] call fails to generate random bytes."]
            #[inline]
            pub fn $t() -> Result<$t, getrandom::Error> {
                const L: usize= core::mem::size_of::<$t>();
                let mut bytes: [u8; L] = [0; L];
                getrandom::getrandom(&mut bytes)?;
                let value: $t = $t::from_ne_bytes(bytes);
                Ok(value)
            }
        )*
    };
}

impl MagicSeed {
    impl_new_for_primitive_types!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);
}
