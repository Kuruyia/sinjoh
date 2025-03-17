//! Data structure and parser for area light files.
//!
//! Those are the files contained in the `arealight.narc` archive.

use std::{
    iter::Enumerate,
    num::ParseIntError,
    str::{Split, Utf8Error},
};

use thiserror::Error;

use sinjoh_nds::{DsFixed16, DsRgb, DsVecFixed16};

/// Represents the different lines that are found in an area light block.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum AreaLightBlockLine {
    /// The end time at which this light is no longer active (in seconds divided by 2, since
    /// midnight).
    EndTime,

    /// Properties for the Nintendo DS first light.
    Light0,

    /// Properties for the Nintendo DS second light.
    Light1,

    /// Properties for the Nintendo DS third light.
    Light2,

    /// Properties for the Nintendo DS fourth light.
    Light3,

    /// The color of the diffuse reflection.
    DiffuseReflectColor,

    /// The color of the ambient reflection.
    AmbientReflectColor,

    /// The color of the specular reflection.
    SpecularReflectColor,

    /// The color of the emitted light.
    EmissionColor,

    /// The end of the block.
    End,
}

impl AreaLightBlockLine {
    /// Moves the line to the next one.
    pub fn next(&mut self) {
        match self {
            Self::EndTime => *self = Self::Light0,
            Self::Light0 => *self = Self::Light1,
            Self::Light1 => *self = Self::Light2,
            Self::Light2 => *self = Self::Light3,
            Self::Light3 => *self = Self::DiffuseReflectColor,
            Self::DiffuseReflectColor => *self = Self::AmbientReflectColor,
            Self::AmbientReflectColor => *self = Self::SpecularReflectColor,
            Self::SpecularReflectColor => *self = Self::EmissionColor,
            Self::EmissionColor => *self = Self::End,
            Self::End => *self = Self::End,
        }
    }
}

/// Error type for area light parsing.
#[derive(Error, Debug)]
pub enum AreaLightError {
    /// Error while converting a byte array to a UTF-8 string.
    #[error("unable to convert the byte array to a UTF-8 string")]
    ConversionError(#[source] Utf8Error),

    /// An unexpected empty line was encountered while parsing a block.
    #[error("an empty line has been encountered while parsing a block (line number {0})")]
    EarlyEmptyLine(usize),

    /// The area light block parser went past the end of the block.
    #[error("there was an overrun while parsing an area light block - this is a bug in the parser")]
    BlockParseOverrun,

    /// A malformed line was encountered while parsing an area light block.
    #[error(
        "a malformed line was encountered while parsing an area light block (line {0:#?}, line number {1})"
    )]
    MalformedBlockLine(AreaLightBlockLine, usize),

    /// A malformed parameter was encountered while parsing an area light block line.
    #[error(
        "a malformed parameter was encountered while parsing an area light block line (line {0:#?}, line number {1}, parameter {2})"
    )]
    MalformedLineParameter(AreaLightBlockLine, usize, usize, #[source] ParseIntError),

    /// Not enough parameters were specified on an area light block line.
    #[error(
        "not enough parameters were specified on an area light block line (line {0:#?}, line number {1})"
    )]
    NotEnoughParameters(AreaLightBlockLine, usize),
}

/// Represents the properties of a Nintendo DS light.
#[derive(Debug, Clone, Copy)]
pub struct AreaLightProperties {
    /// The color of the light.
    pub color: DsRgb,

    /// The direction vector of the light.
    pub direction: DsVecFixed16,
}

/// Represents an area light block.
#[derive(Debug, Default, Clone, Copy)]
pub struct AreaLightBlock {
    /// The end time at which this light is no longer active (in seconds divided by 2, since
    /// midnight).
    pub end_time: u32,

    /// Properties for the Nintendo DS first light. If this light was specified as invalid,
    /// this field will be `None`.
    ///
    /// This light is used for the 2D sprites, the 3D map model and most of the polygons of map
    /// props.
    pub light_0: Option<AreaLightProperties>,

    /// Properties for the Nintendo DS second light. If this light was specified as invalid,
    /// this field will be `None`.
    ///
    /// This light is seemingly unused in the game.
    pub light_1: Option<AreaLightProperties>,

    /// Properties for the Nintendo DS third light. If this light was specified as invalid,
    /// this field will be `None`.
    ///
    /// This light is used for building windows.
    pub light_2: Option<AreaLightProperties>,

    /// Properties for the Nintendo DS fourth light. If this light was specified as invalid,
    /// this field will be `None`.
    ///
    /// This light is used for lamp post lights, and building lights and doors.
    pub light_3: Option<AreaLightProperties>,

    /// The color of the diffuse reflection.
    pub diffuse_reflect_color: DsRgb,

    /// The color of the ambient reflection.
    pub ambient_reflect_color: DsRgb,

    /// The color of the specular reflection.
    pub specular_reflect_color: DsRgb,

    /// The color of the emitted light.
    pub emission_color: DsRgb,
}

/// Represents an area light file.
#[derive(Debug, Clone)]
pub struct AreaLight {
    /// The blocks of the area light file.
    pub blocks: Vec<AreaLightBlock>,
}

impl AreaLight {
    /// Parses an [`AreaLight`] from a byte slice.
    ///
    /// It is expected that the slice is in the same format as the one found in the `arealight.narc`
    /// archive.
    pub fn parse_bytes(bytes: &[u8]) -> Result<Self, AreaLightError> {
        let area_light =
            Self::parse_string(str::from_utf8(bytes).map_err(AreaLightError::ConversionError)?)?;

        Ok(area_light)
    }

    /// Parses an [`AreaLight`] from a string.
    ///
    /// It is expected that the string is in the same format as the one found in the `arealight.narc`
    /// archive.
    pub fn parse_string(str: &str) -> Result<Self, AreaLightError> {
        let mut blocks = Vec::new();
        let mut current_block = AreaLightBlock::default();
        let mut current_block_line = AreaLightBlockLine::EndTime;

        // Parse the file line-by-line
        for (i, line) in str.lines().enumerate() {
            // Check for special lines
            if line.is_empty() {
                if current_block_line == AreaLightBlockLine::EndTime {
                    // Line is empty and we are not parsing a block, we can skip it
                    continue;
                } else {
                    // Line is empty but we were parsing a block, the file is malformed
                    return Err(AreaLightError::EarlyEmptyLine(i));
                }
            } else if line == "EOF" {
                // End of file, stop here
                break;
            }

            // We are in a block, parse the line according to where we are
            match current_block_line {
                AreaLightBlockLine::EndTime => {
                    let end_time_val = line.split(",").nth(0).ok_or_else(|| {
                        AreaLightError::MalformedBlockLine(current_block_line.clone(), i)
                    })?;

                    current_block.end_time = end_time_val.parse().map_err(|e| {
                        AreaLightError::MalformedLineParameter(current_block_line.clone(), i, 0, e)
                    })?;
                }
                AreaLightBlockLine::Light0 => {
                    current_block.light_0 = Self::parse_light_line(line, &current_block_line, i)?
                }
                AreaLightBlockLine::Light1 => {
                    current_block.light_1 = Self::parse_light_line(line, &current_block_line, i)?
                }
                AreaLightBlockLine::Light2 => {
                    current_block.light_2 = Self::parse_light_line(line, &current_block_line, i)?
                }
                AreaLightBlockLine::Light3 => {
                    current_block.light_3 = Self::parse_light_line(line, &current_block_line, i)?
                }
                AreaLightBlockLine::DiffuseReflectColor => {
                    current_block.diffuse_reflect_color =
                        Self::parse_color_line(line, &current_block_line, i)?
                }
                AreaLightBlockLine::AmbientReflectColor => {
                    current_block.ambient_reflect_color =
                        Self::parse_color_line(line, &current_block_line, i)?
                }
                AreaLightBlockLine::SpecularReflectColor => {
                    current_block.specular_reflect_color =
                        Self::parse_color_line(line, &current_block_line, i)?
                }
                AreaLightBlockLine::EmissionColor => {
                    current_block.emission_color =
                        Self::parse_color_line(line, &current_block_line, i)?
                }
                _ => return Err(AreaLightError::BlockParseOverrun),
            }

            // Prepare parsing the next line
            current_block_line.next();

            if current_block_line == AreaLightBlockLine::End {
                blocks.push(current_block);
                current_block = AreaLightBlock::default();
                current_block_line = AreaLightBlockLine::EndTime;
            }
        }

        Ok(Self { blocks })
    }

    /// Parses a light line from an area light block.
    fn parse_light_line(
        line: &str,
        current_block_line: &AreaLightBlockLine,
        current_line: usize,
    ) -> Result<Option<AreaLightProperties>, AreaLightError> {
        let mut parameters = line.split(",").enumerate();

        // Check whether the light is valid
        let (_, valid_val) = parameters
            .next()
            .ok_or(AreaLightError::NotEnoughParameters(
                current_block_line.clone(),
                current_line,
            ))?;

        if valid_val != "1" {
            return Ok(None);
        }

        // Parse the light color and directional vector
        let color =
            Self::parse_color_parameters(&mut parameters, current_block_line, current_line)?;

        let direction =
            Self::parse_vector_parameters(&mut parameters, current_block_line, current_line)?;

        Ok(Some(AreaLightProperties { color, direction }))
    }

    /// Parses a color line from an area light block.
    fn parse_color_line(
        line: &str,
        current_block_line: &AreaLightBlockLine,
        current_line: usize,
    ) -> Result<DsRgb, AreaLightError> {
        let mut parameters = line.split(",").enumerate();
        Self::parse_color_parameters(&mut parameters, current_block_line, current_line)
    }

    /// Parses the color parameters from a line.
    fn parse_color_parameters(
        parameters: &mut Enumerate<Split<&str>>,
        current_block_line: &AreaLightBlockLine,
        current_line: usize,
    ) -> Result<DsRgb, AreaLightError> {
        // Get the color component values
        let (red_val_idx, red_val) =
            parameters
                .next()
                .ok_or(AreaLightError::NotEnoughParameters(
                    current_block_line.clone(),
                    current_line,
                ))?;

        let (green_val_idx, green_val) =
            parameters
                .next()
                .ok_or(AreaLightError::NotEnoughParameters(
                    current_block_line.clone(),
                    current_line,
                ))?;

        let (blue_val_idx, blue_val) =
            parameters
                .next()
                .ok_or(AreaLightError::NotEnoughParameters(
                    current_block_line.clone(),
                    current_line,
                ))?;

        // Parse each component
        let red = red_val.parse().map_err(|e| {
            AreaLightError::MalformedLineParameter(
                current_block_line.clone(),
                current_line,
                red_val_idx,
                e,
            )
        })?;

        let green = green_val.parse().map_err(|e| {
            AreaLightError::MalformedLineParameter(
                current_block_line.clone(),
                current_line,
                green_val_idx,
                e,
            )
        })?;

        let blue = blue_val.parse().map_err(|e| {
            AreaLightError::MalformedLineParameter(
                current_block_line.clone(),
                current_line,
                blue_val_idx,
                e,
            )
        })?;

        Ok(DsRgb { red, green, blue })
    }

    /// Parses the vector parameters from a line.
    fn parse_vector_parameters(
        parameters: &mut Enumerate<Split<&str>>,
        current_block_line: &AreaLightBlockLine,
        current_line: usize,
    ) -> Result<DsVecFixed16, AreaLightError> {
        // Get the vector component values
        let (x_val_idx, x_val) = parameters
            .next()
            .ok_or(AreaLightError::NotEnoughParameters(
                current_block_line.clone(),
                current_line,
            ))?;

        let (y_val_idx, y_val) = parameters
            .next()
            .ok_or(AreaLightError::NotEnoughParameters(
                current_block_line.clone(),
                current_line,
            ))?;

        let (z_val_idx, z_val) = parameters
            .next()
            .ok_or(AreaLightError::NotEnoughParameters(
                current_block_line.clone(),
                current_line,
            ))?;

        // Parse each component
        let x = x_val.parse::<i16>().map_err(|e| {
            AreaLightError::MalformedLineParameter(
                current_block_line.clone(),
                current_line,
                x_val_idx,
                e,
            )
        })?;

        let y = y_val.parse::<i16>().map_err(|e| {
            AreaLightError::MalformedLineParameter(
                current_block_line.clone(),
                current_line,
                y_val_idx,
                e,
            )
        })?;

        let z = z_val.parse::<i16>().map_err(|e| {
            AreaLightError::MalformedLineParameter(
                current_block_line.clone(),
                current_line,
                z_val_idx,
                e,
            )
        })?;

        Ok(DsVecFixed16 {
            x: DsFixed16::from_bits(x),
            y: DsFixed16::from_bits(y),
            z: DsFixed16::from_bits(z),
        })
    }

    /// Fixes the area light file to align its values with how the game would interpret them.
    pub fn fix(&mut self) {
        for block in self.blocks.iter_mut() {
            // Fix all lights in the block
            if let Some(light_0) = &mut block.light_0 {
                Self::fix_light(light_0);
            }

            if let Some(light_1) = &mut block.light_1 {
                Self::fix_light(light_1);
            }

            if let Some(light_2) = &mut block.light_2 {
                Self::fix_light(light_2);
            }

            if let Some(light_3) = &mut block.light_3 {
                Self::fix_light(light_3);
            }
        }
    }

    /// Fixes a light to align its values with how the game would interpret them.
    fn fix_light(light: &mut AreaLightProperties) {
        // Clamp the direction vector components
        light.direction.x = light.direction.x.clamp(DsFixed16::NEG_ONE, DsFixed16::ONE);
        light.direction.y = light.direction.y.clamp(DsFixed16::NEG_ONE, DsFixed16::ONE);
        light.direction.z = light.direction.z.clamp(DsFixed16::NEG_ONE, DsFixed16::ONE);
    }
}
