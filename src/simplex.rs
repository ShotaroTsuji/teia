use std::marker::PhantomData;
use crate::{IteratorExclude, Orientation};

#[macro_export]
macro_rules! simplex {
    (+; $($x:expr),*) => (
        $crate::simplex::Simplex::new(vec![$($x),*], $crate::Orientation::Positive)
    );
    (-; $($x:expr),*) => (
        $crate::simplex::Simplex::new(vec![$($x),*], $crate::Orientation::Negative)
    )
}

/// The struct represents simplex
///
/// This struct represents a simplex, which is a set of vertices with orientation.
#[derive(Debug, PartialEq, Clone)]
pub struct Simplex {
    /// The vertices ordered in descending order.
    vertices: Vec<usize>,
    orientation: Orientation,
}

impl std::fmt::Display for Simplex {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.orientation {
            Orientation::Positive => { write!(f, "+")?; },
            Orientation::Negative => { write!(f, "-")?; },
        }
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
    pub fn new(mut vertices: Vec<usize>, ori: Orientation) -> Simplex {
        assert!(vertices.len() > 0);
        vertices.sort();
        Simplex {
            vertices: vertices,
            orientation: ori,
        }
    }

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
    pub fn dimension(&self) -> usize {
        self.vertices.len() - 1
    }

    /// Returns the orientation of simplex
    pub fn orientation(&self) -> Orientation {
        self.orientation
    }

    /// Returns the reference to the slice of vertices
    pub fn vertices(&self) -> &[usize] {
        &self.vertices[..]
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
    pub fn is_face_of(&self, other: &Simplex) -> bool {
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
    pub fn boundary(&self) -> Boundary {
        Boundary {
            simplex: &self,
            index: 0,
            ori: self.orientation,
            _phantom: PhantomData,
        }
    }
}

pub struct Boundary<'a> {
    simplex: &'a Simplex,
    index: usize,
    ori: Orientation,
    _phantom: PhantomData<&'a Simplex>,
}

impl<'a> Iterator for Boundary<'a>
{
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
            let orientation = self.ori;
            self.index += 1;
            self.ori = self.ori.flip();
            if boundary.len() == 0 {
                None
            } else {
                Some(Simplex {
                    vertices: boundary,
                    orientation: orientation,
                })
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
        let simplex = Simplex::new(vec![0,1,2,3], Orientation::Positive);
        let boundary = simplex.boundary().collect::<Vec<Simplex>>();
        assert_eq!(boundary,
                   vec![Simplex::new(vec![1,2,3], Orientation::Positive),
                        Simplex::new(vec![0,2,3], Orientation::Negative),
                        Simplex::new(vec![0,1,3], Orientation::Positive),
                        Simplex::new(vec![0,1,2], Orientation::Negative)]);
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
