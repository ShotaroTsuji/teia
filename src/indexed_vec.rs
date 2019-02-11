use crate::traits::IndexedSet;

#[derive(Debug,Clone)]
pub struct IndexedVec<T> {
    vec: Vec<T>,
    start: usize,
}

impl<'a, T: 'a> IndexedSet<'a, T> for IndexedVec<T> {
    type Iter = Iter<'a, T>;
    type Range = Range<'a, T>;

    #[inline]
    fn new(start: usize) -> Self {
        IndexedVec {
            vec: Vec::new(),
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
        self.start..(self.start+self.vec.len())
    }

    #[inline]
    fn push(&mut self, elem: T) {
        self.vec.push(elem);
    }

    #[inline]
    fn get(&self, index: usize) -> Option<&T> {
        self.vec.get(index-self.start)
    }

    #[inline]
    fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.vec.get_mut(index-self.start)
    }

    fn position_within(&self, range: std::ops::Range<usize>, elem: &T) -> Option<usize> where T: PartialEq {
        for index in range {
            if self[index] == *elem {
                return Some(index);
            }
        }
        None
    }

    #[inline]
    fn iter(&'a self) -> Iter<'a, T> {
        Iter {
            ivec: &self,
            index: self.start,
            _phantom: std::marker::PhantomData,
        }
    }

    #[inline]
    fn range(&'a self, range: std::ops::Range<usize>) -> Range<'a, T> {
        Range {
            ivec: &self,
            index: range.start,
            end: range.end,
            _phantom: std::marker::PhantomData,
        }
    }
}

#[derive(Debug,Clone)]
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

#[derive(Debug,Clone)]
pub struct Range<'a, T> {
    ivec: &'a IndexedVec<T>,
    index: usize,
    end: usize,
    _phantom: std::marker::PhantomData<&'a IndexedVec<T>>,
}

impl<'a, T> Iterator for Range<'a, T> {
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

    fn index(&self, _index: std::ops::RangeFull) -> &Self::Output {
        &self.vec[..]
    }
}

impl<T> std::ops::IndexMut<std::ops::RangeFull> for IndexedVec<T> {
    fn index_mut(&mut self, _index: std::ops::RangeFull) -> &mut [T] {
        &mut self.vec[..]
    }
}
