//! Data structure and parser for area data files.
//!
//! Those are the files contained in the `area_data.narc` archive.
//!
//! Areas are basically a middle ground between maps and map matrices.
//!
//! They are used to group maps together, and a map matrix can have multiple areas. On the contrary,
//! a map can only belong to one area.

/// Represents an area data file.
#[derive(Debug, Clone, Copy)]
pub struct AreaData {
    /// Index of the associated files in the `area_build.narc` and `areabm_texset.narc` NARCs.
    pub map_prop_archives_id: u16,

    /// Index of the associated file in the `map_tex_set.narc` NARC.
    pub map_texture_archive_id: u16,

    /// Index of the associated file in the `arealight.narc` NARC.
    pub area_light_archive_id: u16,

    /// Unknown: value changes in the NARC, but is unused in the code.
    pub dummy: u16,
}

impl AreaData {
    /// Parses an [`AreaData`] from a byte slice.
    ///
    /// It is expected that the slice is in the same format as the one found in the `area_data.narc`
    /// archive.
    pub fn from_bytes(bytes: [u8; 8]) -> Self {
        // Safety: slice length is explicitly specified, and the length of the `bytes` array is
        // known
        Self {
            map_prop_archives_id: u16::from_le_bytes(bytes[0..=1].try_into().unwrap()),
            map_texture_archive_id: u16::from_le_bytes(bytes[2..=3].try_into().unwrap()),
            area_light_archive_id: u16::from_le_bytes(bytes[6..=7].try_into().unwrap()),
            dummy: u16::from_le_bytes(bytes[4..=5].try_into().unwrap()),
        }
    }
}
