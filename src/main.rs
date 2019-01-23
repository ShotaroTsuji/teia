extern crate teia;

use teia::simplex::Simplex;
use teia::Orientation;
use teia::complex::ComplexBuilder;
use teia::z2vector::Z2VecVector;
use teia::z2vector::Z2Vector;
use teia::z2reduce::{Z2ColumnReduce, Z2Pair};

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
    for v in simpcomp.boundaries::<Z2VecVector>() {
        println!("{}", v);
    }

    println!("");

    println!("Run the reduction");
    let reduce = simpcomp.boundaries().collect::<Z2ColumnReduce<Z2VecVector>>();

    println!("");
    println!("{:?}", reduce);

    println!("");
    for c in reduce.cycles() {
        println!("{:?}", c);
    }
    println!("");

    let pair = Z2Pair::new(&reduce, reduce.cycles());
    for (pers, _) in pair {
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
