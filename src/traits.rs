use crate::sign::Sign;

pub trait ChainGeneratorVertices<'a> {
    type VerticesIter: Iterator<Item = &'a usize>;

    fn vertices(&'a self) -> Self::VerticesIter;
}

pub trait ChainGeneratorBoundary<'a, ChGen>
where
    ChGen: ChainGenerator + Sized,
{
    type BoundaryIter: Iterator<Item = ChGen>;

    fn boundary(&'a self) -> Self::BoundaryIter;
}

pub trait ChainGenerator {
    fn dimension(&self) -> usize;
    fn inner_prod(&self, other: &Self) -> Sign;
    fn is_face_of(&self, other: &Self) -> bool;
}

pub trait IndexedSet<T> {
    fn new(index: usize) -> Self;
    fn with_capacity(index: usize, capacity: usize) -> Self;
    fn from_vec(vec: Vec<T>, start: usize) -> Self;
    fn len(&self) -> usize;
    fn index_start(&self) -> usize;
    fn index_end(&self) -> usize;
    fn index_range(&self) -> std::ops::Range<usize>;
    fn push(&mut self, elem: T);
    fn get(&self, index: usize) -> Option<&T>;
    fn get_mut(&mut self, index: usize) -> Option<&mut T>;
    fn position_within(&self, range: std::ops::Range<usize>, elem: &T) -> Option<usize>
    where
        T: PartialEq;
}

pub trait IndexedSetIters<'a, T: 'a> {
    type Iter: Iterator<Item = (usize, &'a T)> + Clone;
    type IntoIter: Iterator<Item = (usize, T)>;
    type Range: Iterator<Item = (usize, &'a T)> + Clone;

    fn iter(&'a self) -> Self::Iter;
    fn into_iter(self) -> Self::IntoIter;
    fn range(&'a self, range: std::ops::Range<usize>) -> Self::Range;
}

pub trait LookupByLowest {
    fn lookup_by_lowest(&self, lowest: usize) -> Option<usize>;
}
