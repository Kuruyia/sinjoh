# `sinjoh_nds`

The `sinjoh_nds` crate provides Rust data structures to work with formats
encountered when working with the Nintendo DS.

It provides the following:

- Type aliases for manipulating fixed-point numbers commonly found in this
  environment (See [`DsFixed16`], [`DsFixed32`]), and associated 3-dimensional
  vectors (See [`DsVecFixed16`], [`DsVecFixed32`]).
- A utility struct for storing colors (See [`DsRgb`]).
- A reader for reading the files contained in a NARC archive (See
  [`NarcReader`](narc::reader::NarcReader)).
