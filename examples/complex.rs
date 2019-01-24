use teia::simplex;
use teia::complex::ComplexBuilder;
use teia::z2vector::Z2VecVector;

fn main() {
    let mut builder = ComplexBuilder::new();
    builder.push(simplex![+; 0]);
    builder.push(simplex![+; 1]);
    builder.push(simplex![+; 2]);
    builder.push(simplex![+; 1, 2]);
    builder.push(simplex![+; 0, 2]);
    builder.push(simplex![+; 0, 1]);
    builder.push(simplex![+; 0, 1, 2]);

    let comp = builder.build().unwrap();

    println!("Complex:");
    println!("{}", comp);
    println!("");

    println!("index_range method");
    println!("index_range = {:?}", comp.index_range());
    println!("");

    println!("get method");
    println!("get(0) = {:?}", comp.get(0));
    println!("get(10) = {:?}", comp.get(10));
    println!("");

    println!("range method");
    println!("range(index_range()) = ");
    for s in comp.range(comp.index_range()) {
        println!("    {:?}", s);
    }
    println!("range(0..3) = ");
    for s in comp.range(0..3) {
        println!("    {:?}", s);
    }
    println!("");

    println!("index method");
    println!("comp[1] = {:?}", comp[1]);
    println!("");

    println!("boundary method");
    println!("boundary(3) = {:?}", comp.boundary::<Z2VecVector>(3));
    println!("boundary(5) = {:?}", comp.boundary::<Z2VecVector>(5));
    println!("boundary(6) = {:?}", comp.boundary::<Z2VecVector>(6));
    println!("");

    println!("boundary_from method");
    println!("boundary_from(range(0..3), 3) = {:?}",
        comp.boundary_from::<Z2VecVector, _>(comp.range(0..3), 3));
    println!("");

    println!("boundaries method");
    for v in comp.boundaries::<Z2VecVector>() {
        println!("    {:?}", v);
    }
    println!("");

    println!("boundaries_from method");
    println!("boundaries_from(iter())");
    for v in comp.boundaries_from::<Z2VecVector, _>(comp.iter()) {
        println!("    {:?}", v);
    }
    println!("");

    println!("# Dimension-wise construction");
    let mut builder = ComplexBuilder::new();
    builder.push(simplex![+; 0]);
    builder.push(simplex![+; 1]);
    builder.push(simplex![+; 2]);

    let comp0 = builder.build().unwrap();

    let mut builder = ComplexBuilder::new();
    builder.base_index(comp0.index_range().end);


    println!("Complex of dimension 0");
    println!("{}", comp0);
}
