use std::collections::BTreeMap;
use std::marker::PhantomData;
use std::iter::FromIterator;
use crate::Persistence;
use crate::sign::Sign;
use crate::z2vector::Z2Vector;
use crate::traits::*;
use crate::indexed_vec::IndexedVec;
use crate::complex::Complex;

#[derive(Debug)]
pub struct Z2ColumnReduce<V> {
    reduced: IndexedVec<V>,
    // mapping of lowest index to position in `reduced`
    lowest_memo: BTreeMap<usize, usize>,
}

impl<V> Z2ColumnReduce<V>
where
    V: Z2Vector + std::fmt::Debug,
{
    pub fn new(start: usize) -> Z2ColumnReduce<V> {
        Z2ColumnReduce {
            reduced: IndexedVec::new(start),
            lowest_memo: BTreeMap::new(),
        }
    }

    pub fn from_complex<'a, IdVec, Gen>(complex: &'a Complex<'a, IdVec, Gen>) -> Result<Z2ColumnReduce<V>, failure::Error>
    where
        V: FromIterator<(usize, Sign)>,
        Gen: 'a + PartialEq + ChainGenerator + ChainGeneratorBoundary<'a, Gen>,
        IdVec: IndexedSet<'a, Gen>,
        <IdVec as IndexedSet<'a, Gen>>::Range: Clone,
    {
        let mut reduce = Self::new(complex.basis.index_start());

        for result in complex.boundaries::<V>() {
            let (index, image) = result?;
            reduce.push(image);
        }

        Ok(reduce)
    }

    pub fn from_complex_with<'a, IdVec, Gen, F, U>(complex: &'a Complex<'a, IdVec, Gen>, mut f: F) -> Result<Z2ColumnReduce<V>, failure::Error>
    where
        Gen: 'a + PartialEq + ChainGenerator + ChainGeneratorBoundary<'a, Gen>,
        IdVec: IndexedSet<'a, Gen>,
        <IdVec as IndexedSet<'a, Gen>>::Range: Clone,
        F: FnMut(usize, U) -> V,
        U: Z2Vector + FromIterator<(usize, Sign)>,
    {
        let mut reduce = Self::new(complex.basis.index_start());

        for result in complex.boundaries::<U>() {
            let (index, image) = result?;
            reduce.push(f(index, image));
        }

        Ok(reduce)
    }

    pub fn from_complexes<'a, 'b, IdSetA, IdSetB, ChGen>(domain: &'a Complex<'a, IdSetA, ChGen>, target: &'b Complex<'b, IdSetB, ChGen>) -> Result<Z2ColumnReduce<V>, failure::Error>
    where
        V: FromIterator<(usize, Sign)>,
        ChGen: 'a + 'b + PartialEq + ChainGenerator + ChainGeneratorBoundary<'a, ChGen>,
        IdSetA: IndexedSet<'a, ChGen>,
        IdSetB: IndexedSet<'b, ChGen>,
        <IdSetB as IndexedSet<'b, ChGen>>::Range: Clone,
    {
        let mut reduce = Self::new(domain.basis.index_start());

        for result in domain.boundaries_from::<V, _>(target) {
            let (index, image) = result?;
            reduce.push(image);
        }

        Ok(reduce)
    }

    pub fn find_same_lowest(&self, boundary: &V) -> Option<(usize, &V)> {
        boundary.lowest().and_then(|lowest|
            self.lowest_memo.get(&lowest)
                .map(|pos| (*pos, &self.reduced[*pos])))
    }

    pub fn reduce(&self, boundary: &mut V) {
        while let Some((_, chain)) = self.find_same_lowest(boundary) {
            boundary.add_assign(chain);
        }
    }

    pub fn push(&mut self, mut boundary: V) {
        if boundary.lowest().is_some() {
            self.reduce(&mut boundary);
        }

        if let Some(lowest) = boundary.lowest() {
            let index = self.reduced.index_end();
            self.lowest_memo.insert(lowest, index);
        }

        self.reduced.push(boundary);
    }

    pub fn cycles<'a>(&'a self) -> CyclesIter<'a, <IndexedVec<V> as IndexedSet<V>>::Iter, V> {
        CyclesIter {
            iter: self.reduced.iter(),
            _phantom: (PhantomData, PhantomData),
        }
    }
}

pub struct CyclesIter<'a, I, V> {
    iter: I,
    _phantom: (PhantomData<fn () -> V>, PhantomData<&'a V>),
}

impl<'a, I, V> Iterator for CyclesIter<'a, I, V>
where
    I: Iterator<Item=(usize, &'a V)>,
    V: Z2Vector,
{
    type Item = (usize, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        while let Some((index, chain)) = self.iter.next() {
            if chain.is_cycle() {
                return Some((index, chain));
            }
        }
        None
    }
}

pub struct Z2Pair<'a, V, T> {
    reduce: &'a Z2ColumnReduce<V>,
    cycles: T,
    _phantom: PhantomData<&'a Z2ColumnReduce<V>>,
}

impl<'a, V, T> Z2Pair<'a, V, T>
where
    T: Iterator<Item=(usize, &'a V)>,
{
    pub fn new(reduce: &'a Z2ColumnReduce<V>, cycles: T) -> Self {
        Z2Pair {
            reduce: reduce,
            cycles: cycles,
            _phantom: PhantomData,
        }
    }
}

impl<'a, V, T> Iterator for Z2Pair<'a, V, T>
where
    T: Iterator<Item=(usize, &'a V)>,
{
    type Item = (Persistence<usize>, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        self.cycles.next()
            .map(|(index, cycle)| {
                let boundary_pos = self.reduce.lowest_memo.get(&index).map(|pos| *pos);
                (Persistence(index, boundary_pos), cycle)
            })
    }
}
