use crate::sign::Sign;
use std::iter::FromIterator;

/// The trait describes the operations on Z2Vector
pub trait Z2Vector {
    fn lowest(&self) -> Option<usize>;

    #[inline]
    fn is_cycle(&self) -> bool {
        self.lowest().is_none()
    }

    fn add_assign(&mut self, other: &Self);
}

#[derive(Debug, Clone)]
pub struct Z2Chain<V> {
    chain: V,
    boundary: V,
}

#[derive(Debug, Clone)]
pub struct Z2VectorVec {
    // the elements must be sorted in the descending order.
    vec: Vec<usize>,
}

impl Z2VectorVec {
    pub fn new() -> Self {
        Z2VectorVec { vec: Vec::new() }
    }

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

impl Z2Vector for Z2VectorVec {
    #[inline]
    fn lowest(&self) -> Option<usize> {
        self.vec.get(0).cloned()
    }

    fn add_assign(&mut self, other: &Z2VectorVec) {
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
        *self = Z2VectorVec { vec: result };
    }
}

impl FromIterator<(usize, Sign)> for Z2VectorVec {
    fn from_iter<I: IntoIterator<Item = (usize, Sign)>>(iter: I) -> Self {
        let mut vec = Vec::new();
        for (pos, _sign) in iter {
            vec.push(pos);
        }
        vec.sort_by(|a, b| b.cmp(a));

        Z2VectorVec { vec: vec }
    }
}

impl From<Vec<usize>> for Z2VectorVec {
    fn from(mut vec: Vec<usize>) -> Z2VectorVec {
        vec.sort_by(|a, b| b.cmp(a));
        Z2VectorVec { vec: vec }
    }
}

impl PartialEq for Z2VectorVec {
    fn eq(&self, other: &Z2VectorVec) -> bool {
        if self.vec.len() != other.vec.len() {
            false
        } else {
            self.vec.iter().zip(other.vec.iter()).all(|(&x, &y)| x == y)
        }
    }
}
impl Eq for Z2VectorVec {}

impl std::fmt::Display for Z2VectorVec {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Z2VectorVec[")?;
        for x in self.vec.iter() {
            write!(f, "{},", x)?;
        }
        write!(f, "]")
    }
}

#[cfg(test)]
mod tests {
    use crate::z2vector::{Z2VectorVec, Z2Vector};

    #[test]
    pub fn z2vecvec_eq() {
        let x: Z2VectorVec = vec![0, 3, 4, 5, 8, 10, 12].into();
        let y: Z2VectorVec = vec![0, 3, 4, 5, 8, 10, 12].into();
        let z: Z2VectorVec = vec![1, 2, 6, 30].into();
        let zero = Z2VectorVec::new();
        assert_eq!(true, x == y);
        assert_eq!(true, x != z);
        assert_eq!(false, x == zero);

        let x: Vec<usize> = (0..30).map(|x| x * 7).collect();
        let y: Vec<usize> = (0..30).map(|x| x * 11).collect();
        let x = Z2VectorVec::from(x);
        let y = Z2VectorVec::from(y);
        assert_eq!(true, x != y);
    }

    #[test]
    pub fn z2vecvec_addassign() {
        let mut x: Z2VectorVec = vec![0, 2, 5, 6].into();
        let y: Z2VectorVec = vec![1, 2, 5, 9, 11].into();
        let z: Z2VectorVec = vec![0, 1, 6, 9, 11].into();
        let zero = Z2VectorVec::new();
        x.add_assign(&y);
        assert!(x.is_valid());
        assert_eq!(x, z);
        x.add_assign(&z);
        assert!(x.is_valid());
        assert_eq!(x, zero);
        x.add_assign(&y);
        assert!(x.is_valid());
        assert_eq!(x, y);
    }
}
