use teia::traits::*;
use teia::indexed_vec::IndexedVec;
use teia::simplex;
use teia::simplex::Simplex;
use teia::complex::Complex;
use teia::z2vector::*;
use teia::z2reduce::Z2ColumnReduce;
use teia::pair::Pair;

fn main() {
    println!("# All-in-one complex example");
    let mut comp: Complex<IndexedVec<_>, _> = Complex::new();

    comp.push(simplex![0]).unwrap();
    comp.push(simplex![1]).unwrap();
    comp.push(simplex![2]).unwrap();
    comp.push(simplex![0, 1]).unwrap();
    comp.push(simplex![0, 2]).unwrap();
    comp.push(simplex![1, 2]).unwrap();
    comp.push(simplex![0, 1, 2]).unwrap();

    println!("");
    println!("## Complex::boundaries()");
    for chain in comp.boundaries::<Z2VectorVec>() {
        println!("{:?}", chain);
    }

    println!("");

    //let reduce = Z2ColumnReduce::<Z2VectorVec>::from_complex(&comp).unwrap();
    let reduce = Z2ColumnReduce::<Z2Chain<Z2VectorVec>>
        ::from_complex_with(&comp, |index, image| Z2Chain::new(index, image))
        .unwrap();

    println!("{:?}", reduce);
    println!("");

    println!("## cycles");
    for c in reduce.cycles() {
        println!("{:?}", c);
    }
    println!("");

    println!("## Z2Pair");
    for pers in Pair::new(&reduce, reduce.cycles()) {
        println!("{:?}", pers);
    }

    println!("");

    println!("# Separated complex example");

    let mut comp0 = Complex::<IndexedVec<_>, Simplex>::new();
    comp0.push(simplex![0]).unwrap();
    comp0.push(simplex![1]).unwrap();
    comp0.push(simplex![2]).unwrap();
    comp0.push(simplex![3]).unwrap();

    let mut comp1 = Complex::<IndexedVec<_>, Simplex>::with_prev(&comp0);
    comp1.push(simplex![0, 1]).unwrap();
    comp1.push(simplex![0, 2]).unwrap();
    comp1.push(simplex![0, 3]).unwrap();
    comp1.push(simplex![1, 2]).unwrap();
    comp1.push(simplex![1, 3]).unwrap();
    comp1.push(simplex![2, 3]).unwrap();

    let mut comp2 = Complex::<IndexedVec<_>, Simplex>::with_prev(&comp1);
    comp2.push(simplex![0, 1, 2]).unwrap();
    comp2.push(simplex![0, 1, 3]).unwrap();
    comp2.push(simplex![0, 2, 3]).unwrap();
    comp2.push(simplex![1, 2, 3]).unwrap();

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

    let reduce0 = Z2ColumnReduce::<Z2VectorVec>::from_complex(&comp0).unwrap();

    println!("{:?}", reduce0);

    let reduce1 = Z2ColumnReduce::<Z2VectorVec>::from_complexes(&comp1, &comp0).unwrap();

    println!("{:?}", reduce1);

    let reduce2 = Z2ColumnReduce::<Z2VectorVec>::from_complexes(&comp2, &comp1).unwrap();

    println!("{:?}", reduce2);

    println!("");
    println!("## pairing of dim0 and dim1");
    let cycles0 = reduce0.into_cycles();
    for pers in Pair::new(&reduce1, cycles0.iter()) {
        println!("    {:?}", pers);
    }

    println!("");
    println!("## pairing of dim1 and dim2");
    let cycles1 = reduce1.into_cycles();
    for pers in Pair::new(&reduce2, cycles1.iter()) {
        println!("    {:?}", pers);
    }

    println!("");
    println!("## cycles of dim2");
    for pers in Pair::new(&reduce2, reduce2.cycles()) {
        println!("    {:?}", pers);
    }
}
