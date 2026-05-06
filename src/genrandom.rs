//! This module provides a simple way to generate random numbers using the `getrandom` crate.

/// A trait for types that can be magically seeded using runtime entropy.
pub trait GenRandom: Sized {
    /// Create a new instance of the type using OS entropy.
    ///
    /// Invoking this method every time you're generating a number is not a great idea for performance.
    ///
    /// # Errors
    ///
    /// Returns an error if the underlying [`getrandom`](getrandom::getrandom) call fails to generate random bytes.
    fn get_random() -> Result<Self, getrandom::Error>;
}

macro_rules! impl_trait_for_primitive_types {
    ($($current_type:ty),*) => {
        preinterpret::preinterpret! {
            $(
                [!set! #current_type = [!ident! $current_type]]

                impl GenRandom for $current_type {
                    #[doc = [!string! "Create a new [`" #current_type "`] using OS entropy.\n\n"]]
                    #[doc = "Invoking this method every time you're generating a number is not a great idea for performance.\n\n"]
                    #[doc = "# Errors\n\n"]
                    #[doc = "Returns an error if the underlying [`getrandom`](getrandom::getrandom) call fails to generate random bytes."]
                    #[inline]
                    fn get_random() -> Result<$current_type, getrandom::Error> {
                        const L: usize = core::mem::size_of::<$current_type>();
                        let mut bytes: [u8; L] = [0; L];
                        getrandom::fill(&mut bytes)?;
                        let value: $current_type = $current_type::from_ne_bytes(bytes);
                        Ok(value)
                    }
                    }
            )*
        }
    };
}

macro_rules! impl_trait_for_floats {
    ($($current_type:ty: $bittype:ty),*) => {
        preinterpret::preinterpret! {
            $(
                [!set! #current_type = [!ident! $current_type]]

                impl GenRandom for $current_type {
                    #[doc = [!string! "Create a new [`" #current_type "`] using OS entropy.\n\n"]]
                    #[doc = "Invoking this method every time you're generating a number is not a great idea for performance.\n\n"]
                    #[doc = "The resulting number will be between 0 and 1.\n\n"]
                    #[doc = "# Errors\n\n"]
                    #[doc = "Returns an error if the underlying [`getrandom`](getrandom::getrandom) call fails to generate random bytes."]
                    #[inline]
                    fn get_random() -> Result<$current_type, getrandom::Error> {
                        const L: usize = core::mem::size_of::< $current_type >();
                        let mut bytes: [u8; L] = [0; L];
                        getrandom::fill(&mut bytes)?;
                        let value: $bittype = $bittype::from_ne_bytes(bytes);
                        let fvalue = value as $current_type / $bittype::MAX as $current_type;
                        Ok(fvalue)
                    }
                    }
            )*
        }
    };
}

impl_trait_for_primitive_types!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);
impl_trait_for_floats!(f32: u32, f64: u64);
