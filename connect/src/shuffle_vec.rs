use rand::{Rng, SeedableRng, rngs::SmallRng};
use std::{
    ops::{Deref, DerefMut},
    vec::IntoIter,
};

#[derive(Debug, Clone, Default)]
pub struct ShuffleVec<T> {
    vec: Vec<T>,
    indices: Option<Vec<usize>>,
    /// This is primarily necessary to ensure that shuffle does not behave out of place.
    ///
    /// For that reason we swap the first track with the currently playing track. By that we ensure
    /// that the shuffle state is consistent between resets of the state because the first track is
    /// always the track with which we started playing when switching to shuffle.
    original_first_position: Option<usize>,
}

impl<T: PartialEq> PartialEq for ShuffleVec<T> {
    fn eq(&self, other: &Self) -> bool {
        self.vec == other.vec
    }
}

impl<T> Deref for ShuffleVec<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.vec
    }
}

impl<T> DerefMut for ShuffleVec<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.vec.as_mut()
    }
}

impl<T> IntoIterator for ShuffleVec<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.vec.into_iter()
    }
}

impl<T> From<Vec<T>> for ShuffleVec<T> {
    fn from(vec: Vec<T>) -> Self {
        Self {
            vec,
            original_first_position: None,
            indices: None,
        }
    }
}

impl<T> ShuffleVec<T> {
    pub fn shuffle_with_seed<F: Fn(&T) -> bool>(
        &mut self,
        seed: u64,
        is_first: F,
    ) -> Option<usize> {
        self.shuffle_with_rng(SmallRng::seed_from_u64(seed), is_first)
    }

    pub fn shuffle_with_rng<F: Fn(&T) -> bool>(
        &mut self,
        mut rng: impl Rng,
        is_first: F,
    ) -> Option<usize> {
        if self.vec.len() < 3 {
            info!("skipped shuffling for less than three items");
            return None;
        }

        if self.indices.is_some() {
            self.unshuffle()
        }

        let indices: Vec<_> = {
            (1..self.vec.len())
                .rev()
                .map(|i| rng.random_range(0..i + 1))
                .collect()
        };

        for (i, &rnd_ind) in (1..self.vec.len()).rev().zip(&indices) {
            self.vec.swap(i, rnd_ind);
        }

        self.indices = Some(indices);

        self.original_first_position = self.vec.iter().position(is_first);
        if let Some(first_pos) = self.original_first_position {
            self.vec.swap(0, first_pos)
        }

        self.original_first_position
    }

    pub fn unshuffle(&mut self) {
        let indices = match self.indices.take() {
            Some(indices) => indices,
            None => return,
        };

        if let Some(first_pos) = self.original_first_position {
            self.vec.swap(0, first_pos);
            self.original_first_position = None;
        }

        for i in 1..self.vec.len() {
            match indices.get(self.vec.len() - i - 1) {
                None => return,
                Some(n) => self.vec.swap(*n, i),
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rand::Rng;

    #[test]
    fn test_shuffle_with_seed() {
        let seed = rand::rng().random_range(0..10000000000000);

        let vec = (0..100).collect::<Vec<_>>();
        let base_vec: ShuffleVec<i32> = vec.into();

        let mut shuffled_vec = base_vec.clone();
        shuffled_vec.shuffle_with_seed(seed, |_| false);

        let mut different_shuffled_vec = base_vec.clone();
        different_shuffled_vec.shuffle_with_seed(seed, |_| false);

        assert_eq!(shuffled_vec, different_shuffled_vec);

        let mut unshuffled_vec = shuffled_vec.clone();
        unshuffled_vec.unshuffle();

        assert_eq!(base_vec, unshuffled_vec);
    }
}
