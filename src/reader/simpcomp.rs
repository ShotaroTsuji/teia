use botao::text::{DataRecordReaderBuilder, DataBlockReader};
use crate::complex::Complex;
use crate::simplex::Simplex;
use crate::indexed_vec::IndexedVec;
use std::io::BufRead;

pub fn read_simpcomp_text<R: BufRead>(stream: R) -> Option<Complex<IndexedVec<Simplex>, Simplex>> {
    let block = {
        let reader = DataRecordReaderBuilder::new()
            .record_delimiter(b'\n')
            .field_delimiter(b' ')
            .build(stream);
        let mut reader = DataBlockReader::<usize, _>::new(reader);

        reader.next_block().unwrap().unwrap()
    };

    let mut complex = Complex::new();

    for v in block.into_iter() {
        let simplex = Simplex::new(v);
        complex.push(simplex).unwrap();
    }

    Some(complex)
}
