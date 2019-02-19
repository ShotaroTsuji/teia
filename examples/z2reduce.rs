use teia::traits::*;
use teia::indexed_vec::IndexedVec;
use teia::simplex;
use teia::simplex::Simplex;
use teia::complex;
use teia::complex::{Complex, BoundaryFacesPositions};
use teia::z2vector::Z2VectorVec;
use teia::z2reduce::Z2ColumnReduce;

fn main() {
    println!("# All-in-one complex example");
    let mut comp: Complex<IndexedVec<_>, _> = Complex::new();

    comp.push(simplex![0]);
    comp.push(simplex![1]);
    comp.push(simplex![2]);
    comp.push(simplex![0, 1]);
    comp.push(simplex![0, 2]);
    comp.push(simplex![1, 2]);
    comp.push(simplex![0, 1, 2]);

    println!("");
    println!("## Complex::boundaries()");
    for chain in comp.boundaries::<Z2VectorVec>() {
        println!("{:?}", chain);
    }

    println!("");

    let mut reduce = Z2ColumnReduce::new(comp.basis.index_start());
    comp.boundaries::<Z2VectorVec>()
        .map(|result| result.unwrap())
        .map(|(_index, chain)| chain)
        .for_each(|chain| reduce.push(chain));

    println!("{:?}", reduce);
    println!("");

    println!("# Separated complex example");

    let mut comp0 = Complex::<IndexedVec<_>, Simplex>::new();
    comp0.push(simplex![0]);
    comp0.push(simplex![1]);
    comp0.push(simplex![2]);
    comp0.push(simplex![3]);

    let mut comp1 = Complex::<IndexedVec<_>, Simplex>::with_prev(&comp0);
    comp1.push(simplex![0, 1]);
    comp1.push(simplex![0, 2]);
    comp1.push(simplex![0, 3]);
    comp1.push(simplex![1, 2]);
    comp1.push(simplex![1, 3]);
    comp1.push(simplex![2, 3]);

    let mut comp2 = Complex::<IndexedVec<_>, Simplex>::with_prev(&comp1);
    comp2.push(simplex![0, 1, 2]);
    comp2.push(simplex![0, 1, 3]);
    comp2.push(simplex![0, 2, 3]);
    comp2.push(simplex![1, 2, 3]);


    let mut comp3 = Complex::<IndexedVec<_>, Simplex>::with_prev(&comp2);
    comp3.push(simplex![0, 1, 2, 3]);

    println!("");
    println!("## Complex 0");
    for simp in comp0.basis.iter() {
        println!("{:?}", simp);
    }

    println!("");
    println!("## Complex 1");
    for simp in comp1.basis.iter() {
        println!("{:?}", simp);
    }

    println!("");
    println!("## Complex 2");
    for simp in comp2.basis.iter() {
        println!("{:?}", simp);
    }

    println!("");
    println!("## Complex 3");
    for simp in comp3.basis.iter() {
        println!("{:?}", simp);
    }

    println!("");
    println!("## Complex 1's boundaries from complex 0");
    for chain in comp1.boundaries_from::<Z2VectorVec, _>(&comp0) {
        println!("{:?}", chain);
    }

    println!("");
    println!("## Complex 2's boundaries from complex 1");
    for chain in comp2.boundaries_from::<Z2VectorVec, _>(&comp1) {
        println!("{:?}", chain);
    }

    println!("");
    println!("## Complex 3's boundaries from complex 2");
    for chain in comp3.boundaries_from::<Z2VectorVec, _>(&comp2) {
        println!("{:?}", chain);
    }
}
