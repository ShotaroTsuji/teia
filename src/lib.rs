pub mod simplex;
pub mod complex;
pub mod z2vector;
pub mod z2reduce;

pub mod reader;

/// Persistence pair
///
/// This struct represents persistence pair.
/// The first element is the birth filtration value and
/// the second element is the death filtration value.
/// If the second element is `None`, it means that the cycle
/// is an essential cycle.
#[derive(Debug, Clone, Copy)]
pub struct Persistence<T>(pub T, pub Option<T>);

/// Orientation of simplex
///
/// `Orientation` has two variants: `Positive` and `Negative`.
/// It implements `std::ops::Mul` and `std::ops::MulAssign` traits.
///
/// # Example
/// ```
/// use teia::Orientation;
/// 
/// let a = Orientation::Positive;
/// let b = Orientation::Negative;
/// assert_eq!(a*a, Orientation::Positive);
/// assert_eq!(b*b, Orientation::Positive);
/// assert_eq!(a*b, Orientation::Negative);
///
/// let mut a = Orientation::Positive;
/// a *= b;
/// assert_eq!(a, Orientation::Negative);
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Orientation {
    Positive = 1,
    Negative = -1,
}

impl Orientation {
    /// Flip the orientation
    ///
    /// This method returns `Orientation::Negative` for `Orientation::Positive`
    /// and vice versa.
    ///
    /// # Example
    /// ```
    /// use teia::Orientation;
    /// assert_eq!(Orientation::Positive.flip(), Orientation::Negative);
    /// assert_eq!(Orientation::Negative.flip(), Orientation::Positive);
    /// ```
    pub fn flip(&self) -> Orientation {
        match self {
            Orientation::Positive => Orientation::Negative,
            Orientation::Negative => Orientation::Positive,
        }
    }

    /// Convert a signed value into `Orientation`
    ///
    /// This method returns `Orientation::Positive` is `value` is positive
    /// and vice versa.
    /// If `value` is zero, it returns `None`.
    pub fn from_signed<T: num_traits::sign::Signed>(value: T) -> Option<Orientation> {
        if value.is_positive() {
            Some(Orientation::Positive)
        } else if value.is_negative() {
            Some(Orientation::Negative)
        } else {
            None
        }
    }

}

impl std::ops::Mul for Orientation {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Orientation::from_signed(self as isize * rhs as isize).unwrap()
    }
}

impl std::ops::MulAssign for Orientation {
    fn mul_assign(&mut self, rhs: Self) {
        let v = *self * rhs;
        *self = v;
    }
}

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
