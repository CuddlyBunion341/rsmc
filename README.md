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
