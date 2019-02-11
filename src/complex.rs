use crate::indexed_vec::IndexedVec;
use crate::simplex::Simplex;
use crate::traits::IndexedSet;

#[derive(Debug, Clone)]
pub struct Complex {
    basis: IndexedVec<Simplex>,
}

impl Complex {
    pub fn new() -> Self {
        Complex {
            basis: IndexedVec::new(0),
        }
    }

    pub fn with_start_index(start: usize) -> Self {
        Complex {
            basis: IndexedVec::new(start),
        }
    }

    pub fn len(&self) -> usize {
        self.basis.len()
    }

    pub fn index_range(&self) -> std::ops::Range<usize> {
        self.basis.index_range()
    }

    pub fn push(&mut self, elem: Simplex) {
        self.basis.push(elem);
    }

    pub fn get(&self, index: usize) -> Option<&Simplex> {
        self.basis.get(index)
    }
}
