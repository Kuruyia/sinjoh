use anyhow::{Context, Result};
use rusqlite::{Connection, params};
use sinjoh_plat::map_matrix::MapMatrix;

use super::PopulateSql;

impl PopulateSql for Vec<MapMatrix> {
    fn create_sql_tables(&self, conn: &Connection) -> Result<()> {
        conn.execute(
            "CREATE TABLE map_matrix (
                id                  INTEGER NOT NULL PRIMARY KEY,
                height              INTEGER NOT NULL,
                width               INTEGER NOT NULL,
                model_name_prefix   TEXT NOT NULL
            )",
            (),
        )
        .context("Failed to create the `map_matrix` table")?;

        conn.execute(
            "CREATE TABLE map_matrix_header_id (
                map_matrix_id   INTEGER NOT NULL,
                x               INTEGER NOT NULL,
                y               INTEGER NOT NULL,
                map_header_id   INTEGER NOT NULL,
                PRIMARY KEY (map_matrix_id, x, y),
                FOREIGN KEY (map_matrix_id) REFERENCES map_matrix(id)
            )",
            (),
        )
        .context("Failed to create the `map_matrix_header_id` table")?;

        conn.execute(
            "CREATE TABLE map_matrix_altitude (
                map_matrix_id   INTEGER NOT NULL,
                x               INTEGER NOT NULL,
                y               INTEGER NOT NULL,
                altitude        INTEGER NOT NULL,
                PRIMARY KEY (map_matrix_id, x, y),
                FOREIGN KEY (map_matrix_id) REFERENCES map_matrix(id)
            )",
            (),
        )
        .context("Failed to create the `map_matrix_altitude` table")?;

        conn.execute(
            "CREATE TABLE map_matrix_land_data_id (
                map_matrix_id   INTEGER NOT NULL,
                x               INTEGER NOT NULL,
                y               INTEGER NOT NULL,
                land_data_id    INTEGER NOT NULL,
                PRIMARY KEY (map_matrix_id, x, y),
                FOREIGN KEY (map_matrix_id) REFERENCES map_matrix(id)
            )",
            (),
        )
        .context("Failed to create the `map_matrix_land_data_id` table")?;

        Ok(())
    }

    fn populate_sql_tables(&self, conn: &mut Connection) -> Result<()> {
        for (map_matrix_id, map_matrix) in self.iter().enumerate() {
            conn.execute(
                "INSERT INTO map_matrix (id, height, width, model_name_prefix)
                VALUES (?1, ?2, ?3, ?4)",
                params![
                    map_matrix_id as u64,
                    map_matrix.height,
                    map_matrix.width,
                    map_matrix.model_name_prefix
                ],
            )
            .context("Failed to populate the `map_matrix` table")?;

            if let Some(map_header_ids) = &map_matrix.map_header_ids {
                for (map_index, map_header_id) in map_header_ids.iter().enumerate() {
                    let (x, y) = map_matrix.map_index_to_coords(map_index.try_into()?)?;

                    conn.execute(
                        "INSERT INTO map_matrix_header_id (map_matrix_id, x, y, map_header_id)
                        VALUES (?1, ?2, ?3, ?4)",
                        params![map_matrix_id as u64, x, y, map_header_id],
                    )
                    .context("Failed to populate the `map_matrix_header_id` table")?;
                }
            }

            if let Some(altitudes) = &map_matrix.altitudes {
                for (map_index, altitude) in altitudes.iter().enumerate() {
                    let (x, y) = map_matrix.map_index_to_coords(map_index.try_into()?)?;

                    conn.execute(
                        "INSERT INTO map_matrix_altitude (map_matrix_id, x, y, altitude)
                        VALUES (?1, ?2, ?3, ?4)",
                        params![map_matrix_id as u64, x, y, altitude],
                    )
                    .context("Failed to populate the `map_matrix_altitude` table")?;
                }
            }

            for (map_index, land_data_id) in map_matrix.land_data_ids.iter().enumerate() {
                let (x, y) = map_matrix.map_index_to_coords(map_index.try_into()?)?;

                conn.execute(
                    "INSERT INTO map_matrix_land_data_id (map_matrix_id, x, y, land_data_id)
                    VALUES (?1, ?2, ?3, ?4)",
                    params![map_matrix_id as u64, x, y, land_data_id],
                )
                .context("Failed to populate the `map_matrix_land_data_id` table")?;
            }
        }

        Ok(())
    }
}
