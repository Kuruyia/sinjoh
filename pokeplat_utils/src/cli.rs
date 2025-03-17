use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};
use clap_verbosity_flag::{InfoLevel, Verbosity};

use crate::build::{CLAP_LONG_VERSION, PROJECT_NAME};

const AREA_DATA_NARC_REPO_BUILD_PATH: &str = "build/res/prebuilt/fielddata/areadata/area_data.narc";
const AREA_LIGHT_NARC_REPO_BUILD_PATH: &str = "build/res/prebuilt/data/arealight.narc";
const AREA_BUILD_NARC_REPO_BUILD_PATH: &str =
    "build/res/prebuilt/fielddata/areadata/area_build_model/area_build.narc";
const BM_ANIME_LIST_NARC_REPO_BUILD_PATH: &str = "build/res/prebuilt/arc/bm_anime_list.narc";
const BUILD_MODEL_MATSHP_DAT_REPO_BUILD_PATH: &str =
    "build/res/prebuilt/fielddata/build_model/build_model_matshp.dat";
const MAP_MATRIX_NARC_REPO_BUILD_PATH: &str = "build/res/field/maps/matrices/map_matrix.narc";
const LAND_DATA_NARC_REPO_BUILD_PATH: &str = "build/res/field/maps/data/land_data.narc";

#[derive(Debug, Parser)]
#[command(about, author, version, long_about = format!("{} {}", PROJECT_NAME, CLAP_LONG_VERSION))]
pub(crate) struct Cli {
    #[command(flatten)]
    pub verbosity: Verbosity<InfoLevel>,

    #[command(flatten)]
    pub resources: ResourcesArgs,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Args)]
pub(crate) struct ResourcesArgs {
    /// Manual paths to the NARC files.
    #[command(flatten)]
    pub narc_paths: Option<NarcPaths>,

    /// Path to the checkout of the `pret/pokeplatinum` Git repository.
    #[arg(long)]
    pub pokeplatinum_repo_path: Option<PathBuf>,
}

impl ResourcesArgs {
    pub fn narc_paths(&self) -> NarcPaths {
        if let Some(narc_paths) = &self.narc_paths {
            return narc_paths.clone();
        } else if let Some(pokeplatinum_repo_path) = &self.pokeplatinum_repo_path {
            return NarcPaths {
                area_data_narc_path: pokeplatinum_repo_path.join(AREA_DATA_NARC_REPO_BUILD_PATH),
                area_light_narc_path: pokeplatinum_repo_path.join(AREA_LIGHT_NARC_REPO_BUILD_PATH),
                area_build_narc_path: pokeplatinum_repo_path.join(AREA_BUILD_NARC_REPO_BUILD_PATH),
                bm_anime_list_narc_path: pokeplatinum_repo_path
                    .join(BM_ANIME_LIST_NARC_REPO_BUILD_PATH),
                build_model_matshp_dat_path: pokeplatinum_repo_path
                    .join(BUILD_MODEL_MATSHP_DAT_REPO_BUILD_PATH),
                map_matrix_narc_path: pokeplatinum_repo_path.join(MAP_MATRIX_NARC_REPO_BUILD_PATH),
                land_data_narc_path: pokeplatinum_repo_path.join(LAND_DATA_NARC_REPO_BUILD_PATH),
            };
        }

        // Clap should have required exactly one of the arguments to be present in the CLI
        // arguments
        unreachable!();
    }
}

// We, unfortunately, need to do a workaround here because `#[command(flatten)]`
// in clap makes an optional field required.
//
// See [`clap-rs/clap#5092`](https://github.com/clap-rs/clap/issues/5092)
#[derive(Debug, Args, Clone)]
#[group(conflicts_with = "pokeplatinum_repo_path")]
#[group(requires_all = ["area_data_narc_path", "area_light_narc_path", "area_build_narc_path", "bm_anime_list_narc_path", "build_model_matshp_dat_path", "map_matrix_narc_path", "land_data_narc_path"])]
pub(crate) struct NarcPaths {
    /// Path to the `area_data.narc` file.
    #[arg(long, required = false)]
    pub area_data_narc_path: PathBuf,

    /// Path to the `arealight.narc` file.
    #[arg(long, required = false)]
    pub area_light_narc_path: PathBuf,

    /// Path to the `area_build.narc` file.
    #[arg(long, required = false)]
    pub area_build_narc_path: PathBuf,

    /// Path to the `bm_anime_list.narc` file.
    #[arg(long, required = false)]
    pub bm_anime_list_narc_path: PathBuf,

    /// Path to the `build_model_matshp.dat` file.
    #[arg(long, required = false)]
    pub build_model_matshp_dat_path: PathBuf,

    /// Path to the `map_matrix.narc` file.
    #[arg(long, required = false)]
    pub map_matrix_narc_path: PathBuf,

    /// Path to the `land_data.narc` file.
    #[arg(long, required = false)]
    pub land_data_narc_path: PathBuf,
}

#[derive(Debug, Subcommand)]
pub(crate) enum Commands {
    /// Explore game data using SQL queries.
    Sql {
        #[command(subcommand)]
        command: SqlCommands,
    },
}

#[derive(Debug, Subcommand)]
pub(crate) enum SqlCommands {
    /// Start an interactive SQL session for querying game data.
    Repl {},

    /// Export game data to a SQLite database.
    Export {
        /// The file path where the SQLite database will be saved.
        /// If the file does not exist, it will be created.
        /// If it exists, it will be overwritten.
        export_path: PathBuf,
    },
}
