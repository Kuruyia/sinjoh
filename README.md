# Sinjoh

This repository contains multiple Rust crates that allows you to manipulate the data structures used by Pokémon
Platinum.

The different crates are:

- [`sinjoh_nds`](./sinjoh_nds/): A library crate that contains generic data structures used by Nintendo DS games.
- [`sinjoh_plat`](./sinjoh_plat/): A library crate that contains data structures used by Pokémon Platinum.
- [`pokeplat_utils`](./pokeplat_utils/): A CLI tool based on the above crates used to manipulate Pokémon Platinum data
  files.

For more information, please refer to the documentation and README files in each crate.

## Documentation

The goal for library crates is to have their public APIs completely documented using the standard Rust documentation
tools. The documentation can be generated and opened using the following command:

```bash
cargo doc --open
```

## Contributing

Contributions are welcome! If you want to contribute to this project, please feel free to raise an issue or open a pull
request.

## Acknowledgements

This project could not have been possible without the following previous work:

- Martin Korth's [`gbatek` document](https://www.problemkaputt.de/gbatek.htm), which contains a lot of information about
  the Nintendo DS hardware and file formats.
- The [`pret/pokeplatinum` project](https://github.com/pret/pokeplatinum), for the reverse engineering of the Pokémon
  Platinum ROM and the data structures used by the game.
- scurest's [`apicula` project](https://github.com/scurest/apicula) and its [related documentation](https://raw.githubusercontent.com/scurest/nsbmd_docs/master/nsbmd_docs.txt),
  which documents the NSBxx file formats used by Pokémon Platinum, that contains the 3D models, textures and animations.

## License

This project is licensed under the terms of the Apache License, version 2.0. See the [LICENSE](./LICENSE) file for
details.
