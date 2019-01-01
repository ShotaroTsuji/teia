pub mod chaincomplex;
pub mod simplex;
pub mod simpcomp;
pub mod z2vector;

pub trait Index: Ord + Copy + std::fmt::Debug + std::fmt::Display + num_traits::cast::FromPrimitive {}

impl Index for u64 {}
impl Index for u32 {}
impl Index for usize {}

pub trait Vertex: Ord + Copy + std::fmt::Debug + std::fmt::Display {}

impl Vertex for u64 {}
impl Vertex for u32 {}
impl Vertex for usize {}

pub trait Vector {
    type Index : Index;
    fn new() -> Self;
    fn push_element(&mut self, index: Self::Index, orientation: Orientation);
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Orientation {
    Positive,
    Negative,
}

impl Orientation {
    pub fn flip(&self) -> Orientation {
        match self {
            Orientation::Positive => Orientation::Negative,
            Orientation::Negative => Orientation::Positive,
        }
    }
}

impl std::ops::Mul for Orientation {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Orientation::Positive, Orientation::Positive) => Orientation::Positive,
            (Orientation::Positive, Orientation::Negative) => Orientation::Negative,
            (Orientation::Negative, Orientation::Positive) => Orientation::Negative,
            (Orientation::Negative, Orientation::Negative) => Orientation::Positive,
        }
    }
}

trait IteratorExclude: Iterator
where
    Self: Sized,
{
    fn exclude(self, index: usize) -> Exclude<Self> {
        Exclude {
            iter: self,
            n: Some(index),
        }
    }
}

impl<'a, T> IteratorExclude for std::slice::Iter<'a, T> {}

struct Exclude<I> {
    iter: I,
    n: Option<usize>,
}

impl<I> Iterator for Exclude<I>
where
    I: Iterator,
{
    type Item = <I as Iterator>::Item;

    fn next(&mut self) -> Option<Self::Item> {
        match self.n {
            Some(n) if n == 0 => {
                self.n = None;
                let _ = self.iter.next();
                self.iter.next()
            }
            Some(n) => {
                self.n = Some(n - 1);
                self.iter.next()
            }
            None => self.iter.next(),
        }
    }
}
