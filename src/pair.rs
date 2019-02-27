use crate::Persistence;
use crate::traits::LookupByLowest;
use std::marker::PhantomData;

pub struct Pair<'a, B, Z, C> {
    reduce: &'a B,
    cycles: Z,
    _phantom: (PhantomData<&'a B>, PhantomData<fn () -> C>),
}

impl<'a, B, Z, C> Pair<'a, B, Z, C>
where
    B: LookupByLowest,
    Z: Iterator<Item=(usize, C)>,
{
    pub fn new(reduce: &'a B, cycles: Z) -> Self {
        Pair {
            reduce: reduce,
            cycles: cycles,
            _phantom: (PhantomData, PhantomData),
        }
    }
}

impl<'a, B, Z, C> Iterator for Pair<'a, B, Z, C>
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
