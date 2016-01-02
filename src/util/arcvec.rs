use std::sync::Arc;
use std::fmt;
use std::ops::Deref;

#[derive(Clone)]
pub struct ArcVec<T> {
    data: Arc<Vec<T>>,
    offset: usize,
    length: usize,
}

impl<T> ArcVec<T> {
    pub fn new(data: Vec<T>) -> ArcVec<T> {
        let length = data.len();
        ArcVec {
            data: Arc::new(data),
            offset: 0,
            length: length,
        }
    }

    pub fn offset(mut self, offset: usize) -> ArcVec<T> {
        assert!(offset <= self.length);

        self.offset += offset;
        self.length -= offset;

        self
    }

    pub fn limit(mut self, length: usize) -> ArcVec<T> {
        assert!(length <= self.length);
        self.length = length;

        self
    }
}

impl<T> Deref for ArcVec<T> {
    type Target = [T];

    fn deref(&self) -> &[T] {
        &self.data[self.offset..self.offset + self.length]
    }
}

impl<T: fmt::Debug> fmt::Debug for ArcVec<T> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        self.deref().fmt(formatter)
    }
}
