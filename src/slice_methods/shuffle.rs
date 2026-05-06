use crate::prelude::RngCoreExtension;

/// trait for things that can be shuffled around
pub trait Shuffle {
    /// Shuffle the given slice using the given RNG. After this operation,
    /// nothing can be said about the ordering of items inside the slice.
    /// 
    /// This will never panic.
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
