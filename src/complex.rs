use crate::sign::Sign;
use crate::traits::{ChainGenerator, IndexedSet};

pub fn compute_boundary<'a, 'b: 'a, I, G, V>(iter: I, elem: &'b G) -> Option<V>
where
    G: 'a + ChainGenerator<'a> + PartialEq,
    I: Iterator<Item=(usize, &'a G)> + Clone,
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
        }).collect()
}
