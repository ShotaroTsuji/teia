use teia::Persistence;
use teia::traits::*;
use teia::simplex::Simplex;
use teia::z2vector::{Z2Chain, Z2Vector, Z2VectorIter, Z2VectorVec};
use teia::z2reduce::Z2ColumnReduce;
use teia::pair::Pair;
use teia::reader;
use std::collections::BTreeMap;
use std::path::PathBuf;
use std::io::BufReader;
use std::fs::File;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(subcommand)]
    command: Command,
}

#[derive(Debug, StructOpt)]
enum Command {
    /// Compute the generators of homology groups.
    ///
    /// This subcommand computes the generators of homology groups of the given complex.
    /// The argument `INPUT` is the file path to the input file.
    /// The generators of each dimension are printed to the standard output.
    /// A generator is printed as a list of simplices in a line.
    #[structopt(name = "homology")]
    Homology(ComputeHomology),
    /// Compute the persistence of complex.
    ///
    /// This subcommand computes the persistent homology groups of the given complex.
    /// The argument `INPUT` is the file path to the input file.
    /// It prints birth index, death index and dimension separated by a whitespace.
    #[structopt(name = "persistence")]
    Persistence(ComputePersistence),
}

#[derive(Debug, StructOpt)]
struct ComputePersistence {
    /// Input file path
    #[structopt(name = "INPUT", parse(from_os_str))]
    input: PathBuf,
}

fn compute_persistence(cmd: ComputePersistence) {
    let file = File::open(cmd.input).unwrap();

    let comp = reader::simpcomp::read_simpcomp_text(BufReader::new(file)).unwrap();

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

#[derive(Debug, StructOpt)]
struct ComputeHomology {
    /// Input file path
    #[structopt(name = "INPUT", parse(from_os_str))]
    input: PathBuf,
}

fn compute_homology(cmd: ComputeHomology) {
    let file = File::open(cmd.input).unwrap();

    let comp = reader::simpcomp::read_simpcomp_text(BufReader::new(file)).unwrap();

    let reduce = Z2ColumnReduce::<Z2Chain<Z2VectorVec>>
            ::from_complex_with(&comp, |index, chain| Z2Chain::new(index, chain)).unwrap();

    let generators = Pair::new(&reduce, reduce.cycles())
        .filter(|pair| pair.0.is_essential())
        .map(|(_, chain)| {
            chain.chain
                .iter()
                .map(|index| &comp.basis[*index])
                .collect::<Vec<&Simplex>>()
        })
        .collect::<Vec<_>>();

    let mut gen_dict = BTreeMap::<usize, Vec<Vec<&Simplex>>>::new();

    for generator in generators.into_iter() {
        match gen_dict.get_mut(&generator[0].dimension()) {
            Some(vec) => {
                vec.push(generator);
            },
            None => {
                gen_dict.insert(generator[0].dimension(), vec![generator]);
            },
        }
    }

    for (dim, generators) in gen_dict.iter() {
        println!("# dim {}", dim);
        for gen in generators.iter() {
            print!("[");
            for simp in gen.iter() {
                print!("{},", simp);
            }
            println!("]");
        }
    }
}

fn main() {
    let opt = Opt::from_args();

    match opt.command {
        Command::Homology(cmd) => compute_homology(cmd),
        Command::Persistence(cmd) => compute_persistence(cmd),
    }
}
