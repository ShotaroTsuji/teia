use crate::sign::Sign;
use crate::traits::{ChainGenerator, IndexedSet};

#[derive(Debug,Clone)]
pub struct Complex<'a, V, G> {
    pub basis: V,
    _phantom: std::marker::PhantomData<&'a G>,
}

impl<'a, V, G> Complex<'a, V, G>
where
    G: 'a,
    V: IndexedSet<'a, G>,
{
    pub fn new() -> Self {
        Complex {
            basis: V::new(0),
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn push(&mut self, elem: G) {
        self.basis.push(elem);
    }
}

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
