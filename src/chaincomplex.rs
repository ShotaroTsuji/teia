use std::ops::Range;
use Index;

pub trait ChainComplex {
    type Index: Index;
    type Vector;

    fn boundary(&self, index: Self::Index) -> Self::Vector;
    fn dimension(&self) -> usize;
    fn index_range(&self) -> Range<Self::Index>;
}
