# Moss Launcher 

> **M**ods, **O**rganized, **S**hared & **S**ymlinked

[![Made with Rust](https://img.shields.io/badge/Made_with-Rust-dea584?style=for-the-badge&logo=rust&logoColor=white)](https://rust-lang.org)
[![Built with Nix Flake](https://img.shields.io/badge/Built_with-Nix_Flake-5277C3?style=for-the-badge&logo=nixos&logoColor=white)](https://nixos.org)
[![Status: not yet useable](https://img.shields.io/badge/status-not%20yet%20usable-red?style=for-the-badge)]()
[![GitHub License](https://img.shields.io/github/license/BitTim/moss?style=for-the-badge)](LICENSE)

Moss is a Nix-inspired Minecraft launcher written in Rust. Mods, resource packs and other shared game files
live once in a central local store and each instance is assembled from symlinks into that store.
Nothing is duplicated between instances, which saves disk space and makes instances lightweight.

> [!WARNING]
> This project is under heavy development and is **not yet usable**.
> Expect breaking changes, missing features, and incomplete documentation.

## Table of Contents
- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
- [Development](#development)
- [Contributing](#contributing)
- [Credits](#credits)
- [License](#license)

## Features
- Create, manage and launch profiles
- Manage mods and shared resources in a central store
- Import and export profiles in a shareable and declarative format
- Import a modpack from Modrinth or Curseforge into a profile
- Automatic garbage collection to optimize storage

## Installation
>[!NOTE]
> This section is not yet populated with relevant information

## Usage
>[!NOTE]
> This section is not yet populated with relevant information

## Development

### Prerequisites

This project uses [Nix](https://nixos.org) for a reproducible development environment. You'll need:

- [Nix](https://nixos.org/download) with flakes enabled
- (Optional) [direnv](https://direnv.net)

To enable flakes, add this to `~/.config/nix/nix.conf`:

```
experimental-features = nix-command flakes
```

### Entering the dev shell

```sh
git clone https://github.com/BitTim/moss.git
cd moss
nix develop
```

The shell provides the Rust toolchain pinned in [`rust-toolchain.toml`](rust-toolchain.toml) as well
as all necessary dependencies.

With direnv, run `direnv allow` once to automatically load the environment automatically.

>[!TIP]
> When using JetBrains RustRover it is recommended to also use the [direnv Everywhere](https://plugins.jetbrains.com/plugin/33482-direnv-everywhere) plugin

### Without Nix

>[!TIP]
> It is recommended to use Nix to simplify dependency management and have a reproducable environment

Install [rustup](https://rustup.rs/). The correct toolchain and components are picked up automatically from
'rust-toolchain.toml' on the first 'cargo' invocation.

Necessary dependencies are not automatically installed and must be installed manually:
- None

## Configuration
>[!NOTE]
> This section is not yet populated with relevant information

## Contributing
>[!NOTE]
> This section is not yet populated with relevant information

## Credits
This project would not be possible without the work of others:
- [PortableMC](https://github.com/theorzr/portablemc)
- [Ferinth](https://github.com/gorilla-devs/ferinth)
- [Furse](https://github.com/gorilla-devs/furse)

## License
Distributed under GPL-3.0 License. See [LICENSE](LICENSE) for details.
