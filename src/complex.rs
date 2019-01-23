use crate::Orientation;
use crate::simplex::Simplex;
use std::iter::FromIterator;
use std::marker::PhantomData;

pub struct ComplexBuilder {
    simplices: Vec<Simplex>,
}

impl ComplexBuilder
{
    pub fn new() -> Self {
        ComplexBuilder {
            simplices: Vec::new(),
        }
    }

    pub fn push(&mut self, simplex: Simplex) {
        self.simplices.push(simplex);
    }

    pub fn build(self) -> Option<Complex> {
        for index in 1..self.simplices.len() {
            let result = check_boundary(&self.simplices[0..index], &self.simplices[index]);
            if result.is_none() {
                return None;
            }
        }
        Some(Complex {
            simplices: self.simplices,
        })
    }

    pub fn build_unchecked(self) -> Complex {
        Complex {
            simplices: self.simplices,
        }
    }
}

fn check_boundary<T>(simplices: T, simplex: &Simplex) -> Option<()>
where
    T: AsRef<[Simplex]>,
{
    for t in simplex.boundary() {
        let pos = simplex_position(simplices.as_ref().iter(), &t);
        if pos.is_none() {
            return None;
        }
    }
    Some(())
}

pub struct Complex {
    simplices: Vec<Simplex>,
}

impl std::fmt::Display for Complex
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for i in 0..self.simplices.len() {
            write!(f, "{}: {}", i, self.simplices[i])?;
            write!(f, "{}", if i < self.simplices.len()-1 { "\n" } else { "" })?;
        }
        write!(f, "")
    }
}

impl Complex
{
    pub fn len(&self) -> usize {
        self.simplices.len()
    }

    pub fn range(&self) -> std::ops::Range<usize> {
        0..self.len()
    }

    pub fn boundary<I: FromIterator<(usize, Orientation)>>(&self, index: usize) -> I {
        self.simplices[index].boundary()
            .map(|face| {
                let pos = simplex_position(self.simplices[0..index].iter(), &face).unwrap();
                let sign = self.simplices[pos].orientation() * face.orientation();
                (pos, sign)
            }).collect()
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
