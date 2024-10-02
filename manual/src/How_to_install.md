# How to install Hrtor HEAD

Hrtor HEAD can be used by:

1. with Nix package manager
1. with Rust-lang's Cargo

Just use [Nix package manager](https://github.com/nixos/nix) such as:

```bash
nix run github:haruki7049/hrtor#hrtor
# OR
nix run github:haruki7049/hrtor#hrtor -- your_file.txt
```

Or run with [Cargo](https://github.com/rust-lang/cargo) as:

```
cargo install --git https://github.com/haruki7049/hrtor
```
