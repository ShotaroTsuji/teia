use std::collections::BTreeMap;
use std::marker::PhantomData;
use crate::Persistence;
use crate::z2vector::Z2Vector;

#[derive(Debug)]
pub struct Z2ColumnReducer<V> {
    reduced: Vec<V>,
    // mapping of lowest index to position in `reduced`
    lowest_memo: BTreeMap<usize, usize>,
}

impl<V> Z2ColumnReducer<V>
where
    V: Z2Vector + std::fmt::Debug,
{
    pub fn new() -> Z2ColumnReducer<V> {
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
            .enumerate()
            .filter(|(_, c): &(usize, &V)| c.is_cycle())
    }
}

pub struct Z2Pairer<'a, V, T> {
    reducer: &'a Z2ColumnReducer<V>,
    cycles: T,
    _phantom: PhantomData<&'a Z2ColumnReducer<V>>,
}

impl<'a, V, T> Z2Pairer<'a, V, T>
where
    V: Z2Vector,
    T: Iterator<Item=(usize, &'a V)>,
{
    pub fn new(reducer: &'a Z2ColumnReducer<V>, cycles: T) -> Self {
        Z2Pairer {
            reducer: reducer,
            cycles: cycles,
            _phantom: PhantomData,
        }
    }
}

impl<'a, V, T> Iterator for Z2Pairer<'a, V, T>
where
    V: Z2Vector + std::fmt::Debug,
    T: Iterator<Item=(usize, &'a V)>,
{
    type Item = (Persistence<usize>, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        self.cycles.next()
            .map(|(index, cycle)| {
                let boundary_pos = self.reducer.lowest_memo.get(&index).map(|pos| *pos);
                (Persistence(index, boundary_pos), cycle)
            })
    }
}
