pub mod simplex;
pub mod complex;
//pub mod z2vector;
//pub mod z2reduce;

pub mod traits;
pub mod indexed_vec;
pub mod sign;

//pub mod reader;

/// Persistence pair
///
/// This struct represents persistence pair.
/// The first element is the birth filtration value and
/// the second element is the death filtration value.
/// If the second element is `None`, it means that the cycle
/// is an essential cycle.
#[derive(Debug, Clone, Copy)]
pub struct Persistence<T>(pub T, pub Option<T>);

trait IteratorExclude: Iterator
where
    Self: Sized,
{
    /// Exclude nth element in the iterator.
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
