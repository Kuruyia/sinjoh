use std::time::Instant;

use anyhow::Result;
use log::info;
use rusqlite::Connection;
use sinjoh_plat::data::map_headers::PLATINUM_MAP_HEADERS;
use tables::PopulateSql;

use crate::plat_loader::PlatResources;

pub(crate) mod export;
pub(crate) mod repl;
mod tables;

fn prepare_db_from_plat_resources(resources: PlatResources, conn: &mut Connection) -> Result<()> {
    let populate_start = Instant::now();

    resources.area_data.create_and_populate_sql_tables(conn)?;
    resources.area_lights.create_and_populate_sql_tables(conn)?;
    resources
        .area_map_props
        .create_and_populate_sql_tables(conn)?;
    resources.land_data.create_and_populate_sql_tables(conn)?;
    resources
        .map_matrices
        .create_and_populate_sql_tables(conn)?;
    resources
        .map_prop_animation_lists
        .create_and_populate_sql_tables(conn)?;
    resources
        .map_prop_material_shapes
        .create_and_populate_sql_tables(conn)?;

    PLATINUM_MAP_HEADERS.create_and_populate_sql_tables(conn)?;

    let populate_end = Instant::now();
    info!(
        "Populated SQLite database in {} ms",
        (populate_end - populate_start).as_millis()
    );

    Ok(())
}
