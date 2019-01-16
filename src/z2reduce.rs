use std::collections::BTreeMap;
use crate::Index;
use crate::z2vector::Z2Vector;
use num_traits::{FromPrimitive, ToPrimitive};

#[derive(Debug)]
pub struct Z2Reducer<I, V> {
    reduced: Vec<V>,
    lowest_memo: BTreeMap<I, usize>,
}

impl<I, V> Z2Reducer<I, V>
where
    I: Index,
    V: Z2Vector<Index = I> + std::fmt::Debug,
{
    pub fn new() -> Z2Reducer<I, V> {
        Z2Reducer {
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
                .map(|pos| (FromPrimitive::from_usize(*pos).unwrap(), &self.reduced[*pos])))
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
}
