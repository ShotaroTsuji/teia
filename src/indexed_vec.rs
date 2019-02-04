use crate::simplex::Simplex;

#[derive(Debug,Clone)]
pub struct IndexedVec<T> {
    vec: Vec<T>,
    start: usize,
}

impl<T> IndexedVec<T> {
    #[inline]
    pub fn new(start: usize) -> Self {
        IndexedVec {
            vec: Vec::new(),
            start: start,
        }
    }

    #[inline]
    pub fn from_vec(vec: Vec<T>, start: usize) -> Self {
        IndexedVec {
            vec: vec,
            start: start,
        }
    }

    #[inline]
    pub fn index_start(&self) -> usize {
        self.start
    }

    #[inline]
    pub fn index_end(&self) -> usize {
        self.start + self.vec.len()
    }

    #[inline]
    pub fn index_range(&self) -> std::ops::Range<usize> {
        self.start..(self.start+self.vec.len())
    }

    #[inline]
    pub fn push(&mut self, elem: T) {
        self.vec.push(elem);
    }

    #[inline]
    pub fn get(&self, index: usize) -> Option<&T> {
        self.vec.get(index-self.start)
    }

    #[inline]
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.vec.get_mut(index-self.start)
    }

    #[inline]
    pub fn iter(&self) -> Iter<T> {
        Iter {
            ivec: &self,
            index: self.start,
            _phantom: std::marker::PhantomData,
        }
    }
}

pub struct Iter<'a, T> {
    ivec: &'a IndexedVec<T>,
    index: usize,
    _phantom: std::marker::PhantomData<&'a IndexedVec<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = (usize, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.ivec.index_end() {
            let pair = (self.index, &self.ivec[self.index]);
            self.index += 1;
            Some(pair)
        } else {
            None
        }
    }
}

impl<T> std::ops::Index<usize> for IndexedVec<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.get(index).unwrap()
    }
}

impl<T> std::ops::IndexMut<usize> for IndexedVec<T> {
    fn index_mut(&mut self, index: usize) -> &mut T {
        self.get_mut(index).unwrap()
    }
}

impl<T> std::ops::Index<std::ops::Range<usize>> for IndexedVec<T> {
    type Output = [T];

    fn index(&self, index: std::ops::Range<usize>) -> &Self::Output {
        &self.vec[index.start-self.start..index.end-self.start]
    }
}

impl<T> std::ops::IndexMut<std::ops::Range<usize>> for IndexedVec<T> {
    fn index_mut(&mut self, index: std::ops::Range<usize>) -> &mut [T] {
        &mut self.vec[index.start-self.start..index.end-self.start]
    }
}

impl<T> std::ops::Index<std::ops::RangeFrom<usize>> for IndexedVec<T> {
    type Output = [T];

    fn index(&self, index: std::ops::RangeFrom<usize>) -> &Self::Output {
        &self.vec[index.start-self.start..]
    }
}

impl<T> std::ops::IndexMut<std::ops::RangeFrom<usize>> for IndexedVec<T> {
    fn index_mut(&mut self, index: std::ops::RangeFrom<usize>) -> &mut [T] {
        &mut self.vec[index.start-self.start..]
    }
}

impl<T> std::ops::Index<std::ops::RangeTo<usize>> for IndexedVec<T> {
    type Output = [T];

    fn index(&self, index: std::ops::RangeTo<usize>) -> &Self::Output {
        &self.vec[..index.end-self.start]
    }
}

impl<T> std::ops::IndexMut<std::ops::RangeTo<usize>> for IndexedVec<T> {
    fn index_mut(&mut self, index: std::ops::RangeTo<usize>) -> &mut [T] {
        &mut self.vec[..index.end-self.start]
    }
}

impl<T> std::ops::Index<std::ops::RangeFull> for IndexedVec<T> {
    type Output = [T];

    fn index(&self, index: std::ops::RangeFull) -> &Self::Output {
        &self.vec[..]
    }
}

impl<T> std::ops::IndexMut<std::ops::RangeFull> for IndexedVec<T> {
    fn index_mut(&mut self, index: std::ops::RangeFull) -> &mut [T] {
        &mut self.vec[..]
    }
}
