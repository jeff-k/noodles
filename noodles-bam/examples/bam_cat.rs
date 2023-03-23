//! Concatenates BAM files.
//!
//! The result is similar to the output `samtools cat --no-PG <srcs...>`.

use std::{env, io};

use noodles_bam as bam;
use noodles_sam::AlignmentReader;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let srcs: Vec<_> = env::args().skip(1).collect();

    let first_src = srcs.first().expect("missing srcs[0]");
    let header = bam::reader::Builder::default()
        .build_from_path(first_src)
        .and_then(|mut reader| reader.read_alignment_header())?;

    let stdout = io::stdout().lock();
    let mut writer = bam::Writer::new(stdout);

    writer.write_header(&header)?;
    writer.write_reference_sequences(header.reference_sequences())?;

    for src in srcs {
        let mut reader = bam::reader::Builder::default().build_from_path(src)?;
        reader.read_header()?;
        reader.read_reference_sequences()?;

        io::copy(reader.get_mut(), writer.get_mut())?;
    }

    Ok(())
}