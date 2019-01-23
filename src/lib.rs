pub mod simplex;
pub mod complex;
pub mod z2vector;
pub mod z2reduce;

#[derive(Debug, Clone, Copy)]
pub struct Persistence<T>(pub T, pub Option<T>);

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
