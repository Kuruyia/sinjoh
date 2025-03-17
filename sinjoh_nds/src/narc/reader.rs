//! NARC file reader.
//!
//! For more information, see [`NarcReader`].

use std::fs::File;
use std::io::{self, BufReader, Read, Seek, SeekFrom};
use std::path::Path;

use byteorder::{BigEndian, LittleEndian, ReadBytesExt};
use log::warn;
use thiserror::Error;

use super::{
    NarcByteOrder, NarcFileAllocationTableBlock, NarcFileImageBlock, NarcFileNameTableBlock,
    NarcHeader,
};

/// The magic number at the beginning of a NARC chunk. Corresponds to "NARC" in little-endian.
pub const NARC_MAGIC: u32 = 0x4352414E;

/// The magic number at the beginning of a File Allocation Table Block chunk. Corresponds to "BTAF"
/// in little-endian.
pub const FATB_MAGIC: u32 = 0x46415442;

/// The magic number at the beginning of a File Name Table Block chunk. Corresponds to "BTNF" in
/// little-endian.
pub const FNTB_MAGIC: u32 = 0x464E5442;

/// The magic number at the beginning of a File Image Block chunk. Corresponds to "GMIF" in
/// little-endian.
pub const FIMG_MAGIC: u32 = 0x46494D47;

/// Error type for NARC file parsing.
#[derive(Error, Debug)]
pub enum NarcReaderError {
    /// An I/O error has occurred while trying to open the NARC file.
    #[error("unable to open the NARC file ({0})")]
    FileOpenError(#[source] io::Error),

    /// An I/O error has occurred while trying to read from the NARC file.
    #[error("failed to read the NARC file ({0})")]
    FileReadError(#[source] io::Error),

    /// An I/O error has occurred while trying to seek in the NARC file.
    #[error("failed to seek the NARC file ({0})")]
    FileSeekError(#[source] io::Error),

    /// An I/O error has occurred while trying to get the stream position of the NARC file.
    #[error("failed to get the stream position of the NARC file ({0})")]
    FileStreamPositionError(#[source] io::Error),

    /// The NARC file has an incorrect magic number.
    #[error("wrong NARC magic number (expected 0x{NARC_MAGIC:X}, found 0x{0:X})")]
    WrongNarcMagic(u32),

    /// Unable to determine the byte order from the BOM.
    #[error("unable to determine byte order from the BOM (found 0x{0:#X?})")]
    UnknownBom([u8; 2]),

    /// The NARC file does not have a File Allocation Table Block.
    #[error("this NARC does not have a File Allocation Table Block")]
    FatBlockNotFound,

    /// The NARC file does not have a File Image Block.
    #[error("this NARC does not have a File Image Block")]
    FimgBlockNotFound,

    /// The file at the specified index could not be found.
    #[error("the file at index {0} could not be found")]
    FileNotFound(u16),

    /// The file at the specified index is too large to be handled.
    #[error("the file at index {0} is too large to be handled (size is {1})")]
    FileTooLarge(u16, u32),
}

/// An iterator over the files in a NARC file.
#[derive(Debug)]
pub struct NarcReaderFilesIter<'a> {
    /// The current index of the iterator.
    curr: u16,

    /// The NARC reader.
    narc_reader: &'a mut NarcReader,
}

impl Iterator for NarcReaderFilesIter<'_> {
    type Item = Result<Vec<u8>, NarcReaderError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr >= self.narc_reader.number_of_files() {
            return None;
        }

        let file = self.narc_reader.get_file(self.curr);
        self.curr += 1;

        Some(file)
    }
}

impl<'a> NarcReaderFilesIter<'a> {
    /// Creates a new iterator over the files in a NARC file.
    ///
    /// See [`NarcReader::files_iter`] for usage.
    pub fn new(narc_reader: &'a mut NarcReader) -> Self {
        Self {
            curr: 0,
            narc_reader,
        }
    }
}

/// Flags that can be used to configure the behavior of the NARC reader.
#[derive(Debug, Default)]
pub struct NarcReaderFlags {
    /// Whether to skip the magic number check at the beginning of the NARC file.
    skip_narc_magic_number_check: bool,

    /// Whether to skip the BOM check at the beginning of the NARC file.
    skip_bom_check: bool,
}

/// A reader for NARC files.
///
/// This reader reads the header of a NARC file and keeps it in memory. The actual files contained
/// in the NARC are lazily-loaded as needed.
///
/// ## Reading a NARC from the file system
///
/// The NARC reader allows reading the contents of a NARC file that is directly stored on the
/// file system of your machine.
///
/// Please note that the file will stay open as long as the reader is in scope.
///
/// Here's an example on how to use it:
///
/// ```
/// use sinjoh_nds::narc::reader::{NarcReader, NarcReaderFlags};
///
/// let mut narc_reader =
///     NarcReader::read_from_file("/path/to/archive.narc", NarcReaderFlags::default())?;
///
/// for file in narc_reader.files_iter() {
///     println!("{:#?}", file);
/// }
/// ```
#[derive(Debug)]
pub struct NarcReader {
    /// The buffered reader for the NARC file.
    reader: BufReader<File>,

    /// The parsed header of the NARC file.
    narc_header: NarcHeader,
}

impl NarcReader {
    /// Creates a new NARC reader from the given file.
    ///
    /// Flags can be used to configure the behavior of the reader.
    ///
    /// The file will stay open until the reader is dropped. This allows for lazy-loading of the
    /// files contained in the NARC.
    pub fn read_from_file<P: AsRef<Path>>(
        path: P,
        flags: NarcReaderFlags,
    ) -> Result<Self, NarcReaderError> {
        let file = File::open(path).map_err(NarcReaderError::FileOpenError)?;
        let mut reader = BufReader::new(file);

        let narc = Self::read_header(&mut reader, &flags)?;
        Ok(Self {
            reader,
            narc_header: narc,
        })
    }

    /// Reads the header of the NARC file.
    fn read_header(
        reader: &mut BufReader<File>,
        flags: &NarcReaderFlags,
    ) -> Result<NarcHeader, NarcReaderError> {
        // Read the magic value
        if !flags.skip_narc_magic_number_check {
            let magic = reader
                .read_u32::<LittleEndian>()
                .map_err(NarcReaderError::FileReadError)?;

            if magic != NARC_MAGIC {
                return Err(NarcReaderError::WrongNarcMagic(magic));
            }
        }

        // Read the BOM
        let mut bom = [0; 2];
        reader
            .read(&mut bom)
            .map_err(NarcReaderError::FileReadError)?;

        let byte_order = {
            let byte_order = NarcByteOrder::from_bom(&bom);

            if !flags.skip_bom_check && byte_order.is_err() {
                return Err(NarcReaderError::UnknownBom(bom));
            }

            byte_order.ok()
        };

        // Read the version
        let version = match byte_order {
            Some(NarcByteOrder::LittleEndian) => reader.read_u16::<LittleEndian>(),
            _ => reader.read_u16::<BigEndian>(),
        }
        .map_err(NarcReaderError::FileReadError)?;

        // Read the file size
        let file_size = reader
            .read_u32::<LittleEndian>()
            .map_err(NarcReaderError::FileReadError)?;

        // Read the chunk size
        let narc_header_size = reader
            .read_u16::<LittleEndian>()
            .map_err(NarcReaderError::FileReadError)?;

        // Read the number of chunks
        let number_of_chunks = reader
            .read_u16::<LittleEndian>()
            .map_err(NarcReaderError::FileReadError)?;

        // Read the chunks
        let mut narc_header = NarcHeader {
            byte_order,
            version,
            file_size,
            narc_header_size,
            number_of_chunks,
            fat: Option::None,
            fnt: Option::None,
            files: Option::None,
        };

        Self::read_chunks(reader, &mut narc_header)?;

        Ok(narc_header)
    }

    /// Reads the chunks of the NARC file.
    ///
    /// This reads and parses the `FATB`, `FNTB`, and `FIMG` chunks.
    pub fn read_chunks(
        reader: &mut BufReader<File>,
        narc_header: &mut NarcHeader,
    ) -> Result<(), NarcReaderError> {
        for _ in 0..narc_header.number_of_chunks {
            // Prepare reading the current chunk
            let start_pos = reader
                .stream_position()
                .map_err(NarcReaderError::FileStreamPositionError)?;

            let chunk_magic = reader
                .read_u32::<LittleEndian>()
                .map_err(NarcReaderError::FileReadError)?;

            let chunk_size = reader
                .read_u32::<LittleEndian>()
                .map_err(NarcReaderError::FileReadError)?;

            // Check which type of chunk this is
            match chunk_magic {
                FATB_MAGIC => {
                    narc_header.fat = Some(Self::read_fatb_chunk(reader, chunk_size)?);
                }
                FNTB_MAGIC => {
                    narc_header.fnt = Some(Self::read_fntb_chunk(reader, chunk_size)?);
                }
                FIMG_MAGIC => {
                    // The actual files are lazy-loaded as needed
                    let img_position = reader
                        .stream_position()
                        .map_err(NarcReaderError::FileStreamPositionError)?;

                    narc_header.files = Some(NarcFileImageBlock {
                        chunk_size,
                        img_position,
                    });
                }
                _ => warn!(
                    "Encountered unknown chunk in NARC: magic = 0x{:X}, size = {}",
                    chunk_magic, chunk_size
                ),
            }

            // Go to the next chunk
            reader
                .seek(SeekFrom::Start(start_pos + chunk_size as u64))
                .map_err(NarcReaderError::FileSeekError)?;
        }

        Ok(())
    }

    /// Reads a File Allocation Table Block chunk.
    /// This chunk contains the file allocation table, which specifies the location of each file in
    /// the NARC.
    fn read_fatb_chunk(
        reader: &mut BufReader<File>,
        chunk_size: u32,
    ) -> Result<NarcFileAllocationTableBlock, NarcReaderError> {
        // Read the number of files
        let number_of_files = reader
            .read_u16::<LittleEndian>()
            .map_err(NarcReaderError::FileReadError)?;

        // Skip the reserved field
        reader
            .seek_relative(2)
            .map_err(NarcReaderError::FileSeekError)?;

        // Read all FAT entries
        let mut files = Vec::with_capacity(number_of_files.into());

        for _ in 0..number_of_files {
            let start_address = reader
                .read_u32::<LittleEndian>()
                .map_err(NarcReaderError::FileReadError)?;

            let end_address = reader
                .read_u32::<LittleEndian>()
                .map_err(NarcReaderError::FileReadError)?;

            files.push(super::NarcFileAllocationTableEntry {
                start_address,
                end_address,
            });
        }

        Ok(NarcFileAllocationTableBlock {
            chunk_size,
            number_of_files,
            files,
        })
    }

    /// Reads a File Name Table Block chunk.
    /// This chunk contains the file name table, which specifies the names of each file in the
    /// NARC.
    fn read_fntb_chunk(
        _reader: &mut BufReader<File>,
        chunk_size: u32,
    ) -> Result<NarcFileNameTableBlock, NarcReaderError> {
        // TODO: Need an example to implement this correctly
        Ok(NarcFileNameTableBlock { chunk_size })
    }

    /// Returns the parsed header of the NARC file.
    pub fn narc_header(&self) -> &NarcHeader {
        &self.narc_header
    }

    /// Returns the number of files in the NARC file.
    ///
    /// Convenience method for getting the number of files from the FAT block.
    pub fn number_of_files(&self) -> u16 {
        self.narc_header
            .fat
            .as_ref()
            .map_or(0, |fat| fat.number_of_files)
    }

    /// Reads and returns the file at the specified index.
    pub fn get_file(&mut self, index: u16) -> Result<Vec<u8>, NarcReaderError> {
        // Get where the file is located in the NARC
        let fat = self
            .narc_header
            .fat
            .as_ref()
            .ok_or(NarcReaderError::FatBlockNotFound)?;

        let fat_entry = &fat
            .files
            .get(index as usize)
            .ok_or(NarcReaderError::FileNotFound(index))?;

        let files = self
            .narc_header
            .files
            .as_ref()
            .ok_or(NarcReaderError::FimgBlockNotFound)?;

        let start_seek_pos = files.img_position + fat_entry.start_address as u64;
        let file_size = fat_entry.end_address - fat_entry.start_address;

        // Get the file
        self.reader
            .seek(SeekFrom::Start(start_seek_pos))
            .map_err(NarcReaderError::FileSeekError)?;

        let mut file = vec![
            0u8;
            file_size
                .try_into()
                .map_err(|_| NarcReaderError::FileTooLarge(index, file_size))?
        ];

        self.reader
            .read_exact(&mut file)
            .map_err(NarcReaderError::FileReadError)?;

        Ok(file)
    }

    /// Returns an iterator over the files in the NARC file.
    ///
    /// This will sequentially load each file from the NARC file as needed.
    pub fn files_iter(&mut self) -> NarcReaderFilesIter {
        NarcReaderFilesIter::new(self)
    }
}
