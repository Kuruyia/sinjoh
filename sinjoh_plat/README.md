# `sinjoh_plat`

The `sinjoh_plat` crate provides Rust data structures to work with formats
encountered when working with Pokémon Platinum.

It provides the following:

- Data structure and parser for area data files (`area_data.narc`). See
  [`AreaData`](area_data::AreaData).
- Data structure and parser for area light files (`arealight.narc`). See
  [`AreaLight`](area_light::AreaLight).
- Data structure and parser for area map props files (`area_build.narc`). See
  [`AreaMapProps`](area_map_props::AreaMapProps).
- Data structure and parser for BDHC data. See [`Bdhc`](bdhc::Bdhc).
- Data structure and parser for land data files (`land_data.narc`). See
  [`LandData`](land_data::LandData).
- Data structure and parser for map matrix files (`map_matrix.narc`). See
  [`MapMatrix`](map_matrix::MapMatrix).
- Data structure and parser for map prop animation list files
  (`bm_anime_list.narc`). See
  [`MapPropAnimationList`](map_prop_animation_list::MapPropAnimationList).
- Data structure and parser for map prop material & shapes files
  (`build_model_matshp.dat`). See
  [`MapPropMaterialShapes`](map_prop_material_shapes::MapPropMaterialShapes).

It also embeds data that is hard-coded inside the game's code. See the [`data`]
module.

## Reading data

Each data structure parser either has:

- A `from_bytes` function, to parse the associated data structure from a
  fixed-size array.
- A `parse_bytes` function, to parse the associated data structure from a
  slice.

Those functions expect the raw bytes to follow the same format as specified by
the game.

For more information, read [the
documentation](https://github.com/pret/pokeplatinum/blob/main/docs/maps/file_format_specifications.md)
in the `pret/pokeplatinum` repository.

### Reading data from the file system

You can parse data directly from a file present on the file system of your
machine, which makes sense for files that were extracted from a NARC archive,
or files that were not in a NARC in the first place.

Here's an example using the reader for the `build_model_matshp.dat` file:

```rust
use std::fs;
use sinjoh_plat::map_prop_material_shapes::MapPropMaterialShapes;

let raw_data = fs::read("/path/to/build_model_matshp.dat")?;
let map_prop_material_shapes = MapPropMaterialShapes::parse_bytes(&raw_data)?;
```

### Reading data from a NARC archive

If the data structures you want to read are contained in a NARC archive, which
is the case for most of them, you can also use the NARC reader from
[`sinjoh_nds`].

Here's an example using the reader for the `area_data.narc` file:

```rust
use sinjoh_nds::narc::reader::{NarcReader, NarcReaderFlags};
use sinjoh_plat::area_data::AreaData;

let mut narc_reader =
    NarcReader::read_from_file("/path/to/area_data.narc", NarcReaderFlags::default())?;

let file = narc_reader.get_file(0)?;
let area_data = AreaData::from_bytes(file.try_into().unwrap());
```
