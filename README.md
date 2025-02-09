# rsmc

A stupid little Minecraft clone written in Rust, powered by the Bevy engine.

## Features

* Procedual world generation using 3D Perlin noise
* Custom terrain mesher based on face culling
* Custom Client / Server architecture using Renet
* Data serialization using bincode serde
* World update synchronization between game clients
* World physics using rapier
* World updates using primitive ray casting
* Modular architecture using ECS

## Installation

### Prerequisites

* [Cargo](https://doc.rust-lang.org/cargo/)

### Default setup

Just run the cargo command to install the dependencies and start the game:

```bash
cargo run --bin server
cargo run --bin client

```

### More optimal setup

Release Builds (for better performance):

```bash
cargo run --bin server --release
cargo run --bin client --release
```

Dynamic Linking (to reduce compile times):
```bash
cargo run --bin server --features dynamic_linking
cargo run --bin client --features dynamic_linking
```

Automatic Reloading (with [cargo watch](https://docs.rs/crate/cargo-watch)):
```bash
cargo watch -x 'run --bin server'
cargo watch -x 'run --bin client'
```

### Installation on NixOS

Nix shell can be used to run the code using the given [Nix Shell Config File](./shell.nix). This will automatically install rust and the bevy dependencies.
Strongly inspired by the [Bevy NixOS installation guide](https://github.com/bevyengine/bevy/blob/latest/docs/linux_dependencies.md)

```bash
nix-shell --run "cargo run --bin server"
nix-shell --run "cargo run --bin client"
```

## Implementation notes

### Terrain generation

1. Step (terrain shaping)
- 3D fractal perlin noise with low frequency, squash factor to determine terrain density at xyz.
- 2D fractal perlin noise with higher frequency serving as a base layer. Use splines for ridged height map.
- Subtract 3D fractal perlin noise to get cave generation.
- Use strongly flattened 3D noise perlin or worley noise for different rock layers.

- Place stone (base)

2. Step (Decorating)
- Place grass on blocks where there is air above
- Use white noise to place some ores, grow them randomly?


