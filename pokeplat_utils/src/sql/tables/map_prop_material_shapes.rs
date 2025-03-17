use anyhow::{Context, Result};
use rusqlite::{Connection, params};
use sinjoh_plat::map_prop_material_shapes::MapPropMaterialShapes;

use super::PopulateSql;

impl PopulateSql for Vec<Option<MapPropMaterialShapes>> {
    fn create_sql_tables(&self, conn: &Connection) -> Result<()> {
        conn.execute(
            "CREATE TABLE map_prop_material_shape (
                id                          INTEGER NOT NULL PRIMARY KEY,
                material_shape_ids_index    INTEGER NOT NULL
            )",
            (),
        )
        .context("Failed to create the `map_prop_material_shape` table")?;

        conn.execute(
            "CREATE TABLE map_prop_material_shape_ids (
                map_prop_material_shape_id  INTEGER NOT NULL,
                material_id                 INTEGER NOT NULL,
                shape_id                    INTEGER NOT NULL,
                PRIMARY KEY (map_prop_material_shape_id, material_id, shape_id),
                FOREIGN KEY (map_prop_material_shape_id) REFERENCES map_prop_material_shape(id)
            )",
            (),
        )
        .context("Failed to create the `map_prop_material_shape_ids` table")?;

        Ok(())
    }

    fn populate_sql_tables(&self, conn: &mut Connection) -> Result<()> {
        for (map_prop_matshp_id, map_prop_matshp) in self
            .iter()
            .enumerate()
            .filter_map(|elem| elem.1.as_ref().map(|mat_shape| (elem.0, mat_shape)))
        {
            conn.execute(
                "INSERT INTO map_prop_material_shape (id, material_shape_ids_index)
                VALUES (?1, ?2)",
                params![map_prop_matshp_id as u64, map_prop_matshp.ids_index],
            )
            .context("Failed to populate the `map_prop_material_shape` table")?;

            for ids in map_prop_matshp.ids.iter() {
                conn.execute(
                    "INSERT INTO map_prop_material_shape_ids (map_prop_material_shape_id, material_id, shape_id)
                    VALUES (?1, ?2, ?3)",
                    params![map_prop_matshp_id as u64, ids.material_id, ids.shape_id],
                ).context("Failed to populate the `map_prop_material_shape_ids` table")?;
            }
        }

        Ok(())
    }
}
