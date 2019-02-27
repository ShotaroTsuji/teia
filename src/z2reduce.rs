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

    pub fn from_complex<'a, IdVec, Gen>(complex: &'a Complex<IdVec, Gen>) -> Result<Z2ColumnReduce<V>, failure::Error>
    where
        V: FromIterator<(usize, Sign)>,
        Gen: 'a + PartialEq + ChainGenerator + ChainGeneratorBoundary<'a, Gen>,
        IdVec: IndexedSet<Gen> + IndexedSetIters<'a, Gen>,
        <IdVec as IndexedSetIters<'a, Gen>>::Range: Clone,
    {
        let mut reduce = Self::new(complex.basis.index_start());

        for result in complex.boundaries::<V>() {
            let (_, image) = result?;
            reduce.push(image);
        }

        Ok(reduce)
    }

    pub fn from_complex_with<'a, IdSet, ChGen, F, U>(complex: &'a Complex<IdSet, ChGen>, mut f: F) -> Result<Z2ColumnReduce<V>, failure::Error>
    where
        ChGen: 'a + PartialEq + ChainGenerator + ChainGeneratorBoundary<'a, ChGen>,
        IdSet: IndexedSet<ChGen> + IndexedSetIters<'a, ChGen>,
        <IdSet as IndexedSetIters<'a, ChGen>>::Range: Clone,
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

    pub fn from_complexes<'a, 'b, IdSetA, IdSetB, ChGen>(domain: &'a Complex<IdSetA, ChGen>, target: &'b Complex<IdSetB, ChGen>) -> Result<Z2ColumnReduce<V>, failure::Error>
    where
        V: FromIterator<(usize, Sign)>,
        ChGen: 'a + 'b + PartialEq + ChainGenerator + ChainGeneratorBoundary<'a, ChGen>,
        IdSetA: IndexedSet<ChGen>,
        IdSetB: IndexedSetIters<'b, ChGen>,
        <IdSetB as IndexedSetIters<'b, ChGen>>::Range: Clone,
    {
        let mut reduce = Self::new(domain.basis.index_start());

        for result in domain.boundaries_from::<V, _>(target) {
            let (_, image) = result?;
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

    pub fn cycles<'a>(&'a self) -> CyclesIter<'a, <IndexedVec<V> as IndexedSetIters<V>>::Iter, V> {
        CyclesIter {
            iter: self.reduced.iter(),
            _phantom: (PhantomData, PhantomData),
        }
    }

    pub fn into_cycles(self) -> Cycles<V> {
        let mut cycles = Vec::new();

        for (index, chain) in self.reduced.into_iter() {
            if chain.is_cycle() {
                cycles.push((index, chain));
            }
        }

        Cycles {
            cycles: cycles,
        }
    }

    pub fn into_cycle_positions(self) -> CyclePositions {
        let mut positions = Vec::new();

        for (index, chain) in self.reduced.iter() {
            if chain.is_cycle() {
                positions.push(index);
            }
        }

        CyclePositions {
            positions: positions,
        }
    }
}

impl<V> LookupByLowest for Z2ColumnReduce<V> {
    fn lookup_by_lowest(&self, lowest: usize) -> Option<usize> {
        self.lowest_memo.get(&lowest).map(|pos| *pos)
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

pub struct Cycles<V> {
    cycles: Vec<(usize, V)>,
}

impl<V> Cycles<V> {
    pub fn iter(&self) -> impl Iterator<Item=(usize, &V)> {
        self.cycles
            .iter()
            .map(|(index, chain)| (*index, chain))
    }
}

pub struct CyclePositions {
    positions: Vec<usize>,
}

impl CyclePositions {
    pub fn iter<'a>(&'a self) -> impl Iterator<Item=(usize, ())> + 'a {
        self.positions
            .iter()
            .map(|pos| (*pos, ()))
    }
}


pub struct Z2Pair<'a, B, Z, C> {
    reduce: &'a B,
    cycles: Z,
    _phantom: (PhantomData<&'a B>, PhantomData<fn () -> C>),
}

impl<'a, B, Z, C> Z2Pair<'a, B, Z, C>
where
    B: LookupByLowest,
    Z: Iterator<Item=(usize, C)>,
{
    pub fn new(reduce: &'a B, cycles: Z) -> Self {
        Z2Pair {
            reduce: reduce,
            cycles: cycles,
            _phantom: (PhantomData, PhantomData),
        }
    }
}

impl<'a, B, Z, C> Iterator for Z2Pair<'a, B, Z, C>
where
    B: LookupByLowest,
    Z: Iterator<Item=(usize, C)>,
{
    type Item = (Persistence<usize>, C);

    fn next(&mut self) -> Option<Self::Item> {
        self.cycles.next()
            .map(|(index, cycle)| {
                let boundary_pos = self.reduce.lookup_by_lowest(index);
                (Persistence(index, boundary_pos), cycle)
            })
    }
}
