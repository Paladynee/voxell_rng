use core::array;

use crate::{rng::pcg_advanced::pcg_64::PcgInnerState64, runtime_seeded::MagicSeed};

/// trait for handling the shuffle method
pub trait Shuffle {
    /// shuffles a given slice in-place.
    ///
    /// # Errors
    ///
    /// returns an error if the system's random number generator fails
    fn shuffle(&mut self) -> Result<(), getrandom::Error>;
}

impl<T> Shuffle for [T] {
    #[expect(clippy::cast_possible_truncation)]
    #[inline]
    fn shuffle(&mut self) -> Result<(), getrandom::Error> {
        let seed = MagicSeed::new_magic()?;
        let mut rng = PcgInnerState64::oneseq_seeded(seed as u64);

        for i in (1..self.len()).rev() {
            let j = rng.unique_rxs_m_xs_bounded(i as u64) as usize;
            self.swap(i, j);
        }

        Ok(())
    }
}

/// trait for selecting random elements from a slice
pub trait SelectRandom {
    /// type that will be returned
    type Item;

    /// selects a random element from the slice
    ///
    /// returns `None` if the slice is empty
    ///
    /// # Errors
    ///
    /// returns an error if the system's random number generator fails
    fn select_random(&self) -> Result<Option<&Self::Item>, getrandom::Error>;

    /// selects a random element from the slice and returns a mutable reference to it
    ///
    /// returns `None` if the slice is empty
    ///
    /// # Errors
    ///
    /// returns an error if the system's random number generator fails
    fn select_random_mut(&mut self) -> Result<Option<&mut Self::Item>, getrandom::Error>;

    /// selects n random elements from the slice
    ///
    /// returns `None` if the slice is empty
    ///
    /// the references may overlap
    ///
    /// # Errors
    ///
    /// returns an error if the system's random number generator fails
    fn select_random_multiple_overlapping(
        &self,
        n: usize,
    ) -> Result<Option<Vec<&Self::Item>>, getrandom::Error>;

    /// selects n random elements from the slice, without using a runtime parameter
    ///
    /// the references may overlap
    ///
    /// # Errors
    ///
    /// returns an error if the system's random number generator fails
    fn select_random_multiple_overlapping_const<const N: usize>(
        &self,
    ) -> Result<Option<[&Self::Item; N]>, getrandom::Error>;

    /// selects n random elements from the slice, without using a runtime parameter
    ///
    /// returns `None` if the slice is empty
    ///
    /// the references will not overlap
    ///
    /// # Errors
    ///
    /// returns an error if the system's random number generator fails
    fn select_random_multiple_nonoverlapping(
        &self,
        n: usize,
    ) -> Result<Option<Vec<&Self::Item>>, getrandom::Error>;

    /// selects n random elements from the slice, without using a runtime
    ///
    /// the references will not overlap
    ///
    /// # Errors
    ///
    /// returns an error if the system's random number generator fails
    fn select_random_multiple_nonoverlapping_const<const N: usize>(
        &self,
    ) -> Result<Option<[&Self::Item; N]>, getrandom::Error>;

    /// selects n random elements from the slice and returns mutable references to them
    ///
    /// returns `None` if the slice is empty
    ///
    /// the references can not overlap; see rust's borrowing rules
    ///
    /// # Errors
    ///
    /// returns an error if the system's random number generator fails
    fn select_random_multiple_mut(
        &mut self,
        n: usize,
    ) -> Result<Option<Vec<&mut Self::Item>>, getrandom::Error>;

    /// selects n random elements from the slice and returns mutable references to them, without using a runtime parameter
    ///
    /// the references can not overlap; see rust's borrowing rules
    ///
    /// # Errors
    ///
    /// returns an error if the system's random number generator fails
    fn select_random_multiple_mut_const<const N: usize>(
        &mut self,
    ) -> Result<Option<[&mut Self::Item; N]>, getrandom::Error>;
}

impl<T> SelectRandom for [T] {
    type Item = T;

    #[inline]
    #[expect(clippy::cast_possible_truncation)]
    fn select_random(&self) -> Result<Option<&Self::Item>, getrandom::Error> {
        if self.is_empty() {
            return Ok(None);
        }
        let seed = MagicSeed::new_magic()?;
        let mut rng = PcgInnerState64::oneseq_seeded(seed as u64);
        let i = rng.unique_rxs_m_xs_bounded(self.len() as u64) as usize;

        Ok(Some(&self[i]))
    }

    #[inline]
    #[expect(clippy::cast_possible_truncation)]
    fn select_random_mut(&mut self) -> Result<Option<&mut Self::Item>, getrandom::Error> {
        if self.is_empty() {
            return Ok(None);
        }

        let seed = MagicSeed::new_magic()?;
        let mut rng = PcgInnerState64::oneseq_seeded(seed as u64);
        let i = rng.unique_rxs_m_xs_bounded(self.len() as u64) as usize;

        Ok(Some(&mut self[i]))
    }

    #[inline]
    #[expect(clippy::cast_possible_truncation)]
    fn select_random_multiple_overlapping(
        &self,
        n: usize,
    ) -> Result<Option<Vec<&Self::Item>>, getrandom::Error> {
        if self.is_empty() {
            return Ok(None);
        }

        let seed = MagicSeed::new_magic()?;
        let mut rng = PcgInnerState64::oneseq_seeded(seed as u64);

        let mut refs = Vec::with_capacity(n);
        for _ in 0..n {
            let i = rng.unique_rxs_m_xs_bounded(self.len() as u64) as usize;
            refs.push(&self[i]);
        }

        Ok(Some(refs))
    }

    #[inline]
    fn select_random_multiple_mut(
        &mut self,
        n: usize,
    ) -> Result<Option<Vec<&mut Self::Item>>, getrandom::Error> {
        if n <= self.len() {
            return Ok(None);
        }

        if self.is_empty() {
            return Ok(None);
        }

        let mut refs = self.iter_mut().collect::<Vec<_>>();

        refs.shuffle()?;
        refs.truncate(n);

        Ok(Some(refs))
    }

    #[inline]
    #[expect(clippy::cast_possible_truncation)]
    fn select_random_multiple_overlapping_const<const N: usize>(
        &self,
    ) -> Result<Option<[&Self::Item; N]>, getrandom::Error> {
        if self.is_empty() {
            return Ok(None);
        }

        let seed = MagicSeed::new_magic()?;
        let mut rng = PcgInnerState64::oneseq_seeded(seed as u64);

        Ok(Some(array::from_fn(|_| {
            let i = rng.unique_rxs_m_xs_bounded(self.len() as u64) as usize;
            &self[i]
        })))
    }

    #[inline]
    #[expect(clippy::cast_possible_truncation)]
    #[expect(clippy::arithmetic_side_effects)]
    fn select_random_multiple_mut_const<const N: usize>(
        &mut self,
    ) -> Result<Option<[&mut Self::Item; N]>, getrandom::Error> {
        if N > self.len() {
            return Ok(None);
        }

        let mut iter = self.iter_mut();
        let mut reservoir = array::from_fn::<_, N, _>(|_| iter.next().unwrap());

        let seed = MagicSeed::new_magic()?;
        let mut rng = PcgInnerState64::oneseq_seeded(seed as u64);

        for (i, elem) in iter.enumerate() {
            let k = rng.unique_rxs_m_xs_bounded((N + i + 1) as u64) as usize;
            if let Some(slot) = reservoir.get_mut(k) {
                *slot = elem;
            }
        }

        reservoir.shuffle()?;

        Ok(Some(reservoir))
    }

    #[inline]
    fn select_random_multiple_nonoverlapping(
        &self,
        n: usize,
    ) -> Result<Option<Vec<&Self::Item>>, getrandom::Error> {
        if self.is_empty() {
            return Ok(None);
        }

        let mut refs = self.iter().take(n).collect::<Vec<_>>();
        refs.shuffle()?;

        Ok(Some(refs))
    }

    #[inline]
    fn select_random_multiple_nonoverlapping_const<const N: usize>(
        &self,
    ) -> Result<Option<[&Self::Item; N]>, getrandom::Error> {
        if self.is_empty() {
            return Ok(None);
        }

        let mut refs = self.iter().take(N).collect::<Vec<_>>();
        refs.shuffle()?;

        Ok(Some(array::from_fn(|i| refs[i])))
    }
}
