# hrtor

hrtor("hərtər", "ハルター", "ハーター", "ヘルター", "ヘルトル") is HaRuki's scalable line ediTOR.

## Usage

- with Nix package manager

```bash
nix run github:haruki7049/hrtor#hrtor
# OR
nix run github:haruki7049/hrtor#hrtor -- your_file.txt
```

- with Rust-lang's Cargo

```bash
# Install hrtor with:
cargo install --git https://github.com/haruki7049/hrtor
```

## How to add Actions

1. Write down a function by Rust, on any file in `src/processor/actions/` directory, Such as `src/processor/actions/foobar.rs`.
2. For Hrtor internal communications, Add a element to `crate::processor::parser::Action` in `src/processor/parser.rs` file.
3. Edit eval method for HrtorProcessor, written on `src/processor.rs`
4. Then, you can run Hrtor with your action!!
