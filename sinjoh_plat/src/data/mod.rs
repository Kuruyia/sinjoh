//! Data structures contained in the game binary.
//!
//! This module contains data structures that are used to represent the data structures found in the
//! game binary. These data structures cannot easily be read from a file, so they are embedded in
//! the library instead.

#![allow(dead_code)]

pub mod events;
pub mod map_header_ids;
pub mod map_headers;
pub mod scripts;
pub mod text_banks;

/// Contains various metadata about a map.
#[derive(Debug)]
pub struct MapHeader {
    /// Index of the associated file in the `area_data.narc` NARC.
    pub area_data_archive_id: u8,

    /// For now, unknown value.
    pub unk: u8,

    /// Index of the associated file in the `map_matrix.narc` NARC.
    pub map_matrix_id: u16,

    /// Index of the associated file in the `scr_seq.narc` NARC.
    pub scripts_archive_id: u16,

    /// Index of the associated file in the `scr_seq.narc` NARC, for map initialization.
    pub init_scripts_archive_id: u16,

    /// Index of the associated file in the `pl_msg.narc` NARC.
    pub msg_archive_id: u16,

    /// ID of the music to play during daytime.
    pub day_music_id: u16,

    /// ID of the music to play during nighttime.
    pub night_music_id: u16,

    /// Index of the associated file in the `pl_enc_data.narc` NARC.
    pub wild_encounters_archive_id: u16,

    /// Index of the associated file in the `zone_event.narc` NARC.
    pub events_archive_id: u16,

    /// ID of the text to use as the location name for this map.
    ///
    /// It is present in the location names text bank in the `pl_msg.narc` NARC.
    pub map_label_text_id: u16,

    /// ID of the graphics to display for the map name popup.
    ///
    /// Multiply this by 2 to get the associated file in the `area_win_gra.narc` NARC.
    pub map_label_window_id: u16,

    /// ID of the weather conditions on this map.
    ///
    /// This affects whether the weather on the map is clear, raining, snowing, foggy...
    pub weather: u8,

    /// Type of camera to use when on the map.
    ///
    /// This affects camera angle, FOV, projection type...
    pub camera_type: u8,

    /// Type of map.
    ///
    /// This tells you whether:
    /// - Teleport on the map is allowed.
    /// - The map is a Pokémon Center.
    /// - The map is a cave.
    /// - The map is a building.
    /// - The map is outdoors.
    pub map_type: u16,

    /// The default background graphics to use when a battle is initiated on the map.
    ///
    /// This can be overriden based on multiple factors, such as the tile behavior of where the
    /// battle started, or whether the player is surfing.
    pub battle_bg: u16,

    /// Whether using the bicycle is allowed.
    pub is_bike_allowed: bool,

    /// Whether using the running shoes is allowed.
    pub is_running_allowed: bool,

    /// Whether using the escape rope is allowed.
    pub is_escape_rope_allowed: bool,

    /// Whether using the Fly HM is allowed.
    pub is_fly_allowed: bool,
}
