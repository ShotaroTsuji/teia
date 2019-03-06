# Teia

A persistent homology library and its command line interface.

This crate provides

- Simplicial complex
- Column reduction algorithm
- Persistence pairing algorithm

## Command line interface

The CLI program has two subcommands: `homology` and `persistence`.

## How to build

First, if you not have Rust environment, you get it from [https://rustup.rs/](https://rustup.rs/) and install it.
Then you clone this repository with `git clone https://github.com/ShotaroTsuji/teia` and build it with `cargo build --release`.

### Subcommands

- `homology` : computes the generators of the homology groups of the given complex.
- `persistence` : computes the persistent homology groups of the given complex.

### Input file format

- Simplicial complex (text file) : A simplex is represented as numbers separated with whitespace. Each simplex is written in each line.

There is the example files `examples/torus.txt` and `examples/cube.txt`.

## Future work

- [ ] Construction of the filtration from given filtration values.
- [ ] Persistent cohomology algorithm.
- [ ] Lower-star complex.
- [ ] Vietoris-Rips complex.

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
