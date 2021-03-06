use crate::sign::Sign;
use crate::traits::*;
use failure::Fail;

#[derive(Debug, Fail)]
pub enum ComplexError {
    #[fail(display = "complex is not filtered")]
    ComplexIsNotFiltered,
    #[fail(display = "element already exists")]
    ElementAlreadyExists,
}

#[derive(Debug, Clone)]
pub struct Complex<V, G> {
    pub basis: V,
    _phantom: std::marker::PhantomData<fn() -> G>,
}

impl<V, G> Complex<V, G>
where
    G: ChainGenerator,
    V: IndexedSet<G>,
{
    pub fn new() -> Self {
        Complex {
            basis: V::new(0),
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn with_prev<'b, W: IndexedSet<G>>(prev: &Complex<W, G>) -> Self {
        Complex {
            basis: V::new(prev.basis.index_end()),
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn push(&mut self, elem: G) -> Result<(), ComplexError>
    where
        V: for<'a> IndexedSetIters<'a, G>,
    {
        let result = {
            self.basis
                .iter()
                .find(|(_, gen)| !gen.inner_prod(&elem).is_zero())
        };
        match result {
            Some(_) => Err(ComplexError::ElementAlreadyExists),
            None => {
                self.basis.push(elem);
                Ok(())
            }
        }
    }

    pub fn push_unchecked(&mut self, elem: G) {
        self.basis.push(elem);
    }

    pub fn boundaries<'a, FrIt>(&'a self) -> Boundaries<'a, 'a, V, V, G, FrIt> {
        Boundaries {
            index: self.basis.index_start(),
            domain: &self.basis,
            target: &self.basis,
            _phantom0: std::marker::PhantomData,
            _phantom1: std::marker::PhantomData,
            _phantom2: std::marker::PhantomData,
        }
    }

    pub fn boundaries_from<'a, 'b, FrIt, W>(
        &'a self,
        other: &'b Complex<W, G>,
    ) -> Boundaries<'a, 'b, V, W, G, FrIt> {
        Boundaries {
            index: self.basis.index_start(),
            domain: &self.basis,
            target: &other.basis,
            _phantom0: std::marker::PhantomData,
            _phantom1: std::marker::PhantomData,
            _phantom2: std::marker::PhantomData,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Boundaries<'a, 'b, V, W, ChGen, FrIt> {
    index: usize,
    domain: &'a V,
    target: &'b W,
    _phantom0: std::marker::PhantomData<&'a ChGen>,
    _phantom1: std::marker::PhantomData<&'b ChGen>,
    _phantom2: std::marker::PhantomData<fn() -> FrIt>,
}

impl<'a, 'b, V, W, ChGen, FrIt> Iterator for Boundaries<'a, 'b, V, W, ChGen, FrIt>
where
    V: IndexedSet<ChGen>,
    W: IndexedSetIters<'b, ChGen>,
    ChGen: 'a + 'b + PartialEq + ChainGenerator + ChainGeneratorBoundary<'a, ChGen>,
    FrIt: std::iter::FromIterator<(usize, Sign)>,
{
    type Item = Result<(usize, FrIt), ComplexError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.domain.index_end() {
            let chain: Option<FrIt> = BoundaryFacesPositions::new(
                self.target.range(0..self.index),
                self.domain.get(self.index).unwrap(),
            )
            .collect();
            let index = self.index;
            self.index += 1;
            Some(
                chain
                    .ok_or(ComplexError::ComplexIsNotFiltered)
                    .map(|value| (index, value)),
            )
        } else {
            None
        }
    }
}

/// An iterator that produces pairs of the position and the sign of the boundary of the given
/// simplex.
pub struct BoundaryFacesPositions<'a, 'b, Iter, ChGen>
where
    Iter: Iterator<Item = (usize, &'a ChGen)> + Clone,
    ChGen: 'a + ChainGenerator + ChainGeneratorBoundary<'b, ChGen>,
{
    /// An iterator from which generators are found.
    iter: Iter,
    /// An iterator that produces the faces of the boundary.
    boundary: <ChGen as ChainGeneratorBoundary<'b, ChGen>>::BoundaryIter,
}

/// The lifetime 'b is a lifetime for `boundary`.
impl<'a, 'b, Iter, ChGen> BoundaryFacesPositions<'a, 'b, Iter, ChGen>
where
    Iter: Iterator<Item = (usize, &'a ChGen)> + Clone,
    ChGen: 'a + ChainGenerator + ChainGeneratorBoundary<'b, ChGen>,
{
    pub fn new(iter: Iter, elem: &'b ChGen) -> Self {
        BoundaryFacesPositions {
            iter: iter,
            boundary: elem.boundary(),
        }
    }
}

impl<'a, 'b, Iter, ChGen> Iterator for BoundaryFacesPositions<'a, 'b, Iter, ChGen>
where
    Iter: Iterator<Item = (usize, &'a ChGen)> + Clone,
    ChGen: 'a + ChainGenerator + ChainGeneratorBoundary<'b, ChGen>,
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

pub fn compute_boundary<'a, 'b, I, G, V>(iter: I, elem: &'b G) -> Option<V>
where
    G: 'a + PartialEq + ChainGenerator + ChainGeneratorBoundary<'b, G>,
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
