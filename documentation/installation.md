# Installation

## System

This software requires [Ubuntu 20.04.4 LTS](https://releases.ubuntu.com/20.04.4/)

### Update

```bash
sudo apt-get update
```

### Rust

Install Rust using the [rust-lang.org documentation](https://www.rust-lang.org/tools/install)

## Fetch source code

```bash
git clone https://github.com/WasmEdge/FaaS.git
```

## Configure

```bash
cd FaaS/wasm-fass
```

## Build and run

### Build local package and all dependencies (debug)

```bash
cargo build
./target/debug/wasm-faas
```

### Build with optimizations (release)

```bash
cargo build --release
./target/release/wasm-faas
```
