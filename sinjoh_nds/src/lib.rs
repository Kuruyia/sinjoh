#![doc = include_str!("../README.md")]

use cgmath::Vector3;
use fixed::types::{I4F12, I20F12};

pub mod narc;

/// The size of a 32-bit fixed-point number.
pub const DS_FIXED_32_SIZE: usize = 4;

/// The size of a 3D vector of 32-bit fixed-point elements.
pub const DS_VEC_FIXED_32_SIZE: usize = DS_FIXED_32_SIZE * 3;

/// Represents an RBG color.
///
/// Each color component should be 5-bit to follow what the Nintendo DS uses.
#[derive(Debug, Default, Clone, Copy)]
pub struct DsRgb {
    /// The red color component.
    pub red: u8,

    /// The green color component.
    pub green: u8,

    /// The blue color component.
    pub blue: u8,
}

/// A 16-bit signed fixed-point number with 1 sign bit, 3 integer bits and 12 fractional bits.
pub type DsFixed16 = I4F12;

/// A 3-dimensional vector of 16-bit signed fixed-point numbers.
///
/// See [`DsFixed16`].
pub type DsVecFixed16 = Vector3<DsFixed16>;

/// A 32-bit signed fixed-point number with 1 sign bit, 19 integer bits and 12 fractional bits.
pub type DsFixed32 = I20F12;

/// A 3-dimensional vector of 32-bit signed fixed-point numbers.
///
/// See [`DsFixed32`].
pub type DsVecFixed32 = Vector3<DsFixed32>;
