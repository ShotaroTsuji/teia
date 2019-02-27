use crate::traits::{IndexedSet, IndexedSetIters};

#[derive(Debug, Clone)]
pub struct IndexedVec<T> {
    vec: Vec<T>,
    start: usize,
}

impl<T> IndexedSet<T> for IndexedVec<T> {
    #[inline]
    fn new(start: usize) -> Self {
        IndexedVec {
            vec: Vec::new(),
            start: start,
        }
    }

    #[inline]
    fn with_capacity(start: usize, capacity: usize) -> Self {
        IndexedVec {
            vec: Vec::with_capacity(capacity),
            start: start,
        }
    }

    #[inline]
    fn from_vec(vec: Vec<T>, start: usize) -> Self {
        IndexedVec {
            vec: vec,
            start: start,
        }
    }

    #[inline]
    fn len(&self) -> usize {
        self.vec.len()
    }

    #[inline]
    fn index_start(&self) -> usize {
        self.start
    }

    #[inline]
    fn index_end(&self) -> usize {
        self.start + self.vec.len()
    }

    #[inline]
    fn index_range(&self) -> std::ops::Range<usize> {
        self.start..(self.start + self.vec.len())
    }

    #[inline]
    fn push(&mut self, elem: T) {
        self.vec.push(elem);
    }

    #[inline]
    fn get(&self, index: usize) -> Option<&T> {
        self.vec.get(index - self.start)
    }

    #[inline]
    fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.vec.get_mut(index - self.start)
    }

    fn position_within(&self, range: std::ops::Range<usize>, elem: &T) -> Option<usize>
    where
        T: PartialEq,
    {
        for index in range {
            if self[index] == *elem {
                return Some(index);
            }
        }
        None
    }
}

impl<'a, T: 'a> IndexedSetIters<'a, T> for IndexedVec<T> {
    type Iter = Iter<'a, T>;
    type IntoIter = IntoIter<T>;
    type Range = Range<'a, T>;

    #[inline]
    fn iter(&'a self) -> Iter<'a, T> {
        Iter {
            ivec: &self,
            index: self.start,
            _phantom: std::marker::PhantomData,
        }
    }

    #[inline]
    fn into_iter(self) -> IntoIter<T> {
        let start = self.start;
        IntoIter {
            ivec: self,
            index: start,
        }
    }

    #[inline]
    fn range(&'a self, range: std::ops::Range<usize>) -> Range<'a, T> {
        Range {
            ivec: &self,
            index: if range.start < self.index_start() {
                self.index_start()
            } else {
                range.start
            },
            end: if range.end <= self.index_end() {
                range.end
            } else {
                self.index_end()
            },
            _phantom: std::marker::PhantomData,
        }
    }
}

#[derive(Debug)]
pub struct Iter<'a, T> {
    ivec: &'a IndexedVec<T>,
    index: usize,
    _phantom: std::marker::PhantomData<&'a IndexedVec<T>>,
}

impl<'a, T: 'a> Clone for Iter<'a, T> {
    fn clone(&self) -> Self {
        Iter {
            ivec: self.ivec,
            index: self.index,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<'a, T: 'a> Iterator for Iter<'a, T> {
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

pub struct IntoIter<T> {
    ivec: IndexedVec<T>,
    index: usize,
}

impl<T> Iterator for IntoIter<T> {
    type Item = (usize, T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.ivec.index_end() {
            let uninit: T = unsafe { std::mem::zeroed() };
            let data = std::mem::replace(&mut self.ivec[self.index], uninit);
            let pair = (self.index, data);
            self.index += 1;
            Some(pair)
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct Range<'a, T> {
    ivec: &'a IndexedVec<T>,
    index: usize,
    end: usize,
    _phantom: std::marker::PhantomData<&'a IndexedVec<T>>,
}

impl<'a, T: 'a> Clone for Range<'a, T> {
    fn clone(&self) -> Self {
        Range {
            ivec: self.ivec,
            index: self.index,
            end: self.end,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<'a, T: 'a> Iterator for Range<'a, T> {
    type Item = (usize, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.end {
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
        &self.vec[index.start - self.start..index.end - self.start]
    }
}

impl<T> std::ops::IndexMut<std::ops::Range<usize>> for IndexedVec<T> {
    fn index_mut(&mut self, index: std::ops::Range<usize>) -> &mut [T] {
        &mut self.vec[index.start - self.start..index.end - self.start]
    }
}

impl<T> std::ops::Index<std::ops::RangeFrom<usize>> for IndexedVec<T> {
    type Output = [T];

    fn index(&self, index: std::ops::RangeFrom<usize>) -> &Self::Output {
        &self.vec[index.start - self.start..]
    }
}

impl<T> std::ops::IndexMut<std::ops::RangeFrom<usize>> for IndexedVec<T> {
    fn index_mut(&mut self, index: std::ops::RangeFrom<usize>) -> &mut [T] {
        &mut self.vec[index.start - self.start..]
    }
}

impl<T> std::ops::Index<std::ops::RangeTo<usize>> for IndexedVec<T> {
    type Output = [T];

    fn index(&self, index: std::ops::RangeTo<usize>) -> &Self::Output {
        &self.vec[..index.end - self.start]
    }
}

impl<T> std::ops::IndexMut<std::ops::RangeTo<usize>> for IndexedVec<T> {
    fn index_mut(&mut self, index: std::ops::RangeTo<usize>) -> &mut [T] {
        &mut self.vec[..index.end - self.start]
    }
}

impl<T> std::ops::Index<std::ops::RangeFull> for IndexedVec<T> {
    type Output = [T];

    fn index(&self, _index: std::ops::RangeFull) -> &Self::Output {
        &self.vec[..]
    }
}

impl<T> std::ops::IndexMut<std::ops::RangeFull> for IndexedVec<T> {
    fn index_mut(&mut self, _index: std::ops::RangeFull) -> &mut [T] {
        &mut self.vec[..]
    }
}
