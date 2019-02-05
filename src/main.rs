use teia::simplex::Simplex;
use teia::simplex;
use teia::Orientation;
/*
use teia::complex::ComplexBuilder;
use teia::z2vector::Z2VecVector;
use teia::z2vector::Z2Vector;
use teia::z2reduce::{Z2ColumnReduce, Z2Pair};
*/

use teia::indexed_vec::IndexedVec;
use teia::complex::Complex;

fn main() {
    let mut comp = Complex::new();

    comp.push(simplex![0]);
    comp.push(simplex![1]);
    comp.push(simplex![2]);

    println!("{:?}", comp);
    println!("{:?}", comp.get(0));
}

fn print_boundary(simp: Simplex) {
    println!("simplex: {}", simp);
    println!("dimension = {}", simp.dimension());
    println!("boudnary = ");
    for t in simp.boundary() {
        println!("  {}", t);
    }
}
