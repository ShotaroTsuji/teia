use teia::simplex::Simplex;
use teia::simplex;
use teia::sign::Sign;
use teia::traits::ChainGenerator;

use teia::indexed_vec::IndexedVec;
use teia::complex::Complex;

fn test_simplex() {
    println!("# test_simplex");

    let s = simplex![0,1,2,3];
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

    let t = simplex![1,3];
    println!("t = {}", t);
    println!("t.is_face_of(&s) = {}", t.is_face_of(&s));
    println!("s.inner_prod(&t) = {}", s.inner_prod(&t));

    let u = simplex![1,4];
    println!("u = {}", u);
    println!("u.is_face_of(&s) = {}", u.is_face_of(&s));
}

fn main() {
    test_simplex();
    println!("");

    println!("# test_complex");
    let mut comp = Complex::new();

    comp.push(simplex![0]);
    comp.push(simplex![1]);
    comp.push(simplex![2]);

    println!("{:?}", comp);

    for i in 0..comp.len() {
        for j in 0..comp.len() {
            let si = comp.get(i).unwrap();
            let sj = comp.get(j).unwrap();
            println!("(s[{}], s[{}]) = {}", i, j, si.inner_prod(sj));
        }
        println!("");
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
