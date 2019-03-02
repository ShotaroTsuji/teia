use std::fs::File;
use std::io::BufReader;
use teia::reader::simpcomp;
use teia::z2reduce::Z2ColumnReduce;
use teia::z2vector::{Z2Chain, Z2VectorVec, Z2VectorIter};
use teia::traits::*;
use teia::pair::Pair;

fn main() {
    let file = File::open("examples/torus.txt").unwrap();

    let comp = simpcomp::read_simpcomp_text(BufReader::new(file)).unwrap();

    println!("# Complex");
    for simp in comp.basis.iter() {
        println!("{:?}", simp);
    }

    let reduce = Z2ColumnReduce::<Z2Chain<Z2VectorVec>>
            ::from_complex_with(&comp, |index, chain| Z2Chain::new(index, chain)).unwrap();

    for (pers, chain) in Pair::new(&reduce, reduce.cycles()) {
        println!("{:?}", pers);
        for x in chain.chain.iter() {
            println!("  {}", comp.basis[*x]);
        }
    }
}
