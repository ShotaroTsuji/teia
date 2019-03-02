use teia::indexed_vec::IndexedVec;
use teia::traits::{IndexedSet, IndexedSetIters};

fn main() {
    let mut vec = IndexedVec::<usize>::new(10);
    for i in 0..10 {
        vec.push(i);
    }

    println!("{:?}", vec);

    println!("# iter");
    for v in vec.iter() {
        println!("{:?}", v);
    }

    println!("\n# range");
    for v in vec.range(0..30) {
        println!("{:?}", v);
    }
}
