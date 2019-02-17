use crate::sign::Sign;
use crate::traits::ChainGenerator;
use crate::IteratorExclude;
use std::marker::PhantomData;

#[macro_export]
macro_rules! simplex {
    ($($x:expr),*) => (
        $crate::simplex::Simplex::new(vec![$($x),*])
    );
}

/// The struct represents simplex
///
/// This struct represents a simplex, which is a set of vertices with orientation.
#[derive(Debug, PartialEq, Clone)]
pub struct Simplex {
    /// The vertices ordered in descending order.
    vertices: Vec<usize>,
}

impl std::fmt::Display for Simplex {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "< ")?;
        for v in self.vertices.iter() {
            write!(f, "{} ", v)?;
        }
        write!(f, ">")
    }
}

impl Simplex {
    /// Create new simplex from vertices and orientation
    ///
    /// This method creates a new simplex from vertices and orientation.
    /// The vertices are sorted in descending order by this method.
    /// The order of vertices in `vertices` is ignored.
    pub fn new(mut vertices: Vec<usize>) -> Simplex {
        assert!(vertices.len() > 0);
        vertices.sort();
        Simplex { vertices: vertices }
    }
}

impl<'a> ChainGenerator<'a> for Simplex {
    type VerticesIter = Vertices<'a>;
    type BoundaryIter = Boundary<'a>;

    /// Returns the dimension of simplex
    ///
    /// # Example
    /// ```
    /// use teia::Orientation;
    /// use teia::simplex::Simplex;
    ///
    /// let s = Simplex::new(vec![0], Orientation::Positive);
    /// assert_eq!(s.dimension(), 0);
    ///
    /// let s = Simplex::new(vec![3, 4, 8, 10], Orientation::Positive);
    /// assert_eq!(s.dimension(), 3);
    /// ```
    fn dimension(&self) -> usize {
        self.vertices.len() - 1
    }

    /// Returns the reference to the slice of vertices
    fn vertices(&'a self) -> Vertices<'a> {
        Vertices {
            iter: self.vertices.iter(),
            _phantom: PhantomData,
        }
    }

    fn inner_prod(&self, other: &Simplex) -> Sign {
        if self.vertices == other.vertices {
            Sign::positive()
        } else {
            Sign::zero()
        }
    }

    /// Checks whether `self` is a face of `other`
    ///
    /// It returns true if `self` is a face of `other`
    /// and returns false if not.
    ///
    /// # Example
    /// ```
    /// use teia::Orientation;
    /// use teia::simplex::Simplex;
    ///
    /// let s = Simplex::new(vec![0,1,2,3], Orientation::Positive);
    /// let t = Simplex::new(vec![1,3], Orientation::Positive);
    /// assert_eq!(t.is_face_of(&s), true);
    /// ```
    fn is_face_of(&self, other: &Simplex) -> bool {
        self.vertices
            .iter()
            .all(|v| other.vertices.binary_search(v).is_ok())
    }

    /// Computes the boundary of simplex
    ///
    /// This method returns an iterator that computes the boundary of simplex.
    ///
    /// # Example
    /// ```
    /// use teia::simplex;
    ///
    /// let s = simplex![+; 0, 1, 2, 3];
    /// let mut b = s.boundary();
    /// assert_eq!(b.next(), Some(simplex![+; 1, 2, 3]));
    /// assert_eq!(b.next(), Some(simplex![-; 0, 2, 3]));
    /// assert_eq!(b.next(), Some(simplex![+; 0, 1, 3]));
    /// assert_eq!(b.next(), Some(simplex![-; 0, 1, 2]));
    /// assert_eq!(b.next(), None);
    /// ```
    fn boundary(&self) -> Boundary {
        Boundary {
            simplex: &self,
            index: 0,
            _phantom: PhantomData,
        }
    }
}

pub struct Vertices<'a> {
    iter: std::slice::Iter<'a, usize>,
    _phantom: PhantomData<&'a usize>,
}

impl<'a> Iterator for Vertices<'a> {
    type Item = &'a usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

pub struct Boundary<'a> {
    simplex: &'a Simplex,
    index: usize,
    _phantom: PhantomData<&'a Simplex>,
}

impl<'a> Iterator for Boundary<'a> {
    type Item = Simplex;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.simplex.vertices.len() {
            let boundary = self
                .simplex
                .vertices
                .iter()
                .exclude(self.index)
                .map(|v| v.clone())
                .collect::<Vec<usize>>();
            self.index += 1;
            if boundary.len() == 0 {
                None
            } else {
                Some(Simplex { vertices: boundary })
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::simplex::Simplex;
    use crate::Orientation;

    #[test]
    fn test_simplex_boundary() {
        let simplex = Simplex::new(vec![0, 1, 2, 3], Orientation::Positive);
        let boundary = simplex.boundary().collect::<Vec<Simplex>>();
        assert_eq!(
            boundary,
            vec![
                Simplex::new(vec![1, 2, 3], Orientation::Positive),
                Simplex::new(vec![0, 2, 3], Orientation::Negative),
                Simplex::new(vec![0, 1, 3], Orientation::Positive),
                Simplex::new(vec![0, 1, 2], Orientation::Negative)
            ]
        );
    }

    #[test]
    fn test_simplex_face() {
        let s: Simplex = Simplex::new(vec![0, 1, 2, 3], Orientation::Positive);
        let t: Simplex = Simplex::new(vec![0, 1, 3], Orientation::Positive);
        test_simplex_face_inner(&s, &t);

        let t: Simplex = Simplex::new(vec![0, 2, 3], Orientation::Positive);
        test_simplex_face_inner(&s, &t);
    }

    fn test_simplex_face_inner(s: &Simplex, t: &Simplex) {
        assert!(!s.is_face_of(t));
        assert!(t.is_face_of(s));
        assert!(s.is_face_of(s));
        assert!(t.is_face_of(t));
    }
}
