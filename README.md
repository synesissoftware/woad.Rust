# woad.Rust <!-- omit in toc -->

Minimal ANSI terminal colour codes, for Rust

![Language](https://img.shields.io/badge/Rust-000000?style=flat&logo=rust&logoColor=white)
[![License](https://img.shields.io/badge/License-BSD_3--Clause-blue.svg)](https://opensource.org/licenses/BSD-3-Clause)
[![Crates.io](https://img.shields.io/crates/v/woad.svg)](https://crates.io/crates/woad)
[![GitHub release](https://img.shields.io/github/v/release/synesissoftware/woad.Rust.svg)](https://github.com/synesissoftware/woad.Rust/releases/latest)
[![Last Commit](https://img.shields.io/github/last-commit/synesissoftware/woad.Rust)](https://github.com/synesissoftware/woad.Rust/commits/master)
[![CI](https://github.com/synesissoftware/woad.Rust/actions/workflows/ci.yml/badge.svg)](https://github.com/synesissoftware/woad.Rust/actions/workflows/ci.yml)


## Table of Contents <!-- omit in toc -->

- [Introduction](#introduction)
- [Installation](#installation)
- [Components](#components)
- [Project Information](#project-information)
  - [Where to get help](#where-to-get-help)
  - [Contribution guidelines](#contribution-guidelines)
  - [Dependencies](#dependencies)
    - [Efferent (fan-out)](#efferent-fan-out)
    - [Development Dependencies](#development-dependencies)
    - [Afferent (fan-in)](#afferent-fan-in)
  - [Related projects](#related-projects)
  - [License](#license)


## Introduction

**woad** provides the smallest useful set of fixed ANSI SGR colour sequences for library authors. It is not a console or TUI framework.

**woad.Rust** is the **Rust** implementation.


## Installation

Reference in **Cargo.toml** in the usual way:

```toml
woad = { version = "0" }
```


## Components

**woad.Rust** ships SGR string constants (`RESET`, `FG_*`, `BG_*`, including bright variants) and `VERSION`. TTY/stream gating and Windows virtual-terminal opt-in are not implemented yet.

```rust
use woad::{FG_GREEN, RESET};

fn main() {
    println!("{FG_GREEN}ok{RESET}");
}
```


## Project Information


### Where to get help

[GitHub Page](https://github.com/synesissoftware/woad.Rust "GitHub Page")


### Contribution guidelines

Defect reports, feature requests, and pull requests are welcome on https://github.com/synesissoftware/woad.Rust.


### Dependencies


#### Efferent (fan-out)

None.


#### Development Dependencies

None.


#### Afferent (fan-in)

None (currently).


### Related projects

* [**woad.Python**](https://github.com/synesissoftware/woad.Python/)
* [**woad.Ruby**](https://github.com/synesissoftware/woad.Ruby/)


### License

**woad.Rust** is released under the 3-clause BSD license. See [LICENSE](./LICENSE) for details.


<!-- ########################### end of file ########################### -->
