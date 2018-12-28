use std::marker::PhantomData;
use IteratorExclude;
use Orientation;
use Vertex;

#[derive(Debug, PartialEq, Clone)]
pub struct Simplex<V> {
    vertices: Vec<V>,
    orientation: Orientation,
}

impl<V: Vertex> std::fmt::Display for Simplex<V> {
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

impl<V: Vertex> Simplex<V> {
    pub fn new(mut vertices: Vec<V>, ori: Orientation) -> Simplex<V> {
        assert!(vertices.len() > 0);
        vertices.sort();
        Simplex {
            vertices: vertices,
            orientation: ori,
        }
    }

    pub fn dimension(&self) -> usize {
        self.vertices.len() - 1
    }

    pub fn orientation(&self) -> Orientation {
        self.orientation
    }

    pub fn is_face_of(&self, other: &Simplex<V>) -> bool {
        self.vertices
            .iter()
            .all(|v| other.vertices.binary_search(v).is_ok())
    }

    pub fn boundary(&self) -> Boundary<V> {
        Boundary {
            simplex: &self,
            index: 0,
            ori: self.orientation,
            _phantom: PhantomData,
        }
    }
}

pub struct Boundary<'a, V> {
    simplex: &'a Simplex<V>,
    index: usize,
    ori: Orientation,
    _phantom: PhantomData<&'a Simplex<V>>,
}

impl<'a, V> Iterator for Boundary<'a, V>
where
    V: Vertex,
{
    type Item = Simplex<V>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.simplex.vertices.len() {
            let boundary = self
                .simplex
                .vertices
                .iter()
                .exclude(self.index)
                .map(|v| v.clone())
                .collect::<Vec<V>>();
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
