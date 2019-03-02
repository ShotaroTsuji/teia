use std::fs::File;
use std::io::BufReader;
use teia::reader::simpcomp;
use teia::z2reduce::Z2ColumnReduce;
use teia::z2vector::{Z2Chain, Z2VectorVec, Z2VectorIter};
use teia::pair::Pair;

fn main() {
    let file = File::open("examples/torus.txt").unwrap();

    let comp = simpcomp::read_simpcomp_text(BufReader::new(file)).unwrap();

    let reduce = Z2ColumnReduce::<Z2Chain<Z2VectorVec>>
            ::from_complex_with(&comp, |index, chain| Z2Chain::new(index, chain)).unwrap();

    println!("# persistence generator");
    for (pers, chain) in Pair::new(&reduce, reduce.cycles()) {
        print!("{:?} [", pers);
        for x in chain.chain.iter() {
            print!("{},", comp.basis[*x]);
        }
        println!("]");
    }
}
