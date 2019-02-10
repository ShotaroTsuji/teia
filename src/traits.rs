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
