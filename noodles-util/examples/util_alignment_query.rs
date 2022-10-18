//! Queries an alignment file with a given region.
//!
//! The input must has an associated index.
//!
//! Reference sequences in the FASTA format are only required for CRAM inputs that require them.
//!
//! The result matches the output of `samtools view --no-PG --with-header [--reference <fasta-src>]
//! <src> <region>`.

use std::{
    env,
    io::{self, BufWriter},
};

use noodles_fasta::{self as fasta, repository::adapters::IndexedReader};
use noodles_sam::{self as sam, AlignmentWriter};
use noodles_util::alignment;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = env::args().skip(1);

    let src = args.next().expect("missing src");
    let region = args.next().expect("missing region").parse()?;
    let fasta_src = args.next();

    let mut builder = alignment::reader::Builder::default();

    if let Some(fasta_src) = fasta_src {
        let repository = fasta::indexed_reader::Builder::default()
            .build_from_path(fasta_src)
            .map(IndexedReader::new)
            .map(fasta::Repository::new)?;

        builder = builder.set_reference_sequence_repository(repository);
    }

    let mut reader = builder.build_from_path(src)?;
    let header = reader.read_header()?;

    let stdout = io::stdout();
    let handle = stdout.lock();
    let mut writer = sam::Writer::new(BufWriter::new(handle));

    writer.write_header(&header)?;

    for result in reader.query(&header, &region)? {
        let record = result?;
        writer.write_alignment_record(&header, &record)?;
    }

    Ok(())
}
