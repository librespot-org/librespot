use rand::Rng;
use std::{
    ops::{Deref, DerefMut},
    vec::IntoIter,
};

#[derive(Debug)]
pub struct ShuffleVec<T> {
    vec: Vec<T>,
    indices: Option<Vec<usize>>,
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

impl<T> ShuffleVec<T> {
    pub fn new(vec: Vec<T>) -> Self {
        Self { vec, indices: None }
    }

    pub fn shuffle(&mut self) {
        if self.indices.is_some() {
            self.unshuffle()
        }

        let indices = {
            let mut rng = rand::thread_rng();
            (1..self.vec.len())
                .rev()
                .map(|i| rng.gen_range(0..i + 1))
                .collect()
        };

        for (i, &rnd_ind) in (1..self.vec.len()).rev().zip(&indices) {
            self.vec.swap(i, rnd_ind);
        }

        self.indices = Some(indices)
    }

    pub fn unshuffle(&mut self) {
        let indices = match self.indices.take() {
            Some(indices) => indices,
            None => return,
        };

        for i in 1..self.vec.len() {
            let n = indices[self.vec.len() - i - 1];
            self.vec.swap(n, i);
        }
    }
}
