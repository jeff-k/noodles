#![warn(missing_docs)]

//! **noodles-bed** handles the reading and writing of the BED (Browser Extensible Data) format.

pub mod io;
pub mod record;

pub use self::record::Record;

#[deprecated(since = "0.14.0", note = "Use `noodles_bed::io::Reader` instead.")]
pub use self::io::Reader;

#[deprecated(since = "0.14.0", note = "Use `noodles_bed::io::Writer` instead.")]
pub use self::io::Writer;
