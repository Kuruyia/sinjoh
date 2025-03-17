//! Data structure and parser for area map props files.
//!
//! Those are the files contained in the `area_build.narc` archive.

use std::io::{self, Cursor};

use byteorder::{LittleEndian, ReadBytesExt};
use thiserror::Error;

/// Error type for area map props parsing.
#[derive(Error, Debug)]
pub enum AreaMapPropsError {
    /// An I/O error has occurred while trying to read from the buffer.
    #[error("an error has occurred while reading the buffer")]
    ReadError(#[source] io::Error),
}

/// Represents an area map props file.
#[derive(Debug, Clone)]
pub struct AreaMapProps {
    /// IDs of the map props contained in the area and that will be loaded when the player is in
    /// a map belonging to this area.
    pub map_props_ids: Vec<u16>,
}

impl AreaMapProps {
    /// Parses an [`AreaMapProps`] from a byte slice.
    ///
    /// It is expected that the slice is in the same format as the one found in the
    /// `area_build.narc` archive.
    pub fn parse_bytes(bytes: &[u8]) -> Result<Self, AreaMapPropsError> {
        let mut reader = Cursor::new(bytes);

        // Read the map prop IDs count
        let map_props_ids_count = reader
            .read_u16::<LittleEndian>()
            .map_err(AreaMapPropsError::ReadError)?;

        // Read the map prop IDs
        let mut map_props_ids = Vec::with_capacity(map_props_ids_count.into());
        for _ in 0..map_props_ids_count {
            map_props_ids.push(
                reader
                    .read_u16::<LittleEndian>()
                    .map_err(AreaMapPropsError::ReadError)?,
            );
        }

        Ok(Self { map_props_ids })
    }
}
