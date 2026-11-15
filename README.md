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
- [Environment](#environment)
- [Examples](#examples)
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

**woad** provides the smallest useful set of fixed ANSI SGR colour sequences for library authors. It is not a console or TUI framework.

**woad.Rust** is the **Rust** implementation. The declared MSRV is **1.74**.

It has **no dependencies** on any other crate.


## Developer Experience

**woad.Rust** builds with Cargo. The declared MSRV is **1.74**, and the crate has no runtime dependencies.

```bash
cargo test --all-targets
cargo run --example colour
```

The smallest useful call emits a sequence only when the writer supports colour:

```rust
use woad::{seq_for, FG_GREEN, RESET};

fn main() {
    let out = std::io::stdout();

    print!("{}ok{}", seq_for(FG_GREEN, &out), seq_for(RESET, &out));
}
```

That is [examples/colour/main.rs](./examples/colour/main.rs). Fixed sequences (`BOLD`, `BLINKING`, `FG_*`, `BG_*`) always contain the SGR bytes. `colour_policy`, `supports_colour`, and `seq_for` apply the cached environment policy. `is_tty` reports attachment only. `refresh` drops that cache.

The example prints green `ok` only when stdout supports colour. A pipe is not a terminal, so automatic policy drops the sequences. `WOAD_COLOUR` and `NO_COLOR` override that. The accepted forms are listed under [Environment](#environment).

```bash
# Automatic policy, attached to a terminal: green "ok" when TERM is not dumb.
cargo run --example colour

# The same program, piped: plain "ok".
cargo run --example colour | cat

# Force colour, including through the pipe.
WOAD_COLOUR=1 cargo run --example colour | cat
CLICOLOR_FORCE=1 cargo run --example colour | cat

# Inhibit colour, including on a terminal.
WOAD_COLOUR=never cargo run --example colour
NO_COLOR=1 cargo run --example colour

# Automatic policy, dumb terminal: plain "ok".
TERM=dumb cargo run --example colour
```


## Installation

Reference in **Cargo.toml** in the usual way:

```toml
woad = { version = "0.1.0-alpha1" }
```


## Components

**woad.Rust** ships fixed SGR sequences (`BLINKING`, `BOLD`, `RESET`, `FG_*`, `BG_*`, including bright variants), `VERSION`, and the portable colour policy: `colour_policy`, `refresh`, `is_tty`, `supports_colour`, and `seq_for`. On Unix a terminal supports colour when the policy is automatic and `TERM` is not `dumb`. Windows virtual-terminal mode and mintty PTY recognition are not implemented.


## Environment

`colour_policy()` returns `COLOUR_POLICY_AUTO` (0), `COLOUR_POLICY_FORCE` (1), or `COLOUR_POLICY_INHIBIT` (2). The result is cached until `refresh` is invoked.

A variable that is unset or empty is absent. Values are not trimmed. The first present variable in this order wins:

1. Non-empty `WOAD_COLOUR`, otherwise non-empty `WOAD_COLOR`. Compared case-insensitively. Truey forces: `1`, `always`, `force`, `on`, `true`, `yes`. Falsey inhibits: `0`, `false`, `never`, `no`, `none`, `off`. Anything else, including `auto`, selects auto and does not fall through;
2. Non-empty `NO_COLOR` inhibits, whatever its text is;
3. Non-empty `CLICOLOR_FORCE` other than the exact bytes `0` forces. Exact `0` does not force and does not inhibit;
4. Non-empty `FORCE_COLOR` other than exact `0` or `false` (any capitalisation) forces. Exact `0` and `false` do not force and do not inhibit;
5. `CLICOLOR` equal to the exact bytes `0` inhibits. Any other value is ignored;
6. Otherwise auto.

`TERM` equal to `dumb`, in any capitalisation, is cached beside the policy. It is not a policy value. It suppresses colour only when the policy is auto. `force` ignores the terminal and `TERM`. `inhibit` never emits colour.

On Unix, auto plus a terminal plus a `TERM` that is not `dumb` means colour. There is no console probe. `WOAD_SET_CONSOLE_MODE` is not read in this alpha.


## Examples

Examples are provided in the **examples** directory. A catalogue is in [EXAMPLES.md](./EXAMPLES.md).


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

None. Windows virtual-terminal mode and mintty PTY recognition are not implemented, so this crate does not depend on **windows-sys**.


#### Development Dependencies

None.


#### Afferent (fan-in)

None (currently).


### Related projects

* [**woad**](https://github.com/synesissoftware/woad/) (**C**, **C++**)
* [**woad.Python**](https://github.com/synesissoftware/woad.Python/)
* [**woad.Ruby**](https://github.com/synesissoftware/woad.Ruby/)


### License

**woad.Rust** is released under the 3-clause BSD license. See [LICENSE](./LICENSE) for details.


<!-- ########################### end of file ########################### -->
