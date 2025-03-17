//! # `pokeplat_utils`
//!
//! This is a CLI tool that can be used to interactively manipulate Pokémon Platinum data files.
//!
//! Please check the [repository](https://github.com/Kuruyia/sinjoh) for more information.

#![feature(iterator_try_collect)]

use anyhow::{Context, Result};
use build::{COMMIT_DATE_3339, COMMIT_HASH, PKG_VERSION, PROJECT_NAME};
use clap::Parser;
use cli::{Cli, Commands, SqlCommands};
use log::info;
use plat_loader::PlatLoader;
use shadow_rs::shadow;
use sql::repl::SqlRepl;

shadow!(build);

mod cli;
mod plat_loader;
mod sql;

fn main() -> Result<()> {
    // Parse the CLI args and set up logging
    let cli = Cli::parse();

    env_logger::Builder::new()
        .filter_level(cli.verbosity.into())
        .init();

    info!(
        "Starting {} {} (commit {} on {})",
        PROJECT_NAME, PKG_VERSION, COMMIT_HASH, COMMIT_DATE_3339
    );

    // Parse the game resources
    let narc_paths = cli.resources.narc_paths();
    let plat_resources = PlatLoader::load_resources(&narc_paths).with_context(|| {
        if cli.resources.pokeplatinum_repo_path.is_some() {
            "Failed to load the Pokémon Platinum data files. This could be due to multiple reasons:
            - You didn't build the ROM. Make sure that a `build` directory is present in the `pokeplatinum` repo.
            - Path(s) were changed in a newer revision of the `pokeplatinum` repo. Please file an issue in the `sinjoh` project."
        } else {
            "Failed to load the Pokémon Platinum data files."
        }
    })?;

    // Do what the user asked
    match cli.command {
        Commands::Sql { command } => match command {
            SqlCommands::Repl {} => {
                let sql_repl = SqlRepl::from_plat_resources(plat_resources)?;
                sql_repl.repl();
            }
            SqlCommands::Export { export_path } => {
                sql::export::export_plat_resources(plat_resources, &export_path)?
            }
        },
    }

    Ok(())
}
