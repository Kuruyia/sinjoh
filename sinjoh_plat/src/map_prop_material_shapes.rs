//! Data structure and parser for map prop material & shapes files.
//!
//! Those are the files contained in the `build_model_matshp.dat` file.

use std::io::{self, Cursor};

use byteorder::{LittleEndian, ReadBytesExt};
use thiserror::Error;

/// Represents IDs locators for finding the material and shapes IDs associated with a map prop.
#[derive(Debug, Clone, Copy)]
pub struct MapPropMaterialShapesLocators {
    /// Number of IDs in the list.
    pub ids_count: u16,

    /// Index of the first ID in the list.
    pub ids_index: u16,
}

/// Represents material and shapes IDs associated with one or more map props.
#[derive(Debug, Clone, Copy)]
pub struct MapPropMaterialShapesIDs {
    /// ID of the material.
    pub material_id: u16,

    /// ID of the shape (mesh).
    pub shape_id: u16,
}

/// Error type for map prop material & shapes parsing.
#[derive(Error, Debug)]
pub enum MapPropMaterialShapesError {
    /// An I/O error has occurred while trying to read from the buffer.
    #[error("an error has occurred while reading the buffer")]
    ReadError(#[source] io::Error),
}

/// Represents the material and shapes associated with a map prop.
#[derive(Debug, Clone)]
pub struct MapPropMaterialShapes {
    /// Index of where the first IDs were originally located in the file IDs list.
    pub ids_index: u16,

    /// IDs of the material and shapes associated with the map prop.
    pub ids: Vec<MapPropMaterialShapesIDs>,
}

impl MapPropMaterialShapes {
    /// Parses a list of map prop material & shapes from a byte slice.
    ///
    /// It is expected that the slice is in the same format as the one found in the
    /// `build_model_matshp.dat` file.
    pub fn parse_bytes(bytes: &[u8]) -> Result<Vec<Option<Self>>, MapPropMaterialShapesError> {
        let mut reader = Cursor::new(bytes);

        // Read the header
        let locators_count = reader
            .read_u16::<LittleEndian>()
            .map_err(MapPropMaterialShapesError::ReadError)?;

        let ids_count = reader
            .read_u16::<LittleEndian>()
            .map_err(MapPropMaterialShapesError::ReadError)?;

        // Read the data
        let locators = Self::parse_locators(&mut reader, locators_count)?;
        let ids = Self::parse_ids(&mut reader, ids_count)?;

        // Transform the data into something nicer
        let mut map_prop_material_shapes = Vec::with_capacity(locators.len());

        for locator in locators.into_iter() {
            if locator.ids_count > 0 {
                let start_index: usize = locator.ids_index.into();
                let end_index: usize = (locator.ids_index + locator.ids_count - 1).into();

                map_prop_material_shapes.push(Some(Self {
                    ids_index: locator.ids_index,
                    ids: ids[start_index..=end_index].to_vec(),
                }));
            } else {
                map_prop_material_shapes.push(None);
            }
        }

        Ok(map_prop_material_shapes)
    }

    /// Parses the IDs locators from the reader.
    fn parse_locators(
        reader: &mut Cursor<&[u8]>,
        locators_count: u16,
    ) -> Result<Vec<MapPropMaterialShapesLocators>, MapPropMaterialShapesError> {
        let mut locators = Vec::with_capacity(locators_count.into());

        for _ in 0..locators_count {
            let ids_count = reader
                .read_u16::<LittleEndian>()
                .map_err(MapPropMaterialShapesError::ReadError)?;

            let ids_index = reader
                .read_u16::<LittleEndian>()
                .map_err(MapPropMaterialShapesError::ReadError)?;

            locators.push(MapPropMaterialShapesLocators {
                ids_count,
                ids_index,
            });
        }

        Ok(locators)
    }

    /// Parses the IDs from the reader.
    fn parse_ids(
        reader: &mut Cursor<&[u8]>,
        ids_count: u16,
    ) -> Result<Vec<MapPropMaterialShapesIDs>, MapPropMaterialShapesError> {
        let mut ids = Vec::with_capacity(ids_count.into());

        for _ in 0..ids_count {
            let material_id = reader
                .read_u16::<LittleEndian>()
                .map_err(MapPropMaterialShapesError::ReadError)?;

            let shape_id = reader
                .read_u16::<LittleEndian>()
                .map_err(MapPropMaterialShapesError::ReadError)?;

            ids.push(MapPropMaterialShapesIDs {
                material_id,
                shape_id,
            });
        }

        Ok(ids)
    }
}
