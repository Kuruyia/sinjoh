use anyhow::{Context, Result};
use rusqlite::{Connection, params};
use sinjoh_plat::area_data::AreaData;

use super::PopulateSql;

impl PopulateSql for Vec<AreaData> {
    fn create_sql_tables(&self, conn: &Connection) -> Result<()> {
        conn.execute(
            "CREATE TABLE area_data (
                id                  INTEGER NOT NULL PRIMARY KEY,
                area_map_prop_id    INTEGER NOT NULL,
                map_texture_id      INTEGER NOT NULL,
                area_light_id       INTEGER NOT NULL,
                dummy               INTEGER NOT NULL
            )",
            (),
        )
        .context("Failed to create the `area_data` table")?;

        Ok(())
    }

    fn populate_sql_tables(&self, conn: &mut Connection) -> Result<()> {
        for (area_data_id, area_data) in self.iter().enumerate() {
            conn.execute(
                "INSERT INTO area_data (id, area_map_prop_id, map_texture_id, area_light_id, dummy)
                VALUES (?1, ?2, ?3, ?4, ?5)",
                params![
                    area_data_id as u64,
                    area_data.map_prop_archives_id,
                    area_data.map_texture_archive_id,
                    area_data.area_light_archive_id,
                    area_data.dummy
                ],
            )
            .context("Failed to populate the `area_data` table")?;
        }

        Ok(())
    }
}
