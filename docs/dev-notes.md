# NEUTRINO

# RUST
Let's update our rust:
```
./rustup update
```

New project - hey, let's use __cargo__
```
cargo new <project name>
```

if the directory already exists, we can try instead:
```
cargo init <project dir name>
```

To get rust to work with a version control system, to generate a starter package, run with:
```
cargo new --vcs=git
```
It's self explanitory, in this example it will generate git files

## CARGO
When a project is generated a _toml_ file is created:
```
- Cargo.toml

[package]
name = "hello_cargo"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
```

_This file is in the TOML (Tom’s Obvious, Minimal Language) format, which is Cargo’s configuration format_

[TOML](https://toml.io/en/)


