use crate::prelude::RngCoreExtension;
use core::array;

pub type GetRandomResult<T> = Result<T, getrandom::Error>;

/// trait for things that can be shuffled around
pub trait Shuffle {
    fn shuffle_with<R>(&mut self, rng: &mut R) -> &mut Self
    where
        R: RngCoreExtension + ?Sized;
}

impl<T> Shuffle for [T] {
    #[inline]
    fn shuffle_with<R>(&mut self, rng: &mut R) -> &mut Self
    where
        R: RngCoreExtension + ?Sized,
    {
        for i in (1..self.len()).rev() {
            let j = rng.next_usize() % (i + 1);
            self.swap(i, j);
        }

        self
    }
}

/// trait for configuring how to select
pub trait Selector<'rng, R: RngCoreExtension + ?Sized + 'rng, T> {
    /// The input type to be passed directly to [`Selector::run`].
    type Input<'a>
    where
        T: 'a;
    /// The output type to be returned directly to [`Selector::run`].
    type Output<'a>
    where
        T: 'a;

    /// Run the given configuration with the inputs as specified. This isn't intended for calling
    /// directly. Use [`select_random`] instead.
    ///
    /// # Errors
    ///
    /// May return an error if the selection fails at any point.
    fn run<'a>(self, input: Self::Input<'a>, rng: &'rng mut R) -> Self::Output<'a>;
}

/// Select a random item from a slice using the given configuration.
///
/// Possible values for generic parameter `SEL`:
/// - [`SelectOneConfig`]
/// - [`SelectOneMutConfig`]
/// - [`MultiSelectConfig`]
/// - [`MultiSelectConstConfig`]
/// - [`MultiSelectNonoverlappingConfig`]
/// - [`MultiSelectNonoverlappingConstConfig`]
/// - [`MultiSelectMutConfig`]
/// - [`MultiSelectMutConstConfig`]
//
// NB: investigate: why does unsize coercion (&[T; N] -> &[T]) fail when `input` arg is before `config` arg?
#[inline]
pub fn select_random<'s, 'rng, RNG, T, SEL>(config: SEL, input: SEL::Input<'s>, rng: &'rng mut RNG) -> SEL::Output<'s>
where
    RNG: RngCoreExtension + ?Sized + 'rng,
    SEL: Selector<'rng, RNG, T> + 's,
    T: 's,
{
    config.run(input, rng)
}

// pub trait SelectRandom<'s, 'rng, 'i, R, T, SEL>
// where
//     R: RngCoreExtension + ?Sized + 'rng,
//     SEL: Selector<'rng, R, T, Input<'i> = &'s Self> + 's,
//     T: 's,
//     's: 'i,
// {
//     fn select_random(&'s self, config: SEL, rng: &'rng mut R) -> SEL::Output<'s> {
//         config.run(self, rng)
//     }
// }

/// [`select_random`] trait method extension for slices.
///
/// In actuality, this trait will be implemented for the input of everything that implements
/// the [`Selector`] trait. So in the future, this function may also extend `Vec<T>` and
/// such with this method too.
///
/// Enables the very cursed `slice.select_random(conf, rng)` syntax where the type of the
/// slice is inferred from an argument to `select_random`. This is currently very
/// IDE-unfriendly but it compiles and works, and is the same with calling
/// `select_random(slice, conf, rng)`. Unforunately you currently can't call
/// this method on contemporary IDE's because in order to do method suggestions the type
/// must be known first, and this method relies on discovering the type from the argument.
///
pub trait SliceSelectRandomExt<'s, 'rng, SEL, R, T>
where
    R: RngCoreExtension + ?Sized + 'rng,
    SEL: Selector<'rng, R, T>,
{
    fn select_random(self, conf: SEL, rng: &'rng mut R) -> SEL::Output<'s>;
}

impl<'rng, 's, SEL, R, T> SliceSelectRandomExt<'s, 'rng, SEL, R, T> for SEL::Input<'s>
where
    R: RngCoreExtension + ?Sized + 'rng,
    SEL: Selector<'rng, R, T>,
    SEL::Input<'s>: 's,
{
    #[inline]
    fn select_random(self, conf: SEL, rng: &'rng mut R) -> SEL::Output<'s> {
        conf.run(self, rng)
    }
}

/// Configuration for selecting a single immutable item from a given slice.
///
/// #### Input: `&[T]`
/// #### Output: `Option<&T>`
///
/// - If the slice is empty, `None` is returned.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct SelectorOneImmut;

impl<'rng, R: RngCoreExtension + ?Sized + 'rng, T> Selector<'rng, R, T> for SelectorOneImmut {
    type Input<'s>
        = &'s [T]
    where
        T: 's;
    type Output<'s>
        = Option<&'s T>
    where
        T: 's;

    #[inline]
    fn run<'s>(self, input: Self::Input<'s>, rng: &'rng mut R) -> Self::Output<'s> {
        let len = input.len();
        if len == 0 {
            None
        } else {
            let idx = rng.next_usize() % input.len();
            input.get(idx)
        }
    }
}

/// Configuration for selecting a single mutable item from a given slice.
///
/// #### Input: `&mut [T]`
/// #### Output: `Option<&mut T>`
///
/// - If the slice is empty, `None` is returned.
pub struct SelectorOneMut;

impl<'rng, R: RngCoreExtension + ?Sized + 'rng, T> Selector<'rng, R, T> for SelectorOneMut {
    type Input<'s>
        = &'s mut [T]
    where
        T: 's;
    type Output<'s>
        = Option<&'s mut T>
    where
        T: 's;

    #[inline]
    fn run<'s>(self, input: Self::Input<'s>, rng: &'rng mut R) -> Self::Output<'s> {
        let len = input.len();
        if len == 0 {
            None
        } else {
            let idx = rng.next_usize() % len;
            input.get_mut(idx)
        }
    }
}

/// Configuration for selecting multiple maybe-overlapping immutable items from a given slice.
///
/// #### Input: `&[T]`
/// #### Output: `Vec<&T>`
///
/// - If the slice is empty, `vec![]` is returned.
pub struct MultiSelectorImmutOverlap(pub usize);

impl<'rng, R: RngCoreExtension + ?Sized + 'rng, T> Selector<'rng, R, T> for MultiSelectorImmutOverlap {
    type Input<'s>
        = &'s [T]
    where
        T: 's;
    type Output<'s>
        = Vec<&'s T>
    where
        T: 's;

    #[inline]
    fn run<'s>(self, input: Self::Input<'s>, rng: &'rng mut R) -> Self::Output<'s> {
        let len = input.len();
        let mut buf = vec![];
        if len == 0 || self.0 == 0 {
            return buf;
        }

        for _ in 0..self.0 {
            let idx = rng.next_usize() % len;
            buf.push(&input[idx]);
        }

        buf
    }
}

/// Configuration for selecting multiple maybe-overlapping immutable items from a given slice.
///
/// #### Input: `&[T]`
/// #### Output: `Option<[&T; N]>`
///
/// - If the slice is empty, `None` is returned.
pub struct MultiSelectorImmutOverlapArray<const N: usize>;

impl<'rng, R: RngCoreExtension + ?Sized + 'rng, T, const N: usize> Selector<'rng, R, T> for MultiSelectorImmutOverlapArray<N> {
    type Input<'s>
        = &'s [T]
    where
        T: 's;
    type Output<'s>
        = Option<[&'s T; N]>
    where
        T: 's;

    #[inline]
    fn run<'s>(self, input: Self::Input<'s>, rng: &'rng mut R) -> Self::Output<'s> {
        let len = input.len();

        if len == 0 {
            return None;
        }

        // TODO: bench this vs &input[0] for initial values of the buf
        let mut buf: [&T; N] = array::from_fn(|_| &input[0]);
        for val in buf.iter_mut().take(N) {
            let idx = rng.next_usize() % len;
            *val = &input[idx];
        }

        Some(buf)
    }
}

/// Configuration for selecting multiple non-overlapping immutable items from a given slice.
///
/// #### Input: `&[T]`
/// #### Output: `Vec<&T>`
///
/// - If the slice is empty, `k == 0`, or if the requested size `k` is bigger than the slice's length,
///   `vec![]` is returned.
pub struct MultiSelectorImmutNonoverlap(pub usize);

impl<'rng, R: RngCoreExtension + ?Sized + 'rng, T> Selector<'rng, R, T> for MultiSelectorImmutNonoverlap {
    type Input<'s>
        = &'s [T]
    where
        T: 's;
    type Output<'s>
        = Vec<&'s T>
    where
        T: 's;

    #[inline]
    fn run<'s>(self, input: Self::Input<'s>, rng: &'rng mut R) -> Self::Output<'s> {
        let len = input.len();
        let mut reservoir = Vec::new();
        let k = self.0;

        if len == 0 || k == 0 || k > len {
            // cannot select more items than available non-overlapping
            // API design: nonpanicking, return empty vec instead.
            return reservoir;
        }

        reservoir.reserve_exact(k);

        let mut iter = input.iter();
        for _ in 0..k {
            reservoir.push(iter.next().unwrap());
        }

        for (i, elem) in iter.enumerate() {
            let j = rng.next_usize() % (k + i + 1);
            if let Some(slot) = reservoir.get_mut(j) {
                *slot = elem;
            }
        }

        reservoir
    }
}

/// Configuration for selecting multiple non-overlapping immutable items from a given slice.
///
/// #### Input: `&[T]`
/// #### Output: `Option<[&T; N]>`
///
/// - If the slice is empty, or if `N` is greater than the slice's length, `None` is returned.
pub struct MultiSelectorImmutNonoverlapArray<const N: usize>;

impl<'rng, R: RngCoreExtension + ?Sized + 'rng, T, const N: usize> Selector<'rng, R, T> for MultiSelectorImmutNonoverlapArray<N> {
    type Input<'s>
        = &'s [T]
    where
        T: 's;
    type Output<'s>
        = Option<[&'s T; N]>
    where
        T: 's;

    #[inline]
    fn run<'s>(self, input: Self::Input<'s>, rng: &'rng mut R) -> Self::Output<'s> {
        let len = input.len();

        if len == 0 && N != 0 {
            return None;
        }

        if len < N {
            return None;
        }

        let mut iter = input.iter();
        let mut reservoir = array::from_fn::<_, N, _>(|_| iter.next().unwrap());

        if N != 0 {
            for (i, elem) in iter.enumerate() {
                let k = rng.next_usize() % (N + i + 1);
                if let Some(slot) = reservoir.get_mut(k) {
                    *slot = elem;
                }
            }
        }

        Some(reservoir)
    }
}

/// Configuration for selecting multiple non-overlapping mutable items from a given slice.
///
/// #### Input: `&mut [T]`
/// #### Output: `Vec<&mut T>`
///
/// - If the slice is empty, `k == 0`, or if the requested size `k` is bigger than the slice's length,
///   `vec![]` is returned.
pub struct MultiSelectorMutNonoverlap(pub usize);

impl<'rng, R: RngCoreExtension + ?Sized + 'rng, T> Selector<'rng, R, T> for MultiSelectorMutNonoverlap {
    type Input<'s>
        = &'s mut [T]
    where
        T: 's;
    type Output<'s>
        = Vec<&'s mut T>
    where
        T: 's;

    #[inline]
    fn run<'s>(self, input: Self::Input<'s>, rng: &'rng mut R) -> Self::Output<'s> {
        let len = input.len();
        let mut reservoir = Vec::new();
        let k = self.0;

        if len == 0 || k == 0 || k > len {
            // cannot select more items than available non-overlapping
            // API design: nonpanicking, return empty vec instead.
            return reservoir;
        }

        reservoir.reserve_exact(k);

        let mut iter = input.iter_mut();
        for _ in 0..k {
            reservoir.push(iter.next().unwrap());
        }

        for (i, elem) in iter.enumerate() {
            let j = rng.next_usize() % (k + i + 1);
            if let Some(slot) = reservoir.get_mut(j) {
                *slot = elem;
            }
        }

        reservoir
    }
}

/// Configuration for selecting multiple non-overlapping mutable items from a given slice.
///
/// #### Input: `&mut [T]`
/// #### Output: `Option<[&mut T; N]>`
///
/// - If `N == 0`, the slice is empty, or if `N` is greater than the slice's length, `None` is returned.
pub struct MultiSelectorMutNonoverlapArray<const N: usize>;

impl<'rng, R: RngCoreExtension + ?Sized + 'rng, T, const N: usize> Selector<'rng, R, T> for MultiSelectorMutNonoverlapArray<N> {
    type Input<'s>
        = &'s mut [T]
    where
        T: 's;
    type Output<'s>
        = Option<[&'s mut T; N]>
    where
        T: 's;

    #[inline]
    fn run<'s>(self, input: Self::Input<'s>, rng: &'rng mut R) -> Self::Output<'s> {
        let len = input.len();

        if len == 0 && N != 0 {
            return None;
        }

        if len < N {
            return None;
        }

        let mut iter = input.iter_mut();
        let mut reservoir = array::from_fn::<_, N, _>(|_| iter.next().unwrap());

        if N != 0 {
            for (i, elem) in iter.enumerate() {
                let k = rng.next_usize() % (N + i + 1);
                if let Some(slot) = reservoir.get_mut(k) {
                    *slot = elem;
                }
            }
        }

        Some(reservoir)
    }
}

#[test]
fn xd() {
    use crate::rng::XorShift128;
    let mut rng = XorShift128::default();
    let data = vec![69, 420, 1337, 1234, 5678, 9012, 3456, 7890, 360, 420, 6934343];
    let mut out = vec![];
    for _ in 0..5 {
        let rand = (&data[..]).select_random(SelectorOneImmut, &mut rng).unwrap();
        out.push(*rand);
    }

    let out2: [&i32; 5] = (&data[..]).select_random(MultiSelectorImmutOverlapArray::<5>, &mut rng).unwrap();
    println!("{:?}", out);
    println!("{:?}", out2);
}

#[test]
fn msioa() {
    use crate::rng::XorShift128;
    let mut rng = XorShift128::default();
    let mut slice: Vec<u8> = (0..1024).map(|a| a as u8).collect::<Vec<_>>();
    let addr = slice.as_ptr().addr();
    macro_rules! get_and_print {
        ($sel:expr, mut) => {{
            let values = (&mut slice[..]).select_random($sel, &mut rng);
            let values = values.into_iter().map(|a| (a as *mut u8).addr() - addr).collect::<Vec<_>>();
            println!(concat!("Values for ", stringify!($sel), " selector is:\n\t{:?}"), values)
        }};
        ($sel:expr, opt, mut) => {{
            let values = (&mut slice[..]).select_random($sel, &mut rng).unwrap();
            let values = values.map(|a| (a as *mut u8).addr() - addr);
            println!(concat!("Values for ", stringify!($sel), " selector is:\n\t{:?}"), values)
        }};
        ($sel:expr, opt) => {{
            let values = (&slice[..]).select_random($sel, &mut rng).unwrap();
            let values = values.map(|a| (a as *const u8).addr() - addr);
            println!(concat!("Values for ", stringify!($sel), " selector is:\n\t{:?}"), values)
        }};
        ($sel:expr) => {{
            let values = (&slice[..]).select_random($sel, &mut rng);
            let values = values.into_iter().map(|a| (a as *const u8).addr() - addr).collect::<Vec<_>>();
            println!(concat!("Values for ", stringify!($sel), " selector is:\n\t{:?}"), values)
        }};
    }

    get_and_print!(SelectorOneImmut);
    get_and_print!(SelectorOneMut, mut);

    get_and_print!(MultiSelectorImmutOverlap(0));
    get_and_print!(MultiSelectorImmutOverlap(1));
    get_and_print!(MultiSelectorImmutOverlap(10));

    get_and_print!(MultiSelectorImmutNonoverlap(0));
    get_and_print!(MultiSelectorImmutNonoverlap(1));
    get_and_print!(MultiSelectorImmutNonoverlap(10));

    get_and_print!(MultiSelectorImmutNonoverlapArray::<0>, opt);
    get_and_print!(MultiSelectorImmutNonoverlapArray::<1>, opt);
    get_and_print!(MultiSelectorImmutNonoverlapArray::<10>, opt);

    get_and_print!(MultiSelectorImmutOverlapArray::<0>, opt);
    get_and_print!(MultiSelectorImmutOverlapArray::<1>, opt);
    get_and_print!(MultiSelectorImmutOverlapArray::<10>, opt);

    get_and_print!(MultiSelectorMutNonoverlap(0), mut);
    get_and_print!(MultiSelectorMutNonoverlap(1), mut);
    get_and_print!(MultiSelectorMutNonoverlap(10), mut);

    get_and_print!(MultiSelectorMutNonoverlapArray::<0>, opt, mut);
    get_and_print!(MultiSelectorMutNonoverlapArray::<1>, opt, mut);
    get_and_print!(MultiSelectorMutNonoverlapArray::<10>, opt, mut);

    get_and_print!(MultiSelectorMutNonoverlapArray::<1000>, opt, mut);
    get_and_print!(MultiSelectorImmutOverlapArray::<1000>, opt);
}
