use crate::sign::Sign;
use crate::traits::{ChainGenerator, IndexedSet};
use failure::Fail;

#[derive(Debug, Fail)]
pub enum ComplexError {
    #[fail(display = "complex is not filtered")]
    ComplexIsNotFiltered,
}

#[derive(Debug, Clone)]
pub struct Complex<'a, V, G> {
    pub basis: V,
    _phantom: std::marker::PhantomData<&'a G>,
}

impl<'a, V, G> Complex<'a, V, G>
where
    G: 'a,
    V: IndexedSet<'a, G>,
{
    pub fn new() -> Self {
        Complex {
            basis: V::new(0),
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn push(&mut self, elem: G) {
        self.basis.push(elem);
    }

    pub fn boundaries<C>(&'a self) -> Boundaries<'a, V, G, C> {
        Boundaries {
            index: self.basis.index_start(),
            complex: self,
            _phantom0: std::marker::PhantomData,
            _phantom1: std::marker::PhantomData,
        }
    }

    pub fn boundaries_from<C, W>(&'a self, other: &'a Complex<'a, W, G>) -> BoundariesFrom<'a, 'a, V, W, G, C> {
        BoundariesFrom {
            index: self.basis.index_start(),
            complex: self,
            other: other,
            _phantom0: std::marker::PhantomData,
            _phantom1: std::marker::PhantomData,
            _phantom2: std::marker::PhantomData,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Boundaries<'a, V, G, C> {
    index: usize,
    complex: &'a Complex<'a, V, G>,
    _phantom0: std::marker::PhantomData<&'a G>,
    _phantom1: std::marker::PhantomData<fn() -> C>,
}

impl<'a, V, G, C> Iterator for Boundaries<'a, V, G, C>
where
    V: IndexedSet<'a, G> + std::ops::Index<usize, Output = G>,
    <V as IndexedSet<'a, G>>::Range: Clone,
    G: ChainGenerator<'a> + PartialEq,
    C: std::iter::FromIterator<(usize, Sign)>,
{
    type Item = Result<C, ComplexError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.complex.basis.index_end() {
            let chain: Option<C> = BoundaryFacesPositions::new(
                self.complex.basis.range(0..self.index),
                &self.complex.basis[self.index],
            )
            .collect();
            self.index += 1;
            Some(chain.ok_or(ComplexError::ComplexIsNotFiltered))
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
pub struct BoundariesFrom<'a, 'b, V, W, G, C> {
    index: usize,
    complex: &'b Complex<'b, V, G>,
    other: &'a Complex<'a, W, G>,
    _phantom0: std::marker::PhantomData<&'a G>,
    _phantom1: std::marker::PhantomData<&'b G>,
    _phantom2: std::marker::PhantomData<fn() -> C>,
}

impl<'a, 'b: 'a, V, W, G, C> Iterator for BoundariesFrom<'a, 'b, V, W, G, C>
where
    V: IndexedSet<'b, G> + std::ops::Index<usize, Output = G>,
    W: IndexedSet<'a, G>,
    <W as IndexedSet<'a, G>>::Range: Clone,
    G: ChainGenerator<'b> + PartialEq,
    C: std::iter::FromIterator<(usize, Sign)>,
{
    type Item = Result<C, ComplexError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.complex.basis.index_end() {
            let chain: Option<C> = BoundaryFacesPositions::new(
                self.other.basis.range(0..self.index),
                &self.complex.basis[self.index],
            )
            .collect();
            self.index += 1;
            Some(chain.ok_or(ComplexError::ComplexIsNotFiltered))
        } else {
            None
        }
    }
}


/// An iterator that produces pairs of the position and the sign of the boundary of the given
/// simplex.
#[derive(Debug, Clone)]
pub struct BoundaryFacesPositions<Iter, BdIter> {
    /// An iterator from which this struct find generators.
    iter: Iter,
    /// An iterator that produces the faces of the boundary.
    boundary: BdIter,
}

impl<'a, 'b, Iter, BdIter, ChGen> BoundaryFacesPositions<Iter, BdIter>
where
    Iter: Iterator<Item = (usize, &'a ChGen)> + Clone,
    ChGen: 'a + ChainGenerator<'b, BoundaryIter = BdIter> + PartialEq,
    BdIter: Iterator<Item = ChGen>,
{
    pub fn new(iter: Iter, elem: &'b ChGen) -> Self {
        BoundaryFacesPositions {
            iter: iter,
            boundary: elem.boundary(),
        }
    }
}

impl<'a, 'b, Iter, BdIter, ChGen> Iterator for BoundaryFacesPositions<Iter, BdIter>
where
    Iter: Iterator<Item = (usize, &'a ChGen)> + Clone,
    ChGen: 'a + ChainGenerator<'b, BoundaryIter = BdIter> + PartialEq,
    BdIter: Iterator<Item = ChGen>,
{
    type Item = Option<(usize, Sign)>;

    fn next(&mut self) -> Option<Self::Item> {
        self.boundary.next().map(|face| {
            self.iter.clone().find_map(|(index, gen)| {
                let sign = face.inner_prod(gen);
                if sign.is_zero() {
                    None
                } else {
                    Some((index, sign))
                }
            })
        })
    }
}

pub fn compute_boundary<'a, 'b: 'a, I, G, V>(iter: I, elem: &'b G) -> Option<V>
where
    G: 'a + ChainGenerator<'a> + PartialEq,
    I: Iterator<Item = (usize, &'a G)> + Clone,
    V: std::iter::FromIterator<(usize, Sign)>,
{
    elem.boundary()
        .map(|face| {
            iter.clone().find_map(|(i, s)| {
                let sign = face.inner_prod(s);
                if sign.is_zero() {
                    None
                } else {
                    Some((i, sign))
                }
            })
        })
        .collect()
}
