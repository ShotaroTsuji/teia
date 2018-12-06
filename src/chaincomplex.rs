use Index;
use std::ops::Range;

pub trait ChainComplex {
    type Index : Index;
    type Vector;

    fn boundary(&self, index: Self::Index) -> Self::Vector;
    fn dimension(&self) -> usize;
    fn index_range(&self) -> Range<Self::Index>;
}
