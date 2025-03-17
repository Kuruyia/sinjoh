//! Data structure and parser for map prop animation list files.
//!
//! Those are the files contained in the `bm_anime_list.narc` archive.

use std::io::{self, Cursor, Seek};

use byteorder::{LittleEndian, ReadBytesExt};
use thiserror::Error;

/// The mask to get the deferred loading flag from the raw map prop animation list flags.
pub const FLAG_DEFERRED_LOADING_MASK: u8 = 0x01;

/// The mask to get the deferred add to render object flag from the raw map prop animation list
/// flags.
pub const FLAG_DEFERRED_ADD_TO_RENDER_OBJECT_MASK: u8 = 0x02;

/// The maximum number of animations that can be loaded for a map prop.
pub const MAX_MAP_PROP_ANIMATIONS: u8 = 4;

/// The ID for an invalid map prop animation.
pub const INVALID_MAP_PROP_ANIMATION_ID: u32 = 0xFFFFFFFF;

/// Error type for map prop animation list parsing.
#[derive(Error, Debug)]
pub enum MapPropAnimationListError {
    /// An I/O error has occurred while trying to read from the buffer.
    #[error("an error has occurred while reading the buffer")]
    ReadError(#[source] io::Error),

    /// An I/O error has occurred while trying to seek in the buffer.
    #[error("a seek error has occurred while seeking in the buffer")]
    SeekError(#[source] io::Error),
}

/// Represents a map prop animation list file.
#[derive(Debug, Clone)]
pub struct MapPropAnimationList {
    /// IDs of the animations that can be loaded for a map prop.
    ///
    /// Array of indexes in the `bm_anime.narc` NARC. Each map prop model supports up to 4
    /// animations.
    ///
    /// This array can be empty if the map prop does not have any animations.
    pub map_prop_animation_ids: Vec<u32>,

    /// Whether loading the animations is deferred.
    ///
    /// When loading the map prop models for an area, the game tries to load their animations at the
    /// same time. This flag tells the game that the animations for this map prop model will be
    /// loaded later (usually, when needed, such as the animations for opening/closing building
    /// doors).
    pub deferred_loading: bool,

    /// Whether adding the animations to the render object is deferred.
    ///
    /// When loading the map props for a map, the game tries to add all animations to their render
    /// objects. This flag tells the game that the animations for this map prop will be added later
    /// (usually, when the animation should only start under certain conditions, such as honey trees
    /// shaking).
    pub deferred_add_to_render_object: bool,

    /// Whether the map prop model is a slope for the bicycle.
    pub is_bicycle_slope: bool,
}

impl MapPropAnimationList {
    /// Parses a [`MapPropAnimationList`] from a byte slice.
    ///
    /// It is expected that the slice is in the same format as the one found in the
    /// `bm_anime_list.narc` archive.
    pub fn parse_bytes(bytes: &[u8]) -> Result<Self, MapPropAnimationListError> {
        let mut reader = Cursor::new(bytes);

        // Skip the boolean field that informs if there are animations
        reader
            .seek_relative(1)
            .map_err(MapPropAnimationListError::SeekError)?;

        // Read the flags
        let (deferred_loading, deferred_add_to_render_object) = Self::parse_flags(&mut reader)?;
        let raw_is_bicycle_slope = reader
            .read_u8()
            .map_err(MapPropAnimationListError::ReadError)?;

        // Skip the dummy field
        reader
            .seek_relative(1)
            .map_err(MapPropAnimationListError::SeekError)?;

        // Read the animation IDs
        let mut map_prop_animation_ids = Vec::with_capacity(MAX_MAP_PROP_ANIMATIONS.into());

        for _ in 0..MAX_MAP_PROP_ANIMATIONS {
            let map_prop_animation_id = reader
                .read_u32::<LittleEndian>()
                .map_err(MapPropAnimationListError::ReadError)?;

            if map_prop_animation_id == INVALID_MAP_PROP_ANIMATION_ID {
                break;
            }

            map_prop_animation_ids.push(map_prop_animation_id);
        }

        Ok(Self {
            map_prop_animation_ids,
            deferred_loading,
            deferred_add_to_render_object,
            is_bicycle_slope: raw_is_bicycle_slope != 0,
        })
    }

    /// Parses the animations flags from the reader.
    fn parse_flags(reader: &mut Cursor<&[u8]>) -> Result<(bool, bool), MapPropAnimationListError> {
        // Read the raw flags
        let raw_flags = reader
            .read_u8()
            .map_err(MapPropAnimationListError::ReadError)?;

        // Parse the flags
        let deferred_loading_flag = raw_flags & FLAG_DEFERRED_LOADING_MASK;
        let deferred_add_to_render_object_flag =
            raw_flags & FLAG_DEFERRED_ADD_TO_RENDER_OBJECT_MASK;

        Ok((
            deferred_loading_flag != 0,
            deferred_add_to_render_object_flag != 0,
        ))
    }
}
