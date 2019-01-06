use Vertex;
use Index;
use Orientation;
use simplex::Simplex;
use std::marker::PhantomData;

pub struct SimplicialComplexBuilder<V> {
    simplices: Vec<Simplex<V>>,
}

impl<V> SimplicialComplexBuilder<V>
where
    V: Vertex,
{
    pub fn new() -> Self {
        SimplicialComplexBuilder {
            simplices: Vec::new(),
        }
    }

    pub fn push(&mut self, simplex: Simplex<V>) {
        self.simplices.push(simplex);
    }

    pub fn build(self) -> Option<SimplicialComplex<V>> {
        for index in 1..self.simplices.len() {
            let result = check_boundary(&self.simplices[0..index], &self.simplices[index]);
            if result.is_none() {
                return None;
            }
        }
        Some(SimplicialComplex {
            simplices: self.simplices,
        })
    }

    pub fn build_unchecked(self) -> SimplicialComplex<V> {
        SimplicialComplex {
            simplices: self.simplices,
        }
    }
}

fn check_boundary<V, T>(simplices: T, simplex: &Simplex<V>) -> Option<()>
where
    V: Vertex,
    T: AsRef<[Simplex<V>]>,
{
    for t in simplex.boundary() {
        let pos = simplices.as_ref().iter().position(|s| s.vertices() == t.vertices());
        if pos.is_none() {
            return None;
        }
    }
    Some(())
}

pub struct SimplicialComplex<V> {
    simplices: Vec<Simplex<V>>,
}

impl<V> std::fmt::Display for SimplicialComplex<V>
where
    V: Vertex,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for i in 0..self.simplices.len() {
            write!(f, "{}: {}", i, self.simplices[i])?;
            write!(f, "{}", if i < self.simplices.len()-1 { "\n" } else { "" })?;
        }
        write!(f, "")
    }
}

impl<V> SimplicialComplex<V>
where
    V: Vertex,
{
    pub fn new() -> Self {
        SimplicialComplex{
            simplices: Vec::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.simplices.len()
    }

    pub fn enumerate_boundary<'a, I: Index>(&'a self, index: usize) -> EnumerateBoundary<'a, V, I> {
        EnumerateBoundary {
            simplices: &self.simplices[0..index],
            boundary: self.simplices[index].boundary(),
            _phantom1: PhantomData,
            _phantom2: PhantomData,
        }
    }
}

pub struct EnumerateBoundary<'a, V, I> {
    simplices: &'a [Simplex<V>],
    boundary: crate::simplex::Boundary<'a, V>,
    _phantom1: PhantomData<&'a Simplex<V>>,
    _phantom2: PhantomData<fn () -> I>,
}

impl<'a, V, I> Iterator for EnumerateBoundary<'a, V, I>
where
    V: Vertex,
    I: Index,
{
    type Item = (I, Orientation);

    fn next(&mut self) -> Option<Self::Item> {
        match self.boundary.next() {
            Some(face) => {
                let pos = self.simplices.iter().position(|t| t.vertices() == face.vertices()).unwrap();
                let sign = self.simplices[pos].orientation() * face.orientation();
                Some((num_traits::FromPrimitive::from_usize(pos).unwrap(), sign))
            },
            None => None,
        }
    }
}
