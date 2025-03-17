use anyhow::{Context, Result};
use rusqlite::{Connection, params};
use sinjoh_nds::DsRgb;
use sinjoh_plat::area_light::{AreaLight, AreaLightBlock, AreaLightProperties};

use super::PopulateSql;

enum AreaLightColorKind {
    Diffuse,
    Ambient,
    Specular,
    Emission,
}

impl AreaLightColorKind {
    fn as_str(&self) -> &'static str {
        match self {
            AreaLightColorKind::Diffuse => "diffuse",
            AreaLightColorKind::Ambient => "ambient",
            AreaLightColorKind::Specular => "specular",
            AreaLightColorKind::Emission => "emission",
        }
    }
}

fn populate_area_light_properties(
    conn: &Connection,
    light_id: u32,
    area_light_id: usize,
    block: &AreaLightBlock,
    light: &AreaLightProperties,
) -> Result<()> {
    conn.execute(
        "INSERT INTO area_light_properties (light_id, area_light_id, area_light_end_time, red, green, blue, dir_x, dir_y, dir_z)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
        params![
            light_id, area_light_id as u64, block.end_time,
            light.color.red, light.color.green, light.color.blue,
            light.direction.x.to_num::<f32>(), light.direction.y.to_num::<f32>(), light.direction.z.to_num::<f32>()
        ],
    ).context("Failed to populate the `area_light_properties` table")?;

    Ok(())
}

fn populate_area_light_colors(
    conn: &Connection,
    kind: AreaLightColorKind,
    area_light_id: usize,
    end_time: u32,
    color: &DsRgb,
) -> Result<()> {
    conn.execute(
        "INSERT INTO area_light_color (kind, area_light_id, area_light_end_time, red, green, blue)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![
            kind.as_str(),
            area_light_id as u64,
            end_time,
            color.red,
            color.green,
            color.blue,
        ],
    )
    .context("Failed to populate the `area_light_color` table")?;

    Ok(())
}

impl PopulateSql for Vec<AreaLight> {
    fn create_sql_tables(&self, conn: &Connection) -> Result<()> {
        conn.execute(
            "CREATE TABLE area_light (
                id          INTEGER NOT NULL,
                end_time    INTEGER NOT NULL,
                PRIMARY KEY (id, end_time)
            )",
            (),
        )
        .context("Failed to create the `area_light` table")?;

        conn.execute(
            "CREATE TABLE area_light_properties (
                light_id            INTEGER NOT NULL,
                area_light_id       INTEGER NOT NULL,
                area_light_end_time INTEGER NOT NULL,
                red                 INTEGER NOT NULL,
                green               INTEGER NOT NULL,
                blue                INTEGER NOT NULL,
                dir_x               INTEGER NOT NULL,
                dir_y               INTEGER NOT NULL,
                dir_z               INTEGER NOT NULL,
                PRIMARY KEY (light_id, area_light_id, area_light_end_time),
                FOREIGN KEY (area_light_id, area_light_end_time) REFERENCES area_light(id, end_time)
            )",
            (),
        )
        .context("Failed to create the `area_light_properties` table")?;

        conn.execute(
            "CREATE TABLE area_light_color (
                kind                TEXT CHECK(kind IN ('diffuse', 'ambient', 'specular', 'emission'))  NOT NULL,
                area_light_id       INTEGER                                                             NOT NULL,
                area_light_end_time INTEGER                                                             NOT NULL,
                red                 INTEGER                                                             NOT NULL,
                green               INTEGER                                                             NOT NULL,
                blue                INTEGER                                                             NOT NULL,
                PRIMARY KEY (kind, area_light_id, area_light_end_time),
                FOREIGN KEY (area_light_id, area_light_end_time) REFERENCES area_light(id, end_time)
            )",
            (),
        ).context("Failed to create the `area_light_color` table")?;

        Ok(())
    }

    fn populate_sql_tables(&self, conn: &mut Connection) -> Result<()> {
        for (area_light_id, area_light) in self.iter().enumerate() {
            for block in area_light.blocks.iter() {
                // Insert area light
                conn.execute(
                    "INSERT INTO area_light (id, end_time)
                    VALUES (?1, ?2)",
                    params![area_light_id as u64, block.end_time],
                )
                .context("Failed to populate the `area_light` table")?;

                // Insert area light properties
                if let Some(light) = block.light_0 {
                    populate_area_light_properties(conn, 0, area_light_id, block, &light)?;
                }

                if let Some(light) = block.light_1 {
                    populate_area_light_properties(conn, 1, area_light_id, block, &light)?;
                }

                if let Some(light) = block.light_2 {
                    populate_area_light_properties(conn, 2, area_light_id, block, &light)?;
                }

                if let Some(light) = block.light_3 {
                    populate_area_light_properties(conn, 3, area_light_id, block, &light)?;
                }

                // Insert area light colors
                populate_area_light_colors(
                    conn,
                    AreaLightColorKind::Diffuse,
                    area_light_id,
                    block.end_time,
                    &block.diffuse_reflect_color,
                )?;

                populate_area_light_colors(
                    conn,
                    AreaLightColorKind::Ambient,
                    area_light_id,
                    block.end_time,
                    &block.ambient_reflect_color,
                )?;

                populate_area_light_colors(
                    conn,
                    AreaLightColorKind::Specular,
                    area_light_id,
                    block.end_time,
                    &block.specular_reflect_color,
                )?;

                populate_area_light_colors(
                    conn,
                    AreaLightColorKind::Emission,
                    area_light_id,
                    block.end_time,
                    &block.emission_color,
                )?;
            }
        }

        Ok(())
    }
}
