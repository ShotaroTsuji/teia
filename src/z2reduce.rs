use std::collections::BTreeMap;
use std::marker::PhantomData;
use std::iter::FromIterator;
use crate::Persistence;
use crate::z2vector::Z2Vector;
use crate::traits::IndexedSet;
use crate::indexed_vec::IndexedVec;

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

    pub fn find_same_lowest(&self, boundary: &V) -> Option<(usize, &V)> {
        boundary.lowest().and_then(|lowest|
            self.lowest_memo.get(&lowest)
                .map(|pos| (*pos, &self.reduced[*pos])))
    }

    pub fn reduce(&self, boundary: &mut V) {
        println!("reduce the boundary {:?}", boundary);
        while let Some((_, chain)) = self.find_same_lowest(boundary) {
            println!("... {:?}", chain);
            boundary.add_assign(chain);
        }
    }

    pub fn push(&mut self, mut boundary: V) {
        if boundary.lowest().is_some() {
            self.reduce(&mut boundary);
        }

        if let Some(lowest) = boundary.lowest() {
            let index = self.reduced.len();
            self.lowest_memo.insert(lowest, index);
        }

        self.reduced.push(boundary);
    }

    pub fn cycles<'a>(&'a self) -> impl Iterator<Item=(usize, &V)> {
        self.reduced.iter()
            .filter(|(_, c): &(usize, &V)| c.is_cycle())
    }
}

pub struct Z2Pair<'a, V, T> {
    reduce: &'a Z2ColumnReduce<V>,
    cycles: T,
    _phantom: PhantomData<&'a Z2ColumnReduce<V>>,
}

impl<'a, V, T> Z2Pair<'a, V, T>
where
    V: Z2Vector,
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
    V: Z2Vector + std::fmt::Debug,
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
