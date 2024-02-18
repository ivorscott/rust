## rust
- [Install Rust](#install-rust)
- [Cargo](#cargo)
- [VSCODE plugins](#vscode-plugins)
- [Build a rust program](#build-a-rust-program)

Rust is a system programming language.

# Install Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
# Restart current shell

# Check version of Rust compiler
rustc --version
rustc 1.76.0 (07dca489a 2024-02-04)
```
# Cargo

Rust comes with it's own package manager called cargo.

```bash
# Rust package manager
cargo -version
cargo 1.76.0 (c84b36747 2024-01-18)

# Initialize a new project
cargo new hello_world

# Run application
cd hello_world
cargo run

# Build application
cargo build

# Build for production
cargo build --release
```

`cargo new <name>` create a new package with a Cargo.toml file that track package dependencies.

`cargo build --release` creates a release folder.

# VSCODE plugins

- [Rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

# Build a rust program

Rust files end with `.rs`. Before running an application the code must be compiled.

```bash
cat > hello/main.rs <<EOF
fn main() {
    println!("hello world")
}
EOF

# Building program at specific path
rustc --out-dir hello hello/main.rs
./hello/main

# Building program in current directory
cd hello
rustc main.rs
./main
```
