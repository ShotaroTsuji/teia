use Vertex;
use Vector;
use Index;
use z2vector::{Z2Vector, Z2VecVector};
use simplex::Simplex;

pub struct SimplicialComplex<V> {
    simplices: Vec<Simplex<V>>,
}

impl<V> SimplicialComplex<V>
where
    V: Vertex,
{
    pub fn new() -> Self {
        SimplicialComplex{
            simplices: Vec::new(),
        }
    }

    pub fn push(&mut self, simplex: Simplex<V>) {
        self.simplices.push(simplex);
    }

    pub fn build(self) -> SimplicialComplex<V> {
        for i in 1..self.simplices.len() {
            let x: Option<Z2VecVector<usize>> = find_boundary(&self.simplices[0..i], &self.simplices[i]);
            println!("{:?}", x);
        }
        SimplicialComplex {
            simplices: self.simplices,
        }
    }
}

fn find_boundary<V, T, I, U>(simplices: T, simplex: &Simplex<V>) -> Option<U>
where
    V: Vertex,
    T: AsRef<[Simplex<V>]>,
    I: Index,
    U: Vector<Index=I>,
{
    println!("#find_boundary : {}", simplex);
    let mut result = U::new();
    for t in simplex.boundary() {
        let pos = simplices.as_ref().iter().position(|s| s.vertices() == t.vertices());
        match pos {
            Some(pos) => {
                let sign = simplices.as_ref()[pos].orientation() * t.orientation();
                println!("{} -> {}, {:?}", t, pos, sign);
                result.push_element(num_traits::FromPrimitive::from_usize(pos).unwrap(), sign);
            },
            None => { return None; },
        };
    }
    Some(result)
}
