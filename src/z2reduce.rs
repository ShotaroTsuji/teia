use std::collections::BTreeMap;
use std::marker::PhantomData;
use crate::{Index, Persistence};
use crate::z2vector::Z2Vector;

#[derive(Debug)]
pub struct Z2ColumnReducer<I, V> {
    reduced: Vec<V>,
    // mapping of lowest index to position in `reduced`
    lowest_memo: BTreeMap<I, usize>,
}

impl<I, V> Z2ColumnReducer<I, V>
where
    I: Index,
    V: Z2Vector<Index = I> + std::fmt::Debug,
{
    pub fn new() -> Z2ColumnReducer<I, V> {
        Z2ColumnReducer {
            reduced: Vec::new(),
            lowest_memo: BTreeMap::new(),
        }
    }

    /*
    pub fn find_same_lowest(&self, boundary: &V) -> Option<(I, &V)> {
        match boundary.lowest() {
            Some(lowest) => {
                for i in 0..self.reduced.len() {
                    if let Some(pos) = self.reduced[i].lowest() {
                        if *pos == *lowest {
                            return Some((FromPrimitive::from_usize(i).unwrap(), &self.reduced[i]));
                        }
                    }
                }
                None
            },
            None => None,
        }
    }
    */
    pub fn find_same_lowest(&self, boundary: &V) -> Option<(I, &V)> {
        boundary.lowest().and_then(|lowest|
            self.lowest_memo.get(lowest)
                .map(|pos| (Index::from_usize(*pos), &self.reduced[*pos])))
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

        if let Some(&lowest) = boundary.lowest() {
            let index = self.reduced.len();
            self.lowest_memo.insert(lowest, index);
        }

        self.reduced.push(boundary);
    }

    pub fn cycles<'a>(&'a self) -> impl Iterator<Item=(usize, &V)> {
        self.reduced.iter()
            .enumerate()
            .filter(|(_, c): &(usize, &V)| c.is_cycle())
    }
}

pub struct Z2Pairer<'a, I, V, T> {
    reducer: &'a Z2ColumnReducer<I, V>,
    cycles: T,
    _phantom: PhantomData<&'a Z2ColumnReducer<I, V>>,
}

impl<'a, I, V, T> Z2Pairer<'a, I, V, T>
where
    I: Index,
    V: Z2Vector<Index=I>,
    T: Iterator<Item=(usize, &'a V)>,
{
    pub fn new(reducer: &'a Z2ColumnReducer<I, V>, cycles: T) -> Self {
        Z2Pairer {
            reducer: reducer,
            cycles: cycles,
            _phantom: PhantomData,
        }
    }
}

impl<'a, I, V, T> Iterator for Z2Pairer<'a, I, V, T>
where
    I: Index,
    V: Z2Vector<Index=I> + std::fmt::Debug,
    T: Iterator<Item=(usize, &'a V)>,
{
    type Item = (Persistence<I>, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        self.cycles.next()
            .map(|(index, cycle)| {
                let index= Index::from_usize(index);
                let boundary_pos = self.reducer.lowest_memo.get(&index).map(|pos| Index::from_usize(*pos));
                (Persistence(index, boundary_pos), cycle)
            })
    }
}
