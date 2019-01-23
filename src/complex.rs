use crate::Orientation;
use crate::simplex::Simplex;
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
        let pos = simplices.as_ref().iter().position(|s| s.vertices() == t.vertices());
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

    pub fn enumerate_boundary<'a>(&'a self, index: usize) -> EnumerateBoundary<'a> {
        EnumerateBoundary {
            simplices: &self.simplices[0..index],
            boundary: self.simplices[index].boundary(),
            _phantom1: PhantomData,
        }
    }
}

pub struct EnumerateBoundary<'a> {
    simplices: &'a [Simplex],
    boundary: crate::simplex::Boundary<'a>,
    _phantom1: PhantomData<&'a Simplex>,
}

impl<'a> Iterator for EnumerateBoundary<'a>
{
    type Item = (usize, Orientation);

    fn next(&mut self) -> Option<Self::Item> {
        match self.boundary.next() {
            Some(face) => {
                let pos = self.simplices.iter().position(|t| t.vertices() == face.vertices()).unwrap();
                let sign = self.simplices[pos].orientation() * face.orientation();
                Some((pos, sign))
            },
            None => None,
        }
    }
}
