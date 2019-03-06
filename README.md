# Teia

A persistent homology library and its command line interface.

This crate provides

- Simplicial complex
- Column reduction algorithm
- Persistence pairing algorithm

## Command line interface

The CLI program has two subcommands: `homology` and `persistence`.

## Subcommands

- `homology` : computes the generators of the homology groups of the given complex.
- `persistence` : computes the persistent homology groups of the given complex.

### Input file format

- Simplicial complex (text file) : A simplex is represented as numbers separated with whitespace. Each simplex is written in each line.
