## rust
- [Install Rust](#install-rust)
- [Check Versions](#check-versions)
- [VSCODE plugins](#vscode-plugins)
- [Build a rust program](#build-a-rust-program)

Rust is a system programming language.

# Install Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
# Check Versions

```bash
rustc --version
rustc 1.76.0 (07dca489a 2024-02-04)
cargo -version
cargo 1.76.0 (c84b36747 2024-01-18)
```

Rust comes with it's own package manager called cargo.

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
