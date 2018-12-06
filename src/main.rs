extern crate teia;

use teia::z2vector::Z2Vector;
use teia::z2vector::Z2VecVector;

fn main() {
    let mut x = Z2VecVector::<u64>::new();
    let y: Z2VecVector<u64> = (vec![0, 1, 5]).into();
    let z: Z2VecVector<u64> = vec![2, 4, 5, 6].into();
    println!("x = {}", x);
    println!("y = {}", y);
    println!("z = {}", z);
    println!("lowest of x = {:?}", x.lowest());
    println!("lowest of y = {:?}", y.lowest());
    println!("lowest of z = {:?}", z.lowest());
    println!("Add y to x");
    x += &y;
    println!("x = {}", x);
    println!("Add z to x");
    x += &z;
    println!("x = {}", x);
    println!("x is valid? => {}", x.is_valid());
    println!("x is y? => {}", x == y);
    println!("x is x? => {}", x == x);
}
