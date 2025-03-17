use std::collections::HashMap;

use anyhow::{Context, Result};
use rusqlite::{Connection, params};
use sinjoh_plat::data::MapHeader;

use super::PopulateSql;

impl PopulateSql for HashMap<usize, MapHeader> {
    fn create_sql_tables(&self, conn: &Connection) -> Result<()> {
        conn.execute(
            "CREATE TABLE map_header (
                id                          INTEGER NOT NULL PRIMARY KEY,
                area_data_archive_id        INTEGER NOT NULL,
                unk                         INTEGER NOT NULL,
                map_matrix_id               INTEGER NOT NULL,
                scripts_archive_id          INTEGER NOT NULL,
                init_scripts_archive_id     INTEGER NOT NULL,
                msg_archive_id              INTEGER NOT NULL,
                day_music_id                INTEGER NOT NULL,
                night_music_id              INTEGER NOT NULL,
                wild_encounters_archive_id  INTEGER NOT NULL,
                events_archive_id           INTEGER NOT NULL,
                map_label_text_id           INTEGER NOT NULL,
                map_label_window_id         INTEGER NOT NULL,
                weather                     INTEGER NOT NULL,
                camera_type                 INTEGER NOT NULL,
                map_type                    INTEGER NOT NULL,
                battle_bg                   INTEGER NOT NULL,
                is_bike_allowed             INTEGER NOT NULL,
                is_running_allowed          INTEGER NOT NULL,
                is_escape_rope_allowed      INTEGER NOT NULL,
                is_fly_allowed              INTEGER NOT NULL
            )",
            (),
        )
        .context("Failed to create the `map_header` table")?;

        Ok(())
    }

    fn populate_sql_tables(&self, conn: &mut Connection) -> Result<()> {
        for (&map_header_id, map_header) in self.iter() {
            conn.execute(
                "INSERT INTO map_header (
                    id,
                    area_data_archive_id,
                    unk,
                    map_matrix_id,
                    scripts_archive_id,
                    init_scripts_archive_id,
                    msg_archive_id,
                    day_music_id,
                    night_music_id,
                    wild_encounters_archive_id,
                    events_archive_id,
                    map_label_text_id,
                    map_label_window_id,
                    weather,
                    camera_type,
                    map_type,
                    battle_bg,
                    is_bike_allowed,
                    is_running_allowed,
                    is_escape_rope_allowed,
                    is_fly_allowed
                )
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
                params![
                    map_header_id as u64,
                    map_header.area_data_archive_id,
                    map_header.unk,
                    map_header.map_matrix_id,
                    map_header.scripts_archive_id,
                    map_header.init_scripts_archive_id,
                    map_header.msg_archive_id,
                    map_header.day_music_id,
                    map_header.night_music_id,
                    map_header.wild_encounters_archive_id,
                    map_header.events_archive_id,
                    map_header.map_label_text_id,
                    map_header.map_label_window_id,
                    map_header.weather,
                    map_header.camera_type,
                    map_header.map_type,
                    map_header.battle_bg,
                    map_header.is_bike_allowed,
                    map_header.is_running_allowed,
                    map_header.is_escape_rope_allowed,
                    map_header.is_fly_allowed
                ],
            )
            .context("Failed to populate the `map_header` table")?;
        }

        Ok(())
    }
}
