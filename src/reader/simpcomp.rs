use botao::text::{DataRecordReaderBuilder, DataBlockReader};
use crate::complex::{Complex, ComplexBuilder};
use crate::simplex::Simplex;
use crate::Orientation;
use std::io::BufRead;

pub fn read_simpcomp_text<R: BufRead>(stream: R) -> Option<Complex> {
    let reader = DataRecordReaderBuilder::new()
        .record_delimiter(b'\n')
        .field_delimiter(b' ')
        .build(stream);
    let mut reader = DataBlockReader::<usize, _>::new(reader);

    /*
    while let Some(block) = reader.next_block().unwrap() {
        println!("{:?}", block);
        let _ = reader.consume_blanks().unwrap();
    }
    */

    let block = reader.next_block().unwrap().unwrap();

    let mut builder = ComplexBuilder::new();
    for v in block.iter() {
        builder.push(Simplex::new(v.clone(), Orientation::Positive));
    }

    builder.build()
}
