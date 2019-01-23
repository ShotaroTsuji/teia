extern crate teia;

use teia::simplex::Simplex;
use teia::Orientation;
use teia::complex::ComplexBuilder;
use teia::z2vector::Z2VecVector;
use teia::z2vector::Z2Vector;
use teia::z2reduce::{Z2ColumnReducer, Z2Pairer};

fn main() {
    let mut x = Z2VecVector::new();
    let y: Z2VecVector = (vec![0, 1, 5]).into();
    let z: Z2VecVector = vec![2, 4, 5, 6].into();
    println!("x = {}", x);
    println!("y = {}", y);
    println!("z = {}", z);
    println!("lowest of x = {:?}", x.lowest());
    println!("lowest of y = {:?}", y.lowest());
    println!("lowest of z = {:?}", z.lowest());
    println!("Add y to x");
    x.add_assign(&y);
    println!("x = {}", x);
    println!("Add z to x");
    x.add_assign(&z);
    println!("x = {}", x);
    println!("x is valid? => {}", x.is_valid());
    println!("x is y? => {}", x == y);
    println!("x is x? => {}", x == x);
    println!("");

    print_boundary(Simplex::new(vec![0,1,2], Orientation::Positive));
    print_boundary(Simplex::new(vec![0,1,2,3,4,5], Orientation::Positive));
    print_boundary(Simplex::new(vec![0,1,3,5], Orientation::Negative));
    print_boundary(Simplex::new(vec![5], Orientation::Negative));
    println!("");

    let s = Simplex::new(vec![0,1,2], Orientation::Positive);
    let t = Simplex::new(vec![0,2], Orientation::Positive);
    println!("s = {}", s);
    println!("t = {}", t);
    println!("s is face of t? => {}", s.is_face_of(&t));
    println!("t is face of s? => {}", t.is_face_of(&s));
    println!("s is face of s? => {}", s.is_face_of(&s));
    println!("");

    let mut builder = ComplexBuilder::new();
    builder.push(Simplex::new(vec![0], Orientation::Positive));
    builder.push(Simplex::new(vec![1], Orientation::Positive));
    builder.push(Simplex::new(vec![2], Orientation::Positive));
    builder.push(Simplex::new(vec![0,1], Orientation::Positive));
    builder.push(Simplex::new(vec![0,2], Orientation::Positive));
    builder.push(Simplex::new(vec![1,2], Orientation::Positive));
    builder.push(Simplex::new(vec![0,1,2], Orientation::Positive));
    let simpcomp = builder.build().unwrap();
    println!("{}", simpcomp);
    println!("");

    println!("Range: {:?}", simpcomp.range());
    for i in simpcomp.range() {
        let chain: Z2VecVector = simpcomp.boundary(i);
        println!("{}", chain);
    }

    println!("");

    println!("Run the reducer");
    let mut reducer = Z2ColumnReducer::<Z2VecVector>::new();

    for i in simpcomp.range() {
        reducer.push(simpcomp.boundary(i));
    }
    println!("");
    println!("{:?}", reducer);

    println!("");
    for c in reducer.cycles() {
        println!("{:?}", c);
    }
    println!("");

    let pairer = Z2Pairer::new(&reducer, reducer.cycles());
    for (pers, _) in pairer {
        println!("{:?}", pers);
    }
}

fn print_boundary(simp: Simplex) {
    println!("simplex: {}", simp);
    println!("dimension = {}", simp.dimension());
    println!("boudnary = ");
    for t in simp.boundary() {
        println!("  {}", t);
    }
}
