use anyhow::{Context, Result};
use rusqlite::{Connection, params};
use sinjoh_plat::area_map_props::AreaMapProps;

use super::PopulateSql;

impl PopulateSql for Vec<AreaMapProps> {
    fn create_sql_tables(&self, conn: &Connection) -> Result<()> {
        conn.execute(
            "CREATE TABLE area_map_prop (
                id          INTEGER NOT NULL,
                map_prop_id INTEGER NOT NULL,
                PRIMARY KEY (id, map_prop_id)
            )",
            (),
        )
        .context("Failed to create the `area_map_prop` table")?;

        Ok(())
    }

    fn populate_sql_tables(&self, conn: &mut Connection) -> Result<()> {
        for (area_map_props_id, area_map_props) in self.iter().enumerate() {
            for map_prop_id in area_map_props.map_props_ids.iter() {
                conn.execute(
                    "INSERT INTO area_map_prop (id, map_prop_id)
                    VALUES (?1, ?2)",
                    params![area_map_props_id as u64, map_prop_id],
                )
                .context("Failed to populate the `area_map_prop` table")?;
            }
        }

        Ok(())
    }
}
