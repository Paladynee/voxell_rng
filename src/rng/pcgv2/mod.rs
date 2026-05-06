use core::fmt::Debug;
use core::hash::Hash;
use core::marker::PhantomData;
use core::ops::{
    Add, AddAssign, BitAnd, BitAndAssign, BitXor, BitXorAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub,
    SubAssign,
};

pub struct Pcg<B: PcgSupportedBits> {
    state: B,
}

pub trait ZeroAndOne: Copy {
    const ZERO: Self;
    const ONE: Self;
}

pub trait AsCast<T> {
    fn as_cast(self) -> T;
}

pub trait PcgSupportedBits:
    Copy
    + Clone
    + Debug
    + Hash
    + PartialEq
    + Eq
    + PartialOrd
    + Ord
    + Add<Self, Output = Self>
    + AddAssign<Self>
    + Sub<Self, Output = Self>
    + SubAssign<Self>
    + Mul<Self, Output = Self>
    + MulAssign<Self>
    + Div<Self, Output = Self>
    + DivAssign<Self>
    + Rem<Self, Output = Self>
    + RemAssign<Self>
    + Shl<Self, Output = Self>
    + ShlAssign<Self>
    + Shr<Self, Output = Self>
    + ShrAssign<Self>
    + BitXor<Self, Output = Self>
    + BitXorAssign<Self>
    + BitAnd<Self, Output = Self>
    + BitAndAssign<Self>
    + ZeroAndOne
{
}

macro_rules! imp {
    ($($ty:ty),* $(,)?) => {
        $(
            impl ZeroAndOne for $ty {
                const ZERO: Self = 0;
                const ONE: Self = 1;
            }

            impl PcgSupportedBits for $ty {}
        )*
    };
}

macro_rules! imp_ascast {
    ($($lhs:ty as $rhs:ty),* $(,)?) => {
        $(
            impl AsCast<$rhs> for $lhs {
                #[allow(clippy::cast_possible_wrap)]
                #[allow(clippy::cast_sign_loss)]
                #[inline(always)]
                fn as_cast(self) -> $rhs {
                    self as $rhs
                }
            }
        )*
    };
}

impl<B: PcgSupportedBits> Pcg<B> {}

imp!(u8, u16, u32, u64, u128, usize);

#[rustfmt::skip]
imp_ascast!(
    u8 as u8, u8 as u16, u8 as u32, u8 as u64, u8 as u128, u8 as usize,
    u8 as i8, u8 as i16, u8 as i32, u8 as i64, u8 as i128, u8 as isize,
    u16 as u8, u16 as u16, u16 as u32, u16 as u64, u16 as u128, u16 as usize,
    u16 as i8, u16 as i16, u16 as i32, u16 as i64, u16 as i128, u16 as isize,
    u32 as u8, u32 as u16, u32 as u32, u32 as u64, u32 as u128, u32 as usize,
    u32 as i8, u32 as i16, u32 as i32, u32 as i64, u32 as i128, u32 as isize,
    u64 as u8, u64 as u16, u64 as u32, u64 as u64, u64 as u128, u64 as usize,
    u64 as i8, u64 as i16, u64 as i32, u64 as i64, u64 as i128, u64 as isize,
    u128 as u8, u128 as u16, u128 as u32, u128 as u64, u128 as u128, u128 as usize,
    u128 as i8, u128 as i16, u128 as i32, u128 as i64, u128 as i128, u128 as isize,
    usize as u8, usize as u16, usize as u32, usize as u64, usize as u128, usize as usize,
    usize as i8, usize as i16, usize as i32, usize as i64, usize as i128, usize as isize,
    i8 as u8, i8 as u16, i8 as u32, i8 as u64, i8 as u128, i8 as usize,
    i8 as i8, i8 as i16, i8 as i32, i8 as i64, i8 as i128, i8 as isize,
    i16 as u8, i16 as u16, i16 as u32, i16 as u64, i16 as u128, i16 as usize,
    i16 as i8, i16 as i16, i16 as i32, i16 as i64, i16 as i128, i16 as isize,
    i32 as u8, i32 as u16, i32 as u32, i32 as u64, i32 as u128, i32 as usize,
    i32 as i8, i32 as i16, i32 as i32, i32 as i64, i32 as i128, i32 as isize,
    i64 as u8, i64 as u16, i64 as u32, i64 as u64, i64 as u128, i64 as usize,
    i64 as i8, i64 as i16, i64 as i32, i64 as i64, i64 as i128, i64 as isize,
    i128 as u8, i128 as u16, i128 as u32, i128 as u64, i128 as u128, i128 as usize,
    i128 as i8, i128 as i16, i128 as i32, i128 as i64, i128 as i128, i128 as isize,
    isize as u8, isize as u16, isize as u32, isize as u64, isize as u128, isize as usize,
    isize as i8, isize as i16, isize as i32, isize as i64, isize as i128, isize as isize,
);

fn test() {
    // let mut pcg = Pcg::<u64>::new();
    // let n = pcg.advance::<OneSeq, RxsMXs>();
    // let f = pcg.get_advance(OneSeq, RxsMXs);
    // let n2 = f(&mut pcg);
}
