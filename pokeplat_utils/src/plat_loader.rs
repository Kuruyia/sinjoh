use std::{fs, path::PathBuf};

use anyhow::{Context, Result, anyhow};
use log::{debug, info};
use sinjoh_nds::narc::reader::{NarcReader, NarcReaderFlags};
use sinjoh_plat::{
    area_data::AreaData, area_light::AreaLight, area_map_props::AreaMapProps, land_data::LandData,
    map_matrix::MapMatrix, map_prop_animation_list::MapPropAnimationList,
    map_prop_material_shapes::MapPropMaterialShapes,
};

use crate::cli::NarcPaths;

pub(crate) struct PlatResources {
    pub area_data: Vec<AreaData>,
    pub area_lights: Vec<AreaLight>,
    pub area_map_props: Vec<AreaMapProps>,
    pub map_prop_animation_lists: Vec<MapPropAnimationList>,
    pub map_prop_material_shapes: Vec<Option<MapPropMaterialShapes>>,
    pub map_matrices: Vec<MapMatrix>,
    pub land_data: Vec<LandData>,
}

pub(crate) struct PlatLoader {}

impl PlatLoader {
    pub fn load_resources(narc_paths: &NarcPaths) -> Result<PlatResources> {
        // Read area data
        let area_data = Self::read_area_data(&narc_paths.area_data_narc_path)?;
        info!("Read {} area data files", area_data.len());
        debug!("Read area data:\n{:#?}", area_data);

        // Read area lights
        let area_lights = Self::read_area_lights(&narc_paths.area_light_narc_path)?;
        info!("Read {} area lights", area_lights.len());
        debug!("Read area lights:\n{:#?}", area_lights);

        // Read area map props
        let area_map_props = Self::read_area_map_props(&narc_paths.area_build_narc_path)?;
        info!("Read {} area map props", area_map_props.len());
        debug!("Read area map props:\n{:#?}", area_map_props);

        // Read map prop animation lsit
        let map_prop_animation_lists =
            Self::read_area_map_prop_animation_lists(&narc_paths.bm_anime_list_narc_path)?;
        info!(
            "Read {} map prop animation lists",
            map_prop_animation_lists.len()
        );
        debug!(
            "Read map prop animation lists:\n{:#?}",
            map_prop_animation_lists
        );

        // Read map prop material and shapes
        let map_prop_material_shapes =
            Self::read_map_prop_material_shapes(&narc_paths.build_model_matshp_dat_path)?;
        info!(
            "Read {} map prop material & shapes",
            map_prop_material_shapes.len()
        );
        debug!(
            "Read map prop material & shapes:\n{:#?}",
            map_prop_material_shapes
        );

        // Read map matrices
        let map_matrices = Self::read_map_matrices(&narc_paths.map_matrix_narc_path)?;
        info!("Read {} map matrices", map_matrices.len());
        debug!("Read map matrices:\n{:#?}", map_matrices);

        // Read land data
        let land_data = Self::read_land_data(&narc_paths.land_data_narc_path)?;
        info!("Read {} land data files", land_data.len());
        debug!("Read land data:\n{:#?}", land_data);

        Ok(PlatResources {
            area_data,
            area_lights,
            area_map_props,
            map_prop_animation_lists,
            map_prop_material_shapes,
            map_matrices,
            land_data,
        })
    }

    fn read_area_data(area_data_narc_path: &PathBuf) -> Result<Vec<AreaData>> {
        // Read the area data NARC
        info!(
            "Reading `area_data.narc` at: {}",
            area_data_narc_path.display()
        );

        let mut area_data_narc_reader =
            NarcReader::read_from_file(area_data_narc_path, NarcReaderFlags::default())
                .context("Failed to read the area data NARC file")?;

        debug!("Read area data NARC:\n{:#?}", area_data_narc_reader);

        // Parse each area data
        let area_data = area_data_narc_reader
            .files_iter()
            .map(|file| -> Result<AreaData> {
                Ok(AreaData::from_bytes(
                    file.context("Unable to read an area data file from the NARC")?
                        .try_into()
                        .map_err(|_| anyhow!("Unable to convert the area data to an array"))?,
                ))
            })
            .try_collect::<Vec<_>>()?;

        Ok(area_data)
    }

    fn read_area_lights(area_light_narc_path: &PathBuf) -> Result<Vec<AreaLight>> {
        // Read the area light NARC
        info!(
            "Reading `arealight.narc` at: {}",
            area_light_narc_path.display()
        );

        let mut area_light_narc_reader =
            NarcReader::read_from_file(area_light_narc_path, NarcReaderFlags::default())
                .context("Failed to read the area light NARC file")?;

        debug!("Read area light NARC:\n{:#?}", area_light_narc_reader);

        // Parse each area light
        let mut area_lights = area_light_narc_reader
            .files_iter()
            .map(|file| -> Result<AreaLight> {
                Ok(AreaLight::parse_bytes(
                    file.context("Unable to read an area light file from the NARC")?
                        .as_slice(),
                )?)
            })
            .try_collect::<Vec<_>>()?;

        // Fix each area light
        for area_light in area_lights.iter_mut() {
            area_light.fix();
        }

        Ok(area_lights)
    }

    fn read_area_map_props(area_build_narc_path: &PathBuf) -> Result<Vec<AreaMapProps>> {
        // Read the map props NARC
        info!(
            "Reading `area_build.narc` at: {}",
            area_build_narc_path.display()
        );

        let mut map_props_narc_reader =
            NarcReader::read_from_file(area_build_narc_path, NarcReaderFlags::default())
                .context("Failed to read the map props NARC file")?;

        debug!("Read map props NARC:\n{:#?}", map_props_narc_reader);

        // Parse each area map props
        let area_map_props = map_props_narc_reader
            .files_iter()
            .map(|file| -> Result<AreaMapProps> {
                Ok(AreaMapProps::parse_bytes(
                    file.context("Unable to read an area build file from the NARC")?
                        .as_slice(),
                )?)
            })
            .try_collect::<Vec<_>>()?;

        Ok(area_map_props)
    }

    fn read_area_map_prop_animation_lists(
        bm_anime_list_narc_path: &PathBuf,
    ) -> Result<Vec<MapPropAnimationList>> {
        // Read the map prop animation list NARC
        info!(
            "Reading `bm_anime_list.narc` at: {}",
            bm_anime_list_narc_path.display()
        );

        let mut bm_anime_list_narc_reader =
            NarcReader::read_from_file(bm_anime_list_narc_path, NarcReaderFlags::default())
                .context("Failed to read the map prop animation list NARC file")?;

        debug!(
            "Read map prop anime list NARC:\n{:#?}",
            bm_anime_list_narc_reader
        );

        // Parse each map prop animation list
        let map_prop_animation_lists = bm_anime_list_narc_reader
            .files_iter()
            .map(|file| -> Result<MapPropAnimationList> {
                Ok(MapPropAnimationList::parse_bytes(
                    file.context("Unable to read a map prop animation list file from the NARC")?
                        .as_slice(),
                )?)
            })
            .try_collect::<Vec<_>>()?;

        Ok(map_prop_animation_lists)
    }

    fn read_map_prop_material_shapes(
        build_model_matshp_dat_path: &PathBuf,
    ) -> Result<Vec<Option<MapPropMaterialShapes>>> {
        // Read the map prop material shapes data
        info!(
            "Reading `build_model_matshp.dat` at: {}",
            build_model_matshp_dat_path.display()
        );

        let map_prop_material_shapes_data = fs::read(build_model_matshp_dat_path)
            .context("Failed to read the map prop material shapes data file")?;

        // Parse the data
        let map_prop_material_shapes =
            MapPropMaterialShapes::parse_bytes(&map_prop_material_shapes_data)
                .context("Failed to parse the map prop material shapes data file")?;

        Ok(map_prop_material_shapes)
    }

    fn read_map_matrices(map_matrix_narc_path: &PathBuf) -> Result<Vec<MapMatrix>> {
        // Read the map matrix NARC
        info!(
            "Reading `map_matrix.narc` at: {}",
            map_matrix_narc_path.display()
        );

        let mut map_matrix_narc_reader =
            NarcReader::read_from_file(map_matrix_narc_path, NarcReaderFlags::default())
                .context("Failed to read the map matrix NARC file")?;

        debug!("Read map matrix NARC:\n{:#?}", map_matrix_narc_reader);

        // Parse each map matrix
        let map_matrices = map_matrix_narc_reader
            .files_iter()
            .map(|file| -> Result<MapMatrix> {
                Ok(MapMatrix::parse_bytes(
                    file.context("Unable to read a map matrix file from the NARC")?
                        .as_slice(),
                )?)
            })
            .try_collect::<Vec<_>>()?;

        Ok(map_matrices)
    }

    fn read_land_data(land_data_narc_path: &PathBuf) -> Result<Vec<LandData>> {
        // Read the land data NARC
        info!(
            "Reading `land_data.narc` at: {}",
            land_data_narc_path.display()
        );

        let mut land_data_narc_reader =
            NarcReader::read_from_file(land_data_narc_path, NarcReaderFlags::default())
                .context("Failed to read the land data NARC file")?;

        debug!("Read land data NARC:\n{:#?}", land_data_narc_reader);

        // Parse each land data
        let land_data = land_data_narc_reader
            .files_iter()
            .map(|file| -> Result<LandData> {
                Ok(LandData::parse_bytes(
                    file.context("Unable to read a land data file from the NARC")?
                        .as_slice(),
                )?)
            })
            .try_collect::<Vec<_>>()?;

        Ok(land_data)
    }
}
