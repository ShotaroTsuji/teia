use crate::sign::Sign;

pub trait ChainGenerator<'a> {
    type VerticesIter;
    type BoundaryIter;

    fn dimension(&self) -> usize;
    fn vertices(&'a self) -> Self::VerticesIter;
    fn boundary(&'a self) -> Self::BoundaryIter;
    fn inner_prod(&self, other: &Self) -> Sign;
    fn is_face_of(&self, other: &Self) -> bool;
}

pub trait IndexedSet<'a, T: 'a> {
    type Iter: Iterator<Item=(usize, &'a T)>;

    fn from_vec(vec: Vec<T>, start: usize) -> Self;
    fn len(&self) -> usize;
    fn index_start(&self) -> usize;
    fn index_end(&self) -> usize;
    fn index_range(&self) -> std::ops::Range<usize>;
    fn push(&mut self, elem: T);
    fn get(&self, index: usize) -> Option<&T>;
    fn get_mut(&mut self, index: usize) -> Option<&mut T>;
    fn iter(&'a self) -> Self::Iter;
}
