//! Data structure and parser for map matrix files.
//!
//! Those are the files contained in the `map_matrix.narc` archive.

use std::{
    io::{self, Cursor, Read},
    string::FromUtf8Error,
};

use byteorder::{LittleEndian, ReadBytesExt};
use thiserror::Error;

/// Error type for map matrix parsing.
#[derive(Error, Debug)]
pub enum MapMatrixError {
    /// An I/O error has occurred while trying to read from the buffer.
    #[error("an error has occurred while reading the buffer")]
    ReadError(#[source] io::Error),

    /// Error while converting the model name prefix to a UTF-8 string.
    #[error("unable to convert the model name prefix into a string")]
    ModelNamePrefixConversion(#[source] FromUtf8Error),

    /// The specified map index is greater or equal than the amount of maps in the matrix.
    #[error("map index is greater or equal than map count (map index is {0}, map count is {1})")]
    MapIndexTooBig(u16, u16),
}

/// Represents a map matrix file.
#[derive(Debug, Clone)]
pub struct MapMatrix {
    /// Height of the map matrix.
    ///
    /// This is the number of maps in the vertical direction.
    pub height: u8,

    /// Width of the map matrix.
    ///
    /// This is the number of maps in the horizontal direction.
    pub width: u8,

    /// Prefix of the model names used in the map matrix.
    ///
    /// This prefix is given to the associated map models NSBMD files.
    pub model_name_prefix: String,

    /// IDs of the map headers contained in the map matrix.
    ///
    /// This is a 2D array of map header IDs. The IDs are stored in row-major order.
    ///
    /// This field is optional, and is only present if the corresponding section is present in the
    /// file.
    pub map_header_ids: Option<Vec<u16>>,

    /// Altitudes of the maps contained in the map matrix.
    ///
    /// This is a 2D array of altitudes, used to calculate the Y-coordinate when rendering a map and
    /// its props. The altitudes are stored in row-major order.
    ///
    /// This field is optional, and is only present if the corresponding section is present in the
    /// file.
    pub altitudes: Option<Vec<u8>>,

    /// IDs of the land data contained in the map matrix.
    ///
    /// This is a 2D array of indexes in the `land_data.narc` NARC. The indexes are stored in
    /// row-major order.
    pub land_data_ids: Vec<u16>,
}

impl MapMatrix {
    /// Parses a [`MapMatrix`] from a byte slice.
    ///
    /// It is expected that the slice is in the same format as the one found in the `map_matrix.narc`
    /// archive.
    pub fn parse_bytes(bytes: &[u8]) -> Result<Self, MapMatrixError> {
        let mut reader = Cursor::new(bytes);

        // Read the map matrix size
        let height = reader.read_u8().map_err(MapMatrixError::ReadError)?;
        let width = reader.read_u8().map_err(MapMatrixError::ReadError)?;
        let matrix_size = height as usize * width as usize;

        // Read whether some sections are present
        let raw_has_map_header_ids_section = reader.read_u8().map_err(MapMatrixError::ReadError)?;
        let raw_has_altitudes_section = reader.read_u8().map_err(MapMatrixError::ReadError)?;

        // Read the model name prefix
        let model_name_prefix_length = reader.read_u8().map_err(MapMatrixError::ReadError)?;

        let mut raw_model_name_prefix = vec![0; model_name_prefix_length.into()];
        reader
            .read_exact(&mut raw_model_name_prefix)
            .map_err(MapMatrixError::ReadError)?;

        let model_name_prefix = String::from_utf8(raw_model_name_prefix)
            .map_err(MapMatrixError::ModelNamePrefixConversion)?;

        // Read the map header IDs
        let map_header_ids = if raw_has_map_header_ids_section != 0 {
            let mut map_header_ids = Vec::with_capacity(matrix_size);

            for _ in 0..matrix_size {
                let map_header_id = reader
                    .read_u16::<LittleEndian>()
                    .map_err(MapMatrixError::ReadError)?;

                map_header_ids.push(map_header_id);
            }

            Some(map_header_ids)
        } else {
            None
        };

        // Read the altitudes
        let altitudes = if raw_has_altitudes_section != 0 {
            let mut altitudes = Vec::with_capacity(matrix_size);

            for _ in 0..matrix_size {
                let altitude = reader.read_u8().map_err(MapMatrixError::ReadError)?;

                altitudes.push(altitude);
            }

            Some(altitudes)
        } else {
            None
        };

        // Read the land data IDs
        let mut land_data_ids = Vec::with_capacity(matrix_size);

        for _ in 0..matrix_size {
            let land_data_id = reader
                .read_u16::<LittleEndian>()
                .map_err(MapMatrixError::ReadError)?;

            land_data_ids.push(land_data_id);
        }

        Ok(Self {
            height,
            width,
            model_name_prefix,
            map_header_ids,
            altitudes,
            land_data_ids,
        })
    }

    /// Transforms a map index into 2D coordinates.
    pub fn map_index_to_coords(&self, index: u16) -> Result<(u16, u16), MapMatrixError> {
        let map_width = self.width as u16;
        let map_height = self.height as u16;
        let map_count = map_width * map_height;

        if index < map_count {
            Ok((index % map_width, index / map_width))
        } else {
            Err(MapMatrixError::MapIndexTooBig(index, map_count))
        }
    }
}
