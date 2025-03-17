//! Data structure and parser for BDHC data.
//!
//! Those are embedded in the files contained in the `land_data.narc` archive.

use std::io::{self, Cursor, Read};

use byteorder::{LittleEndian, ReadBytesExt};
use thiserror::Error;

use sinjoh_nds::{DS_FIXED_32_SIZE, DS_VEC_FIXED_32_SIZE, DsFixed32, DsVecFixed32};

/// The magic number at the beginning of a BDHC file. Corresponds to "BDHC" in little-endian.
pub const BDHC_MAGIC: u32 = 0x43484442;

/// The size of the BDHC header.
pub const BDHC_HEADER_SIZE: usize = 12;

/// The size of a BDHC point.
pub const BDHC_POINT_SIZE: usize = 8;

/// The size of a BDHC plate.
pub const BDHC_PLATE_SIZE: usize = 8;

/// The size of a BDHC strip.
pub const BDHC_STRIP_SIZE: usize = 8;

/// Represents the header of BDHC data.
///
/// This header contains the counts of the different sections of the BDHC data.
#[derive(Debug, Clone, Copy)]
pub struct BdhcHeader {
    /// The number of points in the BDHC data.
    pub points_count: u16,

    /// The number of normals in the BDHC data.
    pub normals_count: u16,

    /// The number of constants in the BDHC data.
    pub constants_count: u16,

    /// The number of plates in the BDHC data.
    pub plates_count: u16,

    /// The number of strips in the BDHC data.
    pub strips_count: u16,

    /// The number of elements in the access list in the BDHC data.
    pub access_list_count: u16,
}

impl BdhcHeader {
    /// Parses a [`BdhcHeader`] from a byte slice.
    ///
    /// It is expected that the slice is in the same format as the one found in the BDHC data.
    pub fn from_bytes(bytes: &[u8; BDHC_HEADER_SIZE]) -> Self {
        // Safety: slice length is explicitly specified, and the length of the `bytes` array is
        // known
        Self {
            points_count: u16::from_le_bytes(bytes[0..=1].try_into().unwrap()),
            normals_count: u16::from_le_bytes(bytes[2..=3].try_into().unwrap()),
            constants_count: u16::from_le_bytes(bytes[4..=5].try_into().unwrap()),
            plates_count: u16::from_le_bytes(bytes[6..=7].try_into().unwrap()),
            strips_count: u16::from_le_bytes(bytes[8..=9].try_into().unwrap()),
            access_list_count: u16::from_le_bytes(bytes[10..=11].try_into().unwrap()),
        }
    }
}

/// Represents a point in BDHC data.
#[derive(Debug, Clone, Copy)]
pub struct BdhcPoint {
    /// The X coordinate of the point.
    ///
    /// This coordinate is a 32-bit fixed-point number.
    /// See [`DsFixed32`] for more information.
    pub x: DsFixed32,

    /// The Z coordinate of the point.
    ///
    /// This coordinate is a 32-bit fixed-point number.
    /// See [`DsFixed32`] for more information.
    pub z: DsFixed32,
}

impl BdhcPoint {
    /// Parses a [`BdhcPoint`] from a byte slice.
    ///
    /// It is expected that the slice is in the same format as the one found in the BDHC data.
    pub fn from_bytes(bytes: &[u8; BDHC_POINT_SIZE]) -> Self {
        // Safety: slice length is explicitly specified, and the length of the `bytes` array is
        // known
        Self {
            x: DsFixed32::from_le_bytes(bytes[0..=3].try_into().unwrap()),
            z: DsFixed32::from_le_bytes(bytes[4..=7].try_into().unwrap()),
        }
    }
}

/// Represents a plate in BDHC data.
#[derive(Debug, Clone, Copy)]
pub struct BdhcPlate {
    /// The index of the first point in the BDHC point list.
    pub first_point_index: u16,

    /// The index of the second point in the BDHC point list.
    pub second_point_index: u16,

    /// The index of the normal in the BDHC normal list.
    pub normal_index: u16,

    /// The index of the constant in the BDHC constant list.
    pub constant_index: u16,
}

impl BdhcPlate {
    /// Parses a [`BdhcPlate`] from a byte slice.
    ///
    /// It is expected that the slice is in the same format as the one found in the BDHC data.
    pub fn from_bytes(bytes: &[u8; BDHC_PLATE_SIZE]) -> Self {
        // Safety: slice length is explicitly specified, and the length of the `bytes` array is
        // known
        Self {
            first_point_index: u16::from_le_bytes(bytes[0..=1].try_into().unwrap()),
            second_point_index: u16::from_le_bytes(bytes[2..=3].try_into().unwrap()),
            normal_index: u16::from_le_bytes(bytes[4..=5].try_into().unwrap()),
            constant_index: u16::from_le_bytes(bytes[6..=7].try_into().unwrap()),
        }
    }
}

/// Represents a strip in BDHC data.
#[derive(Debug, Clone, Copy)]
pub struct BdhcStrip {
    /// The scanline of the strip.
    ///
    /// All BDHC plates contained in this strip pass through this scanline.
    ///
    /// This coordinate is a 32-bit fixed-point number.
    /// See [`DsFixed32`] for more information.
    pub scanline: DsFixed32,

    /// The number of elements in the access list for this strip.
    pub access_list_element_count: u16,

    /// The index of the first element in the access list for this strip.
    pub access_list_start_index: u16,
}

impl BdhcStrip {
    /// Parses a [`BdhcStrip`] from a byte slice.
    ///
    /// It is expected that the slice is in the same format as the one found in the BDHC data.
    pub fn from_bytes(bytes: &[u8; BDHC_STRIP_SIZE]) -> Self {
        // Safety: slice length is explicitly specified, and the length of the `bytes` array is
        // known
        Self {
            scanline: DsFixed32::from_le_bytes(bytes[0..=3].try_into().unwrap()),
            access_list_element_count: u16::from_le_bytes(bytes[4..=5].try_into().unwrap()),
            access_list_start_index: u16::from_le_bytes(bytes[6..=7].try_into().unwrap()),
        }
    }
}

/// Error type for BDHC parsing.
#[derive(Error, Debug)]
pub enum BdhcError {
    /// An I/O error has occurred while trying to read from the buffer.
    #[error("an error has occurred while reading the buffer")]
    ReadError(#[source] io::Error),

    /// The BDHC magic number is wrong.
    #[error("wrong BDHC magic number (expected 0x{BDHC_MAGIC:X}, found 0x{0:X})")]
    WrongBdhcMagic(u32),
}

/// Represents a BDHC file.
#[derive(Debug, Clone)]
pub struct Bdhc {
    /// The points in the BDHC data.
    ///
    /// These points are used to define the plate boundaries.
    pub points: Vec<BdhcPoint>,

    /// The normals in the BDHC data.
    ///
    /// These normals are referenced by the plates. They represent the normal vector of the equation
    /// of planes that the plates define.
    ///
    /// The normal vector is a 3D vector with fixed-point coordinates.
    /// See [`DsVecFixed32`] for more information.
    pub normals: Vec<DsVecFixed32>,

    /// The constants in the BDHC data.
    ///
    /// These constants are referenced by the plates. They represent the constant term of the equation
    /// of planes that the plates define.
    ///
    /// The constant term is a 32-bit fixed-point number.
    /// See [`DsFixed32`] for more information.
    pub constants: Vec<DsFixed32>,

    /// The plates in the BDHC data.
    pub plates: Vec<BdhcPlate>,

    /// The strips in the BDHC data.
    pub strips: Vec<BdhcStrip>,

    /// The access list in the BDHC data.
    ///
    /// This list contains indices of the plates that are referenced by the strips.
    pub access_list: Vec<u16>,
}

impl Bdhc {
    /// Parses a [`Bdhc`] from a byte slice.
    ///
    /// It is expected that the slice is in the same format as the one found in the BDHC data.
    pub fn parse_bytes(bytes: &[u8]) -> Result<Self, BdhcError> {
        let mut reader = Cursor::new(bytes);

        // Read the magic
        let magic = reader
            .read_u32::<LittleEndian>()
            .map_err(BdhcError::ReadError)?;

        if magic != BDHC_MAGIC {
            return Err(BdhcError::WrongBdhcMagic(magic));
        }

        // Read the section counts
        let mut raw_header = [0; BDHC_HEADER_SIZE];
        reader
            .read_exact(&mut raw_header)
            .map_err(BdhcError::ReadError)?;

        let header = BdhcHeader::from_bytes(&raw_header);

        // Read the points
        let mut points = Vec::with_capacity(header.points_count.into());

        for _ in 0..header.points_count {
            let mut raw_point = [0; BDHC_POINT_SIZE];
            reader
                .read_exact(&mut raw_point)
                .map_err(BdhcError::ReadError)?;

            points.push(BdhcPoint::from_bytes(&raw_point));
        }

        // Read the normals
        let mut normals = Vec::with_capacity(header.normals_count.into());

        for _ in 0..header.normals_count {
            let mut raw_normal = [0; DS_VEC_FIXED_32_SIZE];
            reader
                .read_exact(&mut raw_normal)
                .map_err(BdhcError::ReadError)?;

            normals.push(DsVecFixed32::new(
                DsFixed32::from_le_bytes(raw_normal[0..=3].try_into().unwrap()),
                DsFixed32::from_le_bytes(raw_normal[4..=7].try_into().unwrap()),
                DsFixed32::from_le_bytes(raw_normal[8..=11].try_into().unwrap()),
            ));
        }

        // Read the constants
        let mut constants = Vec::with_capacity(header.constants_count.into());

        for _ in 0..header.constants_count {
            let mut raw_constant = [0; DS_FIXED_32_SIZE];
            reader
                .read_exact(&mut raw_constant)
                .map_err(BdhcError::ReadError)?;

            constants.push(DsFixed32::from_le_bytes(raw_constant));
        }

        // Read the plates
        let mut plates = Vec::with_capacity(header.plates_count.into());

        for _ in 0..header.plates_count {
            let mut raw_plate = [0; BDHC_PLATE_SIZE];
            reader
                .read_exact(&mut raw_plate)
                .map_err(BdhcError::ReadError)?;

            plates.push(BdhcPlate::from_bytes(&raw_plate));
        }

        // Read the strips
        let mut strips = Vec::with_capacity(header.strips_count.into());

        for _ in 0..header.strips_count {
            let mut raw_strip = [0; BDHC_STRIP_SIZE];
            reader
                .read_exact(&mut raw_strip)
                .map_err(BdhcError::ReadError)?;

            strips.push(BdhcStrip::from_bytes(&raw_strip));
        }

        // Read the access list
        let mut access_list = Vec::with_capacity(header.access_list_count.into());

        for _ in 0..header.access_list_count {
            let access_list_element = reader
                .read_u16::<LittleEndian>()
                .map_err(BdhcError::ReadError)?;

            access_list.push(access_list_element);
        }

        Ok(Self {
            points,
            normals,
            constants,
            plates,
            strips,
            access_list,
        })
    }
}
