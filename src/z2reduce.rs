use std::collections::BTreeMap;
use crate::Index;
use crate::z2vector::Z2Vector;
use num_traits::FromPrimitive;

#[derive(Debug)]
pub struct Z2Reducer<I, V> {
    reduced: Vec<V>,
    _phantom: std::marker::PhantomData<fn () -> I>,
}

impl<I, V> Z2Reducer<I, V>
where
    I: Index,
    V: Z2Vector + std::fmt::Debug,
{
    pub fn new() -> Z2Reducer<I, V> {
        Z2Reducer {
            reduced: Vec::new(),
            _phantom: std::marker::PhantomData,
        }
    }

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

        self.reduced.push(boundary);
    }
}
