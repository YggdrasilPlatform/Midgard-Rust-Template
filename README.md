# Midgard Rust Template

Requires the Rust nighly version with `thumbv7em-none-eabihf` toolchain to be installed.

Project ELF file can be found under `target/thumbv7em-none-eabihf/debug/midgard_rust_template` and example ELF files under `target/thumbv7em-none-eabihf/debug/examples/[example_name]`

## Setup

```
$ rustup target add thumbv7em-none-eabihf
$ cargo build
$ cargo build --example=blinky
```
