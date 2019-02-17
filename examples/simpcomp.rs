use std::fs::File;
use std::io::BufReader;
use teia::reader::simpcomp;
use teia::z2reduce::{Z2ColumnReduce, Z2Pair};
use teia::z2vector::Z2VecVector;

fn main() {
    let file = File::open("examples/simpcomp1.txt").unwrap();

    let reduce: Z2ColumnReduce<Z2VecVector> = {
        let comp = simpcomp::read_simpcomp_text(BufReader::new(file)).unwrap();

        println!("Input complex:");
        println!("{}\n", comp);

        comp.boundaries().collect()
    };

    let pair = Z2Pair::new(&reduce, reduce.cycles());

    for (pers, _) in pair {
        println!("{:?}", pers);
    }
}
