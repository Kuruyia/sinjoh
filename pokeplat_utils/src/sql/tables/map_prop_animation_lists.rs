use anyhow::{Context, Result};
use rusqlite::{Connection, params};
use sinjoh_plat::map_prop_animation_list::MapPropAnimationList;

use super::PopulateSql;

impl PopulateSql for Vec<MapPropAnimationList> {
    fn create_sql_tables(&self, conn: &Connection) -> Result<()> {
        conn.execute(
            "CREATE TABLE map_prop_animation_list (
                id                              INTEGER NOT NULL PRIMARY KEY,
                deferred_loading                INTEGER NOT NULL,
                deferred_add_to_render_object   INTEGER NOT NULL,
                is_bicycle_slope                INTEGER NOT NULL
            )",
            (),
        )
        .context("Failed to create the `map_prop_animation_list` table")?;

        conn.execute(
            "CREATE TABLE map_prop_animation_list_ids (
                animation_id                INTEGER NOT NULL,
                map_prop_animation_list_id  INTEGER NOT NULL,
                PRIMARY KEY (animation_id, map_prop_animation_list_id),
                FOREIGN KEY (map_prop_animation_list_id) REFERENCES map_prop_animation_list(id)
            )",
            (),
        )
        .context("Failed to create the `map_prop_animation_list_ids` table")?;

        Ok(())
    }

    fn populate_sql_tables(&self, conn: &mut Connection) -> Result<()> {
        for (map_prop_animation_list_id, map_prop_animation_list) in self.iter().enumerate() {
            conn.execute(
                "INSERT INTO map_prop_animation_list (id, deferred_loading, deferred_add_to_render_object, is_bicycle_slope)
                VALUES (?1, ?2, ?3, ?4)",
                params![
                    map_prop_animation_list_id as u64,
                    map_prop_animation_list.deferred_loading,
                    map_prop_animation_list.deferred_add_to_render_object,
                    map_prop_animation_list.is_bicycle_slope
                ],
            ).context("Failed to populate the `map_prop_animation_list` table")?;

            for animation_id in map_prop_animation_list.map_prop_animation_ids.iter() {
                conn.execute(
                    "INSERT INTO map_prop_animation_list_ids (animation_id, map_prop_animation_list_id)
                    VALUES (?1, ?2)",
                    params![animation_id, map_prop_animation_list_id as u64],
                ).context("Failed to populate the `map_prop_animation_list_ids` table")?;
            }
        }

        Ok(())
    }
}
