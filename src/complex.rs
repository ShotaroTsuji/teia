use crate::indexed_vec::IndexedVec;
use crate::simplex::Simplex;

#[derive(Debug, Clone)]
pub struct Complex {
    basis: Vec<Simplex>,
}

impl Complex {
    pub fn new() -> Self {
        Complex {
            basis: Vec::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.basis.len()
    }

    pub fn index_range(&self) -> std::ops::Range<usize> {
        0..self.basis.len()
    }

    pub fn push(&mut self, elem: Simplex) {
        self.basis.push(elem);
    }
}
