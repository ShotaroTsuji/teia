pub mod chaincomplex;
pub mod simplex;
pub mod simpcomp;
pub mod z2vector;
pub mod z2reduce;

pub trait Index: Ord + PartialEq + Copy + std::fmt::Debug + std::fmt::Display + num_traits::cast::FromPrimitive + num_traits::cast::ToPrimitive + num_traits::identities::Zero {
    #[inline]
    fn from_usize(n: usize) -> Self {
        num_traits::cast::FromPrimitive::from_usize(n)
            .expect("Index::from_usize")
    }

    #[inline]
    fn to_usize(&self) -> usize {
        num_traits::cast::ToPrimitive::to_usize(self)
            .expect("Index::to_usize")
    }

    #[inline]
    fn zero() -> Self {
        num_traits::zero()
    }
}

impl Index for u64 {}
impl Index for u32 {}
impl Index for usize {}

pub trait Vertex: Ord + PartialEq + Copy + std::fmt::Debug + std::fmt::Display {}

impl Vertex for u64 {}
impl Vertex for u32 {}
impl Vertex for usize {}

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
