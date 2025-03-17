//! Data structure and parser for land data files.
//!
//! Those are the files contained in the `land_data.narc` archive.

use std::{
    io::{self, Cursor, Read, Seek, SeekFrom},
    num::TryFromIntError,
};

use byteorder::{LittleEndian, ReadBytesExt};
use thiserror::Error;

use sinjoh_nds::{DsFixed32, DsVecFixed32};

use super::bdhc::{Bdhc, BdhcError};

/// The size of a terrain attributes element.
pub const TERRAIN_ATTRIBUTES_ELEM_SIZE: usize = 2;

/// The size of a map props element.
pub const MAP_PROPS_ELEM_SIZE: usize = 48;

/// The mask to get the tile behavior from a terrain attributes element.
pub const TERRAIN_ATTRIBUTES_ELEM_TILE_BEHAVIOR_MASK: u16 = 0x00FF;

/// The mask to get the collision boolean from a terrain attributes element.
pub const TERRAIN_ATTRIBUTES_ELEM_COLLISION_MASK: u16 = 0x8000;

/// The size of the land data header, that contains the section sizes.
pub const LAND_DATA_HEADER_SIZE: usize = 16;

/// The width of a single map, in tiles.
pub const MAP_TILES_COUNT_X: u32 = 32;

/// The height of a single map, in tiles.
pub const MAP_TILES_COUNT_Y: u32 = 32;

/// The amount of tiles in a map.
pub const MAP_TILES_COUNT: u32 = MAP_TILES_COUNT_X * MAP_TILES_COUNT_Y;

/// Represents the attributes of a terrain tile.
#[derive(Debug, Clone, Copy)]
pub struct TerrainAttributes {
    /// The behavior of the tile.
    ///
    /// Dictates how the tile behaves when the player interacts with it.
    ///
    /// For instance: is it tall grass, water, a trash can, etc.
    pub tile_behavior: u16,

    /// Whether the tile can be walked on or not.
    pub has_collision: bool,
}

impl TerrainAttributes {
    /// Parses a [`TerrainAttributes`] from a raw value.
    ///
    /// It is expected that the value is a 16-bit integer, where the lower 8 bits represent the tile
    /// behavior, and the highest bit represents the collision.
    ///
    /// This is the format used in the `land_data.narc` archive.
    pub fn from_raw(raw_value: u16) -> Self {
        Self {
            tile_behavior: raw_value & TERRAIN_ATTRIBUTES_ELEM_TILE_BEHAVIOR_MASK,
            has_collision: (raw_value & TERRAIN_ATTRIBUTES_ELEM_COLLISION_MASK) != 0,
        }
    }
}

/// Represents an instance of a map prop.
#[derive(Debug, Clone, Copy)]
pub struct MapPropInstance {
    /// The ID of the map prop model.
    ///
    /// Index of the associated model in the `build_model.narc` NARC.
    pub map_prop_model_id: u32,

    /// Position of the map prop on the map.
    pub position: DsVecFixed32,

    /// Rotation of the map prop, where each angle is between 0 and 65535.
    pub rotation: DsVecFixed32,

    /// Scale of the map prop, where 1.0 is the original size.
    pub scale: DsVecFixed32,

    /// Unknown: unused in the code, and seems to be always zero.
    pub dummy: [u32; 2],
}

impl MapPropInstance {
    /// Parses a [`MapPropInstance`] from a byte array.
    ///
    /// It is expected that the array is in the same format as the one found in the `land_data.narc`
    /// archive.
    pub fn from_bytes(bytes: [u8; MAP_PROPS_ELEM_SIZE]) -> Self {
        // Safety: slice length is explicitly specified, and the length of the `bytes` array is
        // known
        Self {
            map_prop_model_id: u32::from_le_bytes(bytes[0..=3].try_into().unwrap()),
            position: DsVecFixed32::new(
                DsFixed32::from_le_bytes(bytes[4..=7].try_into().unwrap()),
                DsFixed32::from_le_bytes(bytes[8..=11].try_into().unwrap()),
                DsFixed32::from_le_bytes(bytes[12..=15].try_into().unwrap()),
            ),
            rotation: DsVecFixed32::new(
                DsFixed32::from_le_bytes(bytes[16..=19].try_into().unwrap()),
                DsFixed32::from_le_bytes(bytes[20..=23].try_into().unwrap()),
                DsFixed32::from_le_bytes(bytes[24..=27].try_into().unwrap()),
            ),
            scale: DsVecFixed32::new(
                DsFixed32::from_le_bytes(bytes[28..=31].try_into().unwrap()),
                DsFixed32::from_le_bytes(bytes[32..=35].try_into().unwrap()),
                DsFixed32::from_le_bytes(bytes[36..=39].try_into().unwrap()),
            ),
            dummy: [
                u32::from_le_bytes(bytes[40..=43].try_into().unwrap()),
                u32::from_le_bytes(bytes[44..=47].try_into().unwrap()),
            ],
        }
    }
}

/// Error type for land data parsing.
#[derive(Error, Debug)]
pub enum LandDataError {
    /// An I/O error has occurred while trying to read from the buffer.
    #[error("an error has occurred while reading the buffer")]
    ReadError(#[source] io::Error),

    /// An I/O error has occurred while trying to seek in the NARC file.
    #[error("a seek error has occurred while seeking in the buffer")]
    SeekError(#[source] io::Error),

    /// Terrain attributes are too large to load into memory.
    #[error("terrain attributes are too large to load into memory (size is {0})")]
    TerrainAttributesTooBig(u32, #[source] TryFromIntError),

    /// Map props are too large to load into memory.
    #[error("map props are too large to load into memory (size is {0})")]
    MapPropsTooBig(u32, #[source] TryFromIntError),

    /// Map model is too large to load into memory.
    #[error("map model is too large to load into memory (size is {0})")]
    MapModelTooBig(u32, #[source] TryFromIntError),

    /// BDHC data is too large to load into memory.
    #[error("BDHC data is too large to load into memory (size is {0})")]
    BdhcTooBig(u32, #[source] TryFromIntError),

    /// An error has occurred while parsing the BDHC data.
    #[error("unable to parse BDHC data")]
    BdhcParseError(#[source] BdhcError),

    /// The specified tile index is greater or equal than the tile count in a map.
    #[error(
        "tile index is greater or equal than tile count (tile index is {0}, tile count is {MAP_TILES_COUNT})"
    )]
    TileIndexTooBig(u32),
}

/// Represents a land data file.
#[derive(Debug, Clone)]
pub struct LandData {
    /// The attributes of the terrain tiles.
    ///
    /// This is a 2D array (32x32) of terrain attributes, which contains tile collision and
    /// behavior. The attributes are stored in row-major order.
    pub terrain_attributes: Vec<TerrainAttributes>,

    /// The map props instances.
    ///
    /// This is an array of map props placed on the map. There can be at most 32 map props per map.
    pub map_props: Vec<MapPropInstance>,

    /// The map model.
    ///
    /// This is a raw binary blob that is used to render the map, stored in the NSBMD format.
    pub map_model: Vec<u8>,

    /// The BDHC data.
    pub bdhc: Bdhc,
}

impl LandData {
    /// Parses a [`LandData`] from a byte slice.
    ///
    /// It is expected that the slice is in the same format as the one found in the `land_data.narc`
    /// archive.
    pub fn parse_bytes(bytes: &[u8]) -> Result<Self, LandDataError> {
        let mut reader = Cursor::new(bytes);

        // Read the section sizes
        let terrain_attributes_size = {
            let raw_value = reader
                .read_u32::<LittleEndian>()
                .map_err(LandDataError::ReadError)?;

            usize::try_from(raw_value)
                .map_err(|e| LandDataError::TerrainAttributesTooBig(raw_value, e))?
        };

        let map_props_size = {
            let raw_value = reader
                .read_u32::<LittleEndian>()
                .map_err(LandDataError::ReadError)?;

            usize::try_from(raw_value).map_err(|e| LandDataError::MapPropsTooBig(raw_value, e))?
        };

        let map_model_size = {
            let raw_value = reader
                .read_u32::<LittleEndian>()
                .map_err(LandDataError::ReadError)?;

            usize::try_from(raw_value).map_err(|e| LandDataError::MapModelTooBig(raw_value, e))?
        };

        let bdhc_size = {
            let raw_value = reader
                .read_u32::<LittleEndian>()
                .map_err(LandDataError::ReadError)?;

            usize::try_from(raw_value).map_err(|e| LandDataError::BdhcTooBig(raw_value, e))?
        };

        let terrain_attributes_count = terrain_attributes_size / TERRAIN_ATTRIBUTES_ELEM_SIZE;
        let map_props_count = map_props_size / MAP_PROPS_ELEM_SIZE;

        // Read the terrain attributes
        reader
            .seek(SeekFrom::Start(LAND_DATA_HEADER_SIZE as u64))
            .map_err(LandDataError::SeekError)?;

        let terrain_attributes =
            Self::parse_terrain_attributes(&mut reader, terrain_attributes_count)?;

        // Read the map props
        reader
            .seek(SeekFrom::Start(
                LAND_DATA_HEADER_SIZE as u64 + terrain_attributes_size as u64,
            ))
            .map_err(LandDataError::SeekError)?;

        let map_props = Self::parse_map_props(&mut reader, map_props_count)?;

        // Read the map model
        reader
            .seek(SeekFrom::Start(
                LAND_DATA_HEADER_SIZE as u64
                    + terrain_attributes_size as u64
                    + map_props_size as u64,
            ))
            .map_err(LandDataError::SeekError)?;

        let mut map_model = vec![0; map_model_size];
        reader
            .read_exact(&mut map_model)
            .map_err(LandDataError::ReadError)?;

        // Read BDHC data
        reader
            .seek(SeekFrom::Start(
                LAND_DATA_HEADER_SIZE as u64
                    + terrain_attributes_size as u64
                    + map_props_size as u64
                    + map_model_size as u64,
            ))
            .map_err(LandDataError::SeekError)?;

        let mut raw_bdhc = vec![0; bdhc_size];
        reader
            .read_exact(&mut raw_bdhc)
            .map_err(LandDataError::ReadError)?;

        let bdhc = Bdhc::parse_bytes(&raw_bdhc).map_err(LandDataError::BdhcParseError)?;

        Ok(Self {
            terrain_attributes,
            map_props,
            map_model,
            bdhc,
        })
    }

    /// Parses the terrain attributes from the reader.
    fn parse_terrain_attributes(
        reader: &mut Cursor<&[u8]>,
        terrain_attributes_count: usize,
    ) -> Result<Vec<TerrainAttributes>, LandDataError> {
        let mut terrain_attributes = Vec::with_capacity(terrain_attributes_count);

        for _ in 0..terrain_attributes_count {
            let raw_terrain_attributes_elem = reader
                .read_u16::<LittleEndian>()
                .map_err(LandDataError::ReadError)?;

            terrain_attributes.push(TerrainAttributes::from_raw(raw_terrain_attributes_elem));
        }

        Ok(terrain_attributes)
    }

    /// Parses the map props from the reader.
    fn parse_map_props(
        reader: &mut Cursor<&[u8]>,
        map_props_count: usize,
    ) -> Result<Vec<MapPropInstance>, LandDataError> {
        let mut map_props = Vec::with_capacity(map_props_count);

        for _ in 0..map_props_count {
            let mut raw_map_prop = [0; MAP_PROPS_ELEM_SIZE];
            reader
                .read_exact(&mut raw_map_prop)
                .map_err(LandDataError::ReadError)?;

            map_props.push(MapPropInstance::from_bytes(raw_map_prop));
        }

        Ok(map_props)
    }

    /// Transforms a tile index into 2D coordinates.
    pub fn tile_index_to_coords(index: u32) -> Result<(u32, u32), LandDataError> {
        if index < MAP_TILES_COUNT {
            Ok((index % MAP_TILES_COUNT_X, index / MAP_TILES_COUNT_X))
        } else {
            Err(LandDataError::TileIndexTooBig(index))
        }
    }
}
