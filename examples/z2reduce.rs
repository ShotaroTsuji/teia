use teia::traits::*;
use teia::indexed_vec::IndexedVec;
use teia::simplex;
use teia::simplex::Simplex;
use teia::complex::Complex;
use teia::z2vector::*;
use teia::z2reduce::{Z2ColumnReduce, Z2Pair};

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

    //let reduce = Z2ColumnReduce::<Z2VectorVec>::from_complex(&comp).unwrap();
    let reduce = Z2ColumnReduce::<Z2Chain<Z2VectorVec>>
        ::from_complex_with(&comp, |index, image| Z2Chain::new(index, image))
        .unwrap();

    /*
    let mut reduce = Z2ColumnReduce::new(comp.basis.index_start());
    comp.boundaries::<Z2VectorVec>()
        .map(|result| result.unwrap())
        .map(|(index, image)| Z2Chain::new(index, image))
        .for_each(|chain| reduce.push(chain));
    */

    println!("{:?}", reduce);
    println!("");

    println!("## cycles");
    for c in reduce.cycles() {
        println!("{:?}", c);
    }
    println!("");

    println!("## Z2Pair");
    let pair = Z2Pair::new(&reduce, reduce.cycles());
    for pers in pair {
        println!("{:?}", pers);
    }

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

    /*
    let mut reduce0 = Z2ColumnReduce::new(comp0.basis.index_start());
    comp0.boundaries::<Z2VectorVec>()
        .map(|result| result.unwrap())
        .map(|(index, image)| Z2Chain::new(index, image))
        .for_each(|chain| reduce0.push(chain));
    */
    let reduce0 = Z2ColumnReduce::<Z2VectorVec>::from_complex(&comp0).unwrap();

    println!("{:?}", reduce0);

    /*
    let mut reduce1 = Z2ColumnReduce::new(comp1.basis.index_start());
    comp1.boundaries_from::<Z2VectorVec, _>(&comp0)
        .map(|result| result.unwrap())
        .map(|(index, image)| Z2Chain::new(index, image))
        .for_each(|chain| reduce1.push(chain));
    */
    let reduce1 = Z2ColumnReduce::<Z2VectorVec>::from_complexes(&comp1, &comp0).unwrap();

    println!("{:?}", reduce1);

    /*
    let mut reduce2 = Z2ColumnReduce::new(comp2.basis.index_start());
    comp2.boundaries_from::<Z2VectorVec, _>(&comp1)
        .map(|result| result.unwrap())
        .map(|(index, image)| Z2Chain::new(index, image))
        .for_each(|chain| reduce2.push(chain));
    */
    let reduce2 = Z2ColumnReduce::<Z2VectorVec>::from_complexes(&comp2, &comp1).unwrap();

    println!("{:?}", reduce2);

    println!("");
    println!("## pairing of dim0 and dim1");
    let pair1 = Z2Pair::new(&reduce1, reduce0.cycles());
    for pers in pair1 {
        println!("    {:?}", pers);
    }

    println!("");
    println!("## pairing of dim1 and dim2");
    let pair2 = Z2Pair::new(&reduce2, reduce1.cycles());
    for pers in pair2 {
        println!("    {:?}", pers);
    }

    println!("");
    println!("## cycles of dim2");
    for cyc in reduce2.cycles() {
        println!("    {:?}", cyc);
    }
}
