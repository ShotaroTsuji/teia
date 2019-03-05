use std::fs::File;
use std::io::BufReader;
use teia::Persistence;
use teia::traits::*;
use teia::reader::simpcomp;
use teia::z2reduce::Z2ColumnReduce;
use teia::z2vector::{Z2Chain, Z2VectorVec, Z2Vector};
use teia::pair::Pair;

fn main() {
    let file = File::open("examples/torus.txt").unwrap();

    let comp = simpcomp::read_simpcomp_text(BufReader::new(file)).unwrap();

    let reduce = Z2ColumnReduce::<Z2Chain<Z2VectorVec>>
            ::from_complex_with(&comp, |index, chain| Z2Chain::new(index, chain)).unwrap();

    for (pers, chain) in Pair::new(&reduce, reduce.cycles()) {
        match pers {
            Persistence(birth, Some(death)) => {
                print!("{} {}", birth, death);
            },
            Persistence(birth, None) => {
                print!("{} inf", birth);
            },
        };
        println!(" {}", comp.basis[chain.chain.lowest().unwrap()].dimension());
    }
}
