pub use self::reader::block::read_block;
pub use self::{
    bit_reader::BitReader, block::Block, compression_header::CompressionHeader,
    container::Container, data_series::DataSeries, encoding::Encoding, feature::Feature,
    flags::Flags, reader::Reader, record::Record, slice::Slice, tag::Tag,
};

mod bit_reader;
mod block;
mod compression_header;
mod container;
pub mod crai;
mod data_series;
mod encoding;
mod feature;
mod flags;
mod huffman;
mod num;
mod rans;
mod reader;
pub mod record;
mod slice;
mod tag;

use std::collections::HashMap;

use crate::num::Itf8;

pub type DataSeriesEncodingMap = HashMap<DataSeries, Encoding>;
pub type TagEncodingMap = HashMap<Itf8, Encoding>;
