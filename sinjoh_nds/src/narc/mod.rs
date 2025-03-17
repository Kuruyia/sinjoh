//! Utils for working with the NARC file format.
//!
//! NARC, or Nitro ARChive, is a file format that's commonly used in Nintendo DS software to
//! regroup multiple files in a single file.
//! Basically, this is the Nintendo DS equivalent of a `.tar` file.

use thiserror::Error;

pub mod reader;

#[derive(Debug, Error)]
pub enum NarcByteOrderError {
    #[error("the provided byte order mark is invalid (provided value is {0:#?})")]
    InvalidBom([u8; 2]),
}

#[derive(Debug)]
pub enum NarcByteOrder {
    BigEndian = 0xFEFF,
    LittleEndian = 0xFFFE,
}

impl NarcByteOrder {
    pub fn from_bom(bom: &[u8; 2]) -> Result<Self, NarcByteOrderError> {
        match bom {
            [0xFE, 0xFF] => Ok(NarcByteOrder::BigEndian),
            [0xFF, 0xFE] => Ok(NarcByteOrder::LittleEndian),
            _ => Err(NarcByteOrderError::InvalidBom(*bom)),
        }
    }
}

#[derive(Debug)]
pub struct NarcFileAllocationTableEntry {
    start_address: u32,
    end_address: u32,
}

#[derive(Debug)]
pub struct NarcFileAllocationTableBlock {
    pub chunk_size: u32,
    pub number_of_files: u16,
    pub files: Vec<NarcFileAllocationTableEntry>,
}

#[derive(Debug)]
pub struct NarcFileNameTableBlock {
    pub chunk_size: u32,
}

#[derive(Debug)]
pub struct NarcFileImageBlock {
    pub chunk_size: u32,
    pub img_position: u64,
}

#[derive(Debug)]
pub struct NarcHeader {
    pub byte_order: Option<NarcByteOrder>,
    pub version: u16,
    pub file_size: u32,
    pub narc_header_size: u16,
    pub number_of_chunks: u16,
    pub fat: Option<NarcFileAllocationTableBlock>,
    pub fnt: Option<NarcFileNameTableBlock>,
    pub files: Option<NarcFileImageBlock>,
}
