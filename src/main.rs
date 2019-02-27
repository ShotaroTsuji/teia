use teia::complex;
use teia::complex::BoundaryFacesPositions;
use teia::complex::Complex;
use teia::indexed_vec::IndexedVec;
use teia::simplex;
use teia::simplex::Simplex;
use teia::traits::*;
use teia::z2vector::{Z2Vector, Z2VectorVec};

fn test_simplex() {
    println!("# test_simplex");

    let s = simplex![0, 1, 2, 3];
    println!("## Debug print");
    println!("    {}", s);

    println!("s.dimension() = {}", s.dimension());
    println!("s.vertices() :");
    for v in s.vertices() {
        println!("  {}", v);
    }
    println!("s.boundary() :");
    for t in s.boundary() {
        println!("  {}", t);
    }

    let t = simplex![1, 3];
    println!("t = {}", t);
    println!("t.is_face_of(&s) = {}", t.is_face_of(&s));
    println!("s.inner_prod(&t) = {}", s.inner_prod(&t));

    let u = simplex![1, 4];
    println!("u = {}", u);
    println!("u.is_face_of(&s) = {}", u.is_face_of(&s));
}

fn main() {
    test_simplex();
}
