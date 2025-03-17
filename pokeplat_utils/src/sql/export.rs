use std::{
    fs, io,
    path::{self, PathBuf},
};

use anyhow::{Context, Result};
use log::info;
use rusqlite::Connection;

use crate::plat_loader::PlatResources;

pub fn export_plat_resources(resources: PlatResources, path: &PathBuf) -> Result<()> {
    let remove_file_res = fs::remove_file(path);

    if let Err(err) = remove_file_res {
        if err.kind() != io::ErrorKind::NotFound {
            return Err(err).context("Failed to delete the file at the export path");
        }
    }

    let mut conn = Connection::open(path)?;
    super::prepare_db_from_plat_resources(resources, &mut conn)?;

    info!(
        "Finished exporting SQLite database to: {}",
        path::absolute(path)?.display()
    );

    Ok(())
}
