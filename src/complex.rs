use crate::simplex::Simplex;
use crate::Orientation;
use std::iter::FromIterator;
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct ComplexBuilder {
    simplices: Vec<Simplex>,
    base_index: usize,
}

impl ComplexBuilder {
    pub fn new() -> Self {
        ComplexBuilder {
            simplices: Vec::new(),
            base_index: 0,
        }
    }

    pub fn base_index(&mut self, base: usize) {
        self.base_index = base;
    }

    pub fn push(&mut self, simplex: Simplex) {
        self.simplices.push(simplex);
    }

    pub fn build(self) -> Option<Complex> {
        for index in 1..self.simplices.len() {
            let result = check_boundary(self.simplices[0..index].iter(), &self.simplices[index]);
            if result.is_none() {
                return None;
            }
        }
        Some(Complex {
            simplices: self.simplices,
            base_index: self.base_index,
        })
    }

    pub fn build_unchecked(self) -> Complex {
        Complex {
            simplices: self.simplices,
            base_index: self.base_index,
        }
    }
}

fn check_boundary<'a, T>(simplices: T, simplex: &Simplex) -> Option<()>
where
    T: Iterator<Item = &'a Simplex> + Clone,
{
    for t in simplex.boundary() {
        let pos = simplex_position(simplices.clone(), &t);
        if pos.is_none() {
            return None;
        }
    }
    Some(())
}

#[derive(Debug, Clone)]
pub struct Complex {
    simplices: Vec<Simplex>,
    base_index: usize,
}

impl std::fmt::Display for Complex {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for i in 0..self.simplices.len() {
            write!(f, "{}: {}", i, self.simplices[i])?;
            write!(
                f,
                "{}",
                if i < self.simplices.len() - 1 {
                    "\n"
                } else {
                    ""
                }
            )?;
        }
        write!(f, "")
    }
}

impl Complex {
    pub fn len(&self) -> usize {
        self.simplices.len()
    }

    pub fn index_range(&self) -> std::ops::Range<usize> {
        self.base_index..self.base_index + self.len()
    }

    /// Returns an iterator that returns simplices in the complex
    pub fn range<'a, U: Iterator<Item = usize>>(&'a self, range: U) -> Range<'a, U> {
        Range {
            complex: self,
            range: range,
            _phantom: PhantomData,
        }
    }

    pub fn iter<'a>(&'a self) -> Range<'a, std::ops::Range<usize>> {
        Range {
            complex: self,
            range: self.index_range(),
            _phantom: PhantomData,
        }
    }

    pub fn get(&self, index: usize) -> Option<&Simplex> {
        self.simplices.get(index - self.base_index)
    }

    pub fn boundary<I: FromIterator<(usize, Orientation)>>(&self, index: usize) -> I {
        self.boundary_from(self.range(0..index), index)
    }

    pub fn boundary_from<'a, I, J>(&self, iter: J, index: usize) -> I
    where
        I: FromIterator<(usize, Orientation)>,
        J: Iterator<Item = &'a Simplex> + Clone,
    {
        self[index]
            .boundary()
            .map(|face| {
                simplex_position(iter.clone(), &face)
                    .map(|pos| (pos, self[pos].orientation() * face.orientation()))
                    .unwrap()
            })
            .collect()
    }

    pub fn boundaries<'a, V: FromIterator<(usize, Orientation)>>(&'a self) -> Boundaries<'a, V> {
        Boundaries {
            complex: self,
            range: 0..self.simplices.len(),
            _phantom0: PhantomData,
            _phantom1: PhantomData,
        }
    }

    pub fn boundaries_from<'a, 'b, V, I>(&'a self, iter: I) -> BoundariesFrom<'a, 'b, V, I>
    where
        V: FromIterator<(usize, Orientation)>,
        I: Iterator<Item = &'b Simplex> + Clone,
    {
        BoundariesFrom {
            complex: self,
            iter: iter,
            range: 0..self.simplices.len(),
            _phantom0: PhantomData,
            _phantom1: PhantomData,
            _phantom2: PhantomData,
        }
    }
}

impl std::ops::Index<usize> for Complex {
    type Output = Simplex;

    fn index(&self, index: usize) -> &Self::Output {
        self.get(index).unwrap()
    }
}

/// Return the position of `simplex` in the iterator `iter`.
///
/// The comparision ignores the orientation of simplices.
fn simplex_position<'a, I>(mut iter: I, simplex: &Simplex) -> Option<usize>
where
    I: Iterator<Item = &'a Simplex>,
{
    iter.position(|t| t.vertices() == simplex.vertices())
}

#[derive(Debug, Clone)]
pub struct Boundaries<'a, V> {
    complex: &'a Complex,
    range: std::ops::Range<usize>,
    _phantom0: PhantomData<&'a Complex>,
    _phantom1: PhantomData<fn() -> V>,
}

impl<'a, V> Iterator for Boundaries<'a, V>
where
    V: FromIterator<(usize, Orientation)>,
{
    type Item = V;

    fn next(&mut self) -> Option<Self::Item> {
        self.range.next().map(|index| self.complex.boundary(index))
    }
}

#[derive(Debug, Clone)]
pub struct BoundariesFrom<'a, 'b, V, I> {
    complex: &'a Complex,
    iter: I,
    range: std::ops::Range<usize>,
    _phantom0: PhantomData<&'a Complex>,
    _phantom1: PhantomData<fn() -> V>,
    _phantom2: PhantomData<&'b Simplex>,
}

impl<'a, 'b, V, I> Iterator for BoundariesFrom<'a, 'b, V, I>
where
    V: FromIterator<(usize, Orientation)>,
    I: Iterator<Item = &'b Simplex> + Clone,
{
    type Item = V;

    fn next(&mut self) -> Option<Self::Item> {
        self.range.next().map(|index| self.complex.boundary_from(self.iter.clone(), index))
    }
}

#[derive(Debug, Clone)]
pub struct Range<'a, U> {
    complex: &'a Complex,
    range: U,
    _phantom: PhantomData<&'a Complex>,
}

impl<'a, U> Iterator for Range<'a, U>
where
    U: Iterator<Item = usize>,
{
    type Item = &'a Simplex;

    fn next(&mut self) -> Option<Self::Item> {
        self.range.next().and_then(|index| self.complex.get(index))
    }
}
