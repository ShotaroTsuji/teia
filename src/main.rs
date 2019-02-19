use teia::complex;
use teia::complex::BoundaryFacesPositions;
use teia::complex::Complex;
use teia::indexed_vec::IndexedVec;
use teia::simplex;
use teia::simplex::Simplex;
use teia::traits::*;
use teia::z2vector::{Z2VecVector, Z2Vector};

fn test_simplex() {
    println!("# test_simplex");

    let s = simplex![0, 1, 2, 3];
    println!("s = {}", s);

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

fn test_complex() {
    println!("# test_complex");
    let mut comp: Complex<IndexedVec<_>, _> = Complex::new();

    comp.push(simplex![0]);
    comp.push(simplex![1]);
    comp.push(simplex![2]);
    comp.push(simplex![0, 1]);
    comp.push(simplex![0, 2]);
    comp.push(simplex![1, 2]);
    comp.push(simplex![0, 1, 2]);
    comp.push(simplex![0, 1, 2, 3]);

    println!("{:?}", comp);

    println!("Iterator");
    for t in comp.basis.iter() {
        println!("{:?}", t);
    }

    println!("Range(3..6)");
    for t in comp.basis.range(3..6) {
        println!("{:?}", t);
    }

    println!("compute_boundary");
    for index in comp.basis.index_range() {
        println!("  index = {}, range = {:?}", index, 0..index);
        let res: Option<Z2VecVector> =
            complex::compute_boundary(comp.basis.range(0..index), &comp.basis[index]);
        println!("  -> {:?}", res);
    }

    println!("BoundaryFacesPositions");
    for index in comp.basis.index_range() {
        println!("  index = {}, range = {:?}", index, 0..index);
        let iter = BoundaryFacesPositions::new(comp.basis.range(0..index), &comp.basis[index]);
        let res: Option<Z2VecVector> = iter.collect();
        println!("  -> {:?}", res);
    }

    println!("BoundaryFacesPositions");
    {
        let simp = simplex![0,1,2];
        let range = comp.basis.range(3..6);
        let iter = BoundaryFacesPositions::new(range, &simp);
        for item in iter {
            println!("  {:?}", item);
        }
    }

    println!("boundaries()");
    for chain in comp.boundaries::<Z2VecVector>() {
        println!("{:?}", chain);
    }
}

fn test_complex2() {
    let mut comp0 = Complex::<IndexedVec<_>, Simplex>::new();
    comp0.push(simplex![0]);
    comp0.push(simplex![1]);
    comp0.push(simplex![2]);

    let mut comp1 = Complex::<IndexedVec<_>, Simplex>::with_prev(&comp0);
    comp1.push(simplex![0,1]);
    comp1.push(simplex![0,2]);
    comp1.push(simplex![1,2]);

    let mut comp2 = Complex::<IndexedVec<_>, Simplex>::with_prev(&comp1);
    comp2.push(simplex![0,1,2]);

    println!("Complex 0 : {:?}", comp0);
    println!("Complex 1 : {:?}", comp1);
    println!("Complex 2 : {:?}", comp2);
    println!("Complex 1's boundaries from complex 0");
    for chain in comp1.boundaries_from::<Z2VecVector, _>(&comp0) {
        println!("    {:?}", chain);
    }
    println!("Complex 2's boundaries from complex 1");
    for chain in comp2.boundaries_from::<Z2VecVector, _>(&comp1) {
        println!("    {:?}", chain);
    }
}

fn main() {
    test_simplex();
    println!("");

    test_complex();
    println!("");

    test_complex2();
}
