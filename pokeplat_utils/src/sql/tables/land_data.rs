use anyhow::{Context, Result};
use rusqlite::{Connection, Transaction, params};
use sinjoh_nds::{DsFixed32, DsVecFixed32};
use sinjoh_plat::{
    bdhc::{BdhcPlate, BdhcPoint, BdhcStrip},
    land_data::{LandData, MapPropInstance, TerrainAttributes},
};

use super::PopulateSql;

fn populate_land_data_terrain_attributes(
    tx: &Transaction,
    land_data_id: usize,
    terrain_attributes: &[TerrainAttributes],
) -> Result<()> {
    let mut stmt = tx.prepare_cached(
        "INSERT INTO land_data_terrain_attributes (land_data_id, x, y, tile_behavior, has_collision)
        VALUES (?1, ?2, ?3, ?4, ?5)",
    ).context("Failed to prepare populating the `land_data_terrain_attributes` table")?;

    for (tile_index, attrs) in terrain_attributes.iter().enumerate() {
        let (x, y) = LandData::tile_index_to_coords(tile_index.try_into()?)?;

        stmt.execute(params![
            land_data_id as u64,
            x,
            y,
            attrs.tile_behavior,
            attrs.has_collision
        ])
        .context("Failed to populate the `land_data_terrain_attributes` table")?;
    }

    Ok(())
}

fn populate_land_data_map_prop_instances(
    tx: &Transaction,
    land_data_id: usize,
    map_props: &[MapPropInstance],
) -> Result<()> {
    let mut stmt = tx.prepare_cached(
        "INSERT INTO land_data_map_prop (idx, land_data_id, map_prop_id, pos_x, pos_y, pos_z, rotation_x, rotation_y, rotation_z, scale_x, scale_y, scale_z, dummy_1, dummy_2)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14)",
    ).context("Failed to prepare populating the `land_data_map_prop` table")?;

    for (index, map_prop_instance) in map_props.iter().enumerate() {
        stmt.execute(params![
            index as u64,
            land_data_id as u64,
            map_prop_instance.map_prop_model_id,
            map_prop_instance.position.x.to_num::<f32>(),
            map_prop_instance.position.y.to_num::<f32>(),
            map_prop_instance.position.z.to_num::<f32>(),
            map_prop_instance.rotation.x.to_num::<f32>(),
            map_prop_instance.rotation.y.to_num::<f32>(),
            map_prop_instance.rotation.z.to_num::<f32>(),
            map_prop_instance.scale.x.to_num::<f32>(),
            map_prop_instance.scale.y.to_num::<f32>(),
            map_prop_instance.scale.z.to_num::<f32>(),
            map_prop_instance.dummy[0],
            map_prop_instance.dummy[1]
        ])
        .context("Failed to populate the `land_data_map_prop` table")?;
    }

    Ok(())
}

fn populate_bdhc_points(
    tx: &Transaction,
    land_data_id: usize,
    bdhc_points: &[BdhcPoint],
) -> Result<()> {
    let mut stmt = tx
        .prepare_cached(
            "INSERT INTO bdhc_point (idx, land_data_id, pos_x, pos_z)
            VALUES (?1, ?2, ?3, ?4)",
        )
        .context("Failed to prepare populating the `bdhc_point` table")?;

    for (index, point) in bdhc_points.iter().enumerate() {
        stmt.execute(params![
            index as u64,
            land_data_id as u64,
            point.x.to_num::<f32>(),
            point.z.to_num::<f32>()
        ])
        .context("Failed to populate the `bdhc_point` table")?;
    }

    Ok(())
}

fn populate_bdhc_normals(
    tx: &Transaction,
    land_data_id: usize,
    bdhc_normals: &[DsVecFixed32],
) -> Result<()> {
    let mut stmt = tx
        .prepare_cached(
            "INSERT INTO bdhc_normal (idx, land_data_id, pos_x, pos_y, pos_z)
            VALUES (?1, ?2, ?3, ?4, ?5)",
        )
        .context("Failed to prepare populating the `bdhc_normal` table")?;

    for (index, normal) in bdhc_normals.iter().enumerate() {
        stmt.execute(params![
            index as u64,
            land_data_id as u64,
            normal.x.to_num::<f32>(),
            normal.y.to_num::<f32>(),
            normal.z.to_num::<f32>()
        ])
        .context("Failed to populate the `bdhc_normal` table")?;
    }

    Ok(())
}

fn populate_bdhc_constants(
    tx: &Transaction,
    land_data_id: usize,
    bdhc_constants: &[DsFixed32],
) -> Result<()> {
    let mut stmt = tx
        .prepare_cached(
            "INSERT INTO bdhc_constant (idx, land_data_id, constant)
            VALUES (?1, ?2, ?3)",
        )
        .context("Failed to prepare populating the `bdhc_constant` table")?;

    for (index, constant) in bdhc_constants.iter().enumerate() {
        stmt.execute(params![
            index as u64,
            land_data_id as u64,
            constant.to_num::<f32>()
        ])
        .context("Failed to populate the `bdhc_constant` table")?;
    }

    Ok(())
}

fn populate_bdhc_plates(
    tx: &Transaction,
    land_data_id: usize,
    bdhc_plates: &[BdhcPlate],
) -> Result<()> {
    let mut stmt = tx.prepare_cached(
        "INSERT INTO bdhc_plate (idx, land_data_id, first_point_idx, second_point_idx, normal_idx, constant_idx)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
    ).context("Failed to prepare populating the `bdhc_plate` table")?;

    for (index, plate) in bdhc_plates.iter().enumerate() {
        stmt.execute(params![
            index as u64,
            land_data_id as u64,
            plate.first_point_index,
            plate.second_point_index,
            plate.normal_index,
            plate.constant_index
        ])
        .context("Failed to populate the `bdhc_plate` table")?;
    }

    Ok(())
}

fn populate_bdhc_access_lists(
    tx: &Transaction,
    land_data_id: usize,
    bdhc_access_lists: &[u16],
) -> Result<()> {
    let mut stmt = tx
        .prepare_cached(
            "INSERT INTO bdhc_access_list (idx, land_data_id, plate_idx)
            VALUES (?1, ?2, ?3)",
        )
        .context("Failed to prepapre populating the `bdhc_access_list` table")?;

    for (index, plate_idx) in bdhc_access_lists.iter().enumerate() {
        stmt.execute(params![index as u64, land_data_id as u64, plate_idx])
            .context("Failed to populate the `bdhc_access_list` table")?;
    }

    Ok(())
}

fn populate_bdhc_strips(
    tx: &Transaction,
    land_data_id: usize,
    bdhc_strips: &[BdhcStrip],
) -> Result<()> {
    let mut stmt = tx.prepare_cached(
        "INSERT INTO bdhc_strip (idx, land_data_id, scanline, access_list_element_count, access_list_start_index)
        VALUES (?1, ?2, ?3, ?4, ?5)",
    ).context("Failed to prepare populating the `bdhc_strip` table")?;

    for (index, strip) in bdhc_strips.iter().enumerate() {
        stmt.execute(params![
            index as u64,
            land_data_id as u64,
            strip.scanline.to_num::<f32>(),
            strip.access_list_element_count,
            strip.access_list_start_index
        ])
        .context("Failed to populate the `bdhc_strip` table")?;
    }

    Ok(())
}

impl PopulateSql for Vec<LandData> {
    fn create_sql_tables(&self, conn: &Connection) -> Result<()> {
        conn.execute(
            "CREATE TABLE land_data_terrain_attributes (
                land_data_id    INTEGER NOT NULL,
                x               INTEGER NOT NULL,
                y               INTEGER NOT NULL,
                tile_behavior   INTEGER NOT NULL,
                has_collision   INTEGER NOT NULL,
                PRIMARY KEY (land_data_id, x, y)
            )",
            (),
        )
        .context("Failed to create the `land_data_terrain_attributes` table")?;

        conn.execute(
            "CREATE TABLE land_data_map_prop (
                idx             INTEGER NOT NULL,
                land_data_id    INTEGER NOT NULL,
                map_prop_id     INTEGER NOT NULL,
                pos_x           INTEGER NOT NULL,
                pos_y           INTEGER NOT NULL,
                pos_z           INTEGER NOT NULL,
                rotation_x      INTEGER NOT NULL,
                rotation_y      INTEGER NOT NULL,
                rotation_z      INTEGER NOT NULL,
                scale_x         INTEGER NOT NULL,
                scale_y         INTEGER NOT NULL,
                scale_z         INTEGER NOT NULL,
                dummy_1         INTEGER NOT NULL,
                dummy_2         INTEGER NOT NULL,
                PRIMARY KEY (idx, land_data_id)
            )",
            (),
        )
        .context("Failed to create the `land_data_map_prop` table")?;

        conn.execute(
            "CREATE TABLE bdhc_point (
                idx             INTEGER NOT NULL,
                land_data_id    INTEGER NOT NULL,
                pos_x           INTEGER NOT NULL,
                pos_z           INTEGER NOT NULL,
                PRIMARY KEY (idx, land_data_id)
            )",
            (),
        )
        .context("Failed to create the `bdhc_point` table")?;

        conn.execute(
            "CREATE TABLE bdhc_normal (
                idx             INTEGER NOT NULL,
                land_data_id    INTEGER NOT NULL,
                pos_x           INTEGER NOT NULL,
                pos_y           INTEGER NOT NULL,
                pos_z           INTEGER NOT NULL,
                PRIMARY KEY (idx, land_data_id)
            )",
            (),
        )
        .context("Failed to create the `bdhc_normal` table")?;

        conn.execute(
            "CREATE TABLE bdhc_constant (
                idx             INTEGER NOT NULL,
                land_data_id    INTEGER NOT NULL,
                constant        INTEGER NOT NULL,
                PRIMARY KEY (idx, land_data_id)
            )",
            (),
        )
        .context("Failed to create the `bdhc_constant` table")?;

        conn.execute(
            "CREATE TABLE bdhc_plate (
                idx                 INTEGER NOT NULL,
                land_data_id        INTEGER NOT NULL,
                first_point_idx     INTEGER NOT NULL,
                second_point_idx    INTEGER NOT NULL,
                normal_idx          INTEGER NOT NULL,
                constant_idx        INTEGER NOT NULL,
                PRIMARY KEY (idx, land_data_id),
                FOREIGN KEY (first_point_idx, land_data_id) REFERENCES bdhc_point(idx, land_data_id),
                FOREIGN KEY (second_point_idx, land_data_id) REFERENCES bdhc_point(idx, land_data_id),
                FOREIGN KEY (normal_idx, land_data_id) REFERENCES bdhc_normal(idx, land_data_id),
                FOREIGN KEY (constant_idx, land_data_id) REFERENCES bdhc_constant(idx, land_data_id)
            )",
            (),
        ).context("Failed to create the `bdhc_plate` table")?;

        conn.execute(
            "CREATE TABLE bdhc_access_list (
                idx                         INTEGER NOT NULL,
                land_data_id                INTEGER NOT NULL,
                plate_idx                   INTEGER NOT NULL,
                PRIMARY KEY (idx, land_data_id),
                FOREIGN KEY (plate_idx, land_data_id) REFERENCES bdhc_plate(idx, land_data_id)
            )",
            (),
        )
        .context("Failed to create the `bdhc_access_list` table")?;

        conn.execute(
            "CREATE TABLE bdhc_strip (
                idx                         INTEGER NOT NULL,
                land_data_id                INTEGER NOT NULL,
                scanline                    INTEGER NOT NULL,
                access_list_element_count   INTEGER NOT NULL,
                access_list_start_index     INTEGER NOT NULL,
                PRIMARY KEY (idx, land_data_id),
                FOREIGN KEY (access_list_start_index, land_data_id) REFERENCES bdhc_access_list(idx, land_data_id)
            )",
            (),
        ).context("Failed to create the `bdhc_strip` table")?;

        Ok(())
    }

    fn populate_sql_tables(&self, conn: &mut Connection) -> Result<()> {
        let tx = conn.transaction()?;

        for (land_data_id, land_data) in self.iter().enumerate() {
            populate_land_data_terrain_attributes(
                &tx,
                land_data_id,
                &land_data.terrain_attributes,
            )?;
            populate_land_data_map_prop_instances(&tx, land_data_id, &land_data.map_props)?;

            populate_bdhc_points(&tx, land_data_id, &land_data.bdhc.points)?;
            populate_bdhc_normals(&tx, land_data_id, &land_data.bdhc.normals)?;
            populate_bdhc_constants(&tx, land_data_id, &land_data.bdhc.constants)?;
            populate_bdhc_plates(&tx, land_data_id, &land_data.bdhc.plates)?;
            populate_bdhc_access_lists(&tx, land_data_id, &land_data.bdhc.access_list)?;
            populate_bdhc_strips(&tx, land_data_id, &land_data.bdhc.strips)?;
        }

        tx.commit()?;
        Ok(())
    }
}
