use std::fmt;
use std::ops::AddAssign;
use {Vector, Index, Orientation};

pub trait Z2Vector {
    type Index;

    fn lowest(&self) -> Option<&Self::Index>;
}

#[derive(Debug, Clone)]
pub struct Z2VecVector<T> {
    // the elements must be sorted in the descending order.
    vec: Vec<T>,
}

impl<T: Index> Z2VecVector<T> {
    pub fn is_valid(&self) -> bool {
        let mut prev = None;
        for x in self.vec.iter() {
            match (prev, x) {
                (Some(p), x) if p <= x => {
                    return false;
                }
                _ => {}
            }
            prev = Some(x);
        }
        true
    }
}

impl<T: Index> Z2Vector for Z2VecVector<T> {
    type Index = T;

    fn lowest(&self) -> Option<&T> {
        self.vec.get(0)
    }
}

impl<T: Index> Vector for Z2VecVector<T> {
    type Index = T;

    fn new() -> Self {
        Z2VecVector { vec: Vec::new() }
    }

    fn push_element(&mut self, index: T, _orientation: Orientation) {
        self.vec.push(index);
        self.vec.sort();
    }
}

impl<T: Index> From<Vec<T>> for Z2VecVector<T> {
    fn from(mut vec: Vec<T>) -> Z2VecVector<T> {
        vec.sort_by(|a, b| b.cmp(a));
        Z2VecVector { vec: vec }
    }
}

impl<'a, T: Index> AddAssign<&'a Z2VecVector<T>> for Z2VecVector<T> {
    fn add_assign(&mut self, other: &Z2VecVector<T>) {
        let mut result = Vec::new();
        let mut i = 0;
        let mut j = 0;
        loop {
            match (self.vec.get(i), other.vec.get(j)) {
                (None, None) => {
                    break;
                }
                (Some(x), None) => {
                    result.push(*x);
                    i += 1;
                }
                (None, Some(y)) => {
                    result.push(*y);
                    j += 1;
                }
                (Some(x), Some(y)) if x == y => {
                    i += 1;
                    j += 1;
                }
                (Some(x), Some(y)) if x > y => {
                    result.push(*x);
                    i += 1;
                }
                (Some(x), Some(y)) if x < y => {
                    result.push(*y);
                    j += 1;
                }
                _ => unreachable!(),
            }
        }
        *self = Z2VecVector { vec: result };
    }
}

impl<T: Index> PartialEq for Z2VecVector<T> {
    fn eq(&self, other: &Z2VecVector<T>) -> bool {
        if self.vec.len() != other.vec.len() {
            false
        } else {
            self.vec.iter().zip(other.vec.iter()).all(|(&x, &y)| x == y)
        }
    }
}
impl<T: Index> Eq for Z2VecVector<T> {}

impl<T> fmt::Display for Z2VecVector<T>
where
    T: Index + fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Z2VecVector[")?;
        for x in self.vec.iter() {
            write!(f, "{},", x)?;
        }
        write!(f, "]")
    }
}

#[cfg(test)]
mod tests {
    use z2vector::Z2VecVector;

    #[test]
    pub fn z2vecvec_eq() {
        let x: Z2VecVector<u64> = vec![0, 3, 4, 5, 8, 10, 12].into();
        let y: Z2VecVector<u64> = vec![0, 3, 4, 5, 8, 10, 12].into();
        let z: Z2VecVector<u64> = vec![1, 2, 6, 30].into();
        let zero = Z2VecVector::new();
        assert_eq!(true, x == y);
        assert_eq!(true, x != z);
        assert_eq!(false, x == zero);

        let x: Vec<usize> = (0..30).map(|x| x * 7).collect();
        let y: Vec<usize> = (0..30).map(|x| x * 11).collect();
        let x = Z2VecVector::from(x);
        let y = Z2VecVector::from(y);
        assert_eq!(true, x != y);
    }

    #[test]
    pub fn z2vecvec_addassign() {
        let mut x: Z2VecVector<u64> = vec![0, 2, 5, 6].into();
        let y: Z2VecVector<u64> = vec![1, 2, 5, 9, 11].into();
        let z: Z2VecVector<u64> = vec![0, 1, 6, 9, 11].into();
        let zero = Z2VecVector::new();
        x += &y;
        assert!(x.is_valid());
        assert_eq!(x, z);
        x += &z;
        assert!(x.is_valid());
        assert_eq!(x, zero);
        x += &y;
        assert!(x.is_valid());
        assert_eq!(x, y);
    }
}
