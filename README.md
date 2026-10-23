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
- [Developer Experience](#developer-experience)
- [Installation](#installation)
- [Components](#components)
  - [Fixed sequences](#fixed-sequences)
  - [Stream-conditional sequences](#stream-conditional-sequences)
  - [Colour policy](#colour-policy)
  - [Version](#version)
- [Project Information](#project-information)
  - [Where to get help](#where-to-get-help)
  - [Contribution guidelines](#contribution-guidelines)
  - [Minimum Supported Rust Version (MSRV)](#minimum-supported-rust-version-msrv)
  - [Dependencies](#dependencies)
    - [Efferent (fan-out)](#efferent-fan-out)
    - [Development Dependencies](#development-dependencies)
    - [Afferent (fan-in)](#afferent-fan-in)
  - [Related projects](#related-projects)
  - [License](#license)


## Introduction

**woad** provides fixed ANSI SGR sequences, and stream-conditional counterparts that emit only when the destination warrants colour. It is not a console or TUI framework.

**woad.Rust** is the **Rust** implementation. This alpha matches the dynamic API of **woad** off Windows. Console-mode changes and Windows handle currencies are not included.


## Developer Experience

**woad.Rust** builds with Cargo. The minimum supported Rust version is **1.74**.

```bash
cargo test
cargo run --example colour_for
```

`colour_for` prints colour only when stdout or stderr warrants it. Pipe it through `cat`, then repeat with `WOAD_COLOUR=always` and `WOAD_COLOUR=never`. Fixed sequences need no policy check. The conditional helpers are under [Components](#components).


## Installation

Reference in **Cargo.toml** in the usual way:

```toml
woad = { version = "0.1.0-alpha1" }
```


## Components


### Fixed sequences

String constants: `RESET`, `BOLD`, `BLINKING`, `FG_*`, and `BG_*`, the colours each in normal and `BRIGHT` variants. They always emit.

```rust
use woad::{FG_GREEN, RESET};

fn main() {
    println!("{FG_GREEN}ok{RESET}");
}
```


### Stream-conditional sequences

`seq_for`, `fg_for`, and `bg_for` yield the sequence when the destination warrants colour, and `""` when it does not. `reset_for`, `bold_for`, and `blinking_for` are the attribute forms. The destination is any [`std::io::IsTerminal`](https://doc.rust-lang.org/std/io/trait.IsTerminal.html) value (`Stdout`, `Stderr`, `Stdin`, `File`, and on Unix a borrowed file descriptor).

```rust
let stdout = std::io::stdout();

println!(
    "{}ok{}",
    woad::seq_for(woad::FG_GREEN, &stdout),
    woad::reset_for(&stdout),
);
```

On Unix, `fd_is_tty`, `fd_supports_colour`, and `seq_for_fd` take [`AsFd`](https://doc.rust-lang.org/std/os/fd/trait.AsFd.html). A missing destination is `supports_colour_absent` / `seq_for_absent` (the C null-stream case: the policy alone).


### Colour policy

The environment is read once, on first need, until `refresh()`:

| Variable | Value | Effect |
| --- | --- | --- |
| `WOAD_COLOUR`, or `WOAD_COLOR` | `1`, `always`, `force`, `on`, `true`, `yes` | Force colour |
| `WOAD_COLOUR`, or `WOAD_COLOR` | `0`, `false`, `never`, `no`, `none`, `off` | Inhibit colour |
| `WOAD_COLOUR`, or `WOAD_COLOR` | `auto`, or anything unrecognised | Defer to the destination |
| `NO_COLOR` | set and non-empty | Inhibit colour |
| `CLICOLOR_FORCE` | set, non-empty, and not `0` | Force colour |
| `CLICOLOR` | `0` | Inhibit colour |

`WOAD_COLOUR` / `WOAD_COLOR` are case-insensitive. In the deferring case, a `TERM` of `dumb` also suppresses colour. `console_supports_ansi()` is always `true` in this alpha.


### Version

`VERSION` and `version_string()` are the SemVer string, including the prerelease label (`0.1.0-alpha1`). `version()` is the packed **woad** `WOAD_VER` (`0x00010041`; low byte `0x41` is alpha 1).


## Project Information


### Where to get help

[GitHub Page](https://github.com/synesissoftware/woad.Rust "GitHub Page")


### Contribution guidelines

Defect reports, feature requests, and pull requests are welcome on https://github.com/synesissoftware/woad.Rust.


### Minimum Supported Rust Version (MSRV)

The declared Minimum Supported Rust Version (MSRV) for **woad.Rust** is **1.74**.

This MSRV guarantee applies to the library crate itself, its runtime dependencies (`[dependencies]`), and its build dependencies (`[build-dependencies]`). Downstream consumers compiling this crate as a dependency are guaranteed that it builds cleanly on the declared MSRV toolchain.

Development dependencies (`[dev-dependencies]`, such as benchmarking frameworks like **criterion**) may require newer Rust toolchains for local development or performance testing. These dev-dependencies are never fetched or compiled by downstream consumers and do not affect the library's MSRV guarantee.


### Dependencies


#### Efferent (fan-out)

None.


#### Development Dependencies

None.


#### Afferent (fan-in)

None (currently).


### Related projects

* [**woad**](https://github.com/synesissoftware/woad/)
* [**woad.Python**](https://github.com/synesissoftware/woad.Python/)
* [**woad.Ruby**](https://github.com/synesissoftware/woad.Ruby/)


### License

**woad.Rust** is released under the 3-clause BSD license. See [LICENSE](./LICENSE) for details.


<!-- ########################### end of file ########################### -->
