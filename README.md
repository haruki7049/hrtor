# hrtor

hrtor("hərtər", "ハルター", "ハーター", "ヘルター", "ヘルトル") is HaRuki's scalable line ediTOR.

## Concepts

1. Hrtor is a text-editor of the you, by the you, for the you.
1. Do all things about Hrtor text-editor.
1. All things in Hrtor text-editor MUST be extensible.

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

## hrtor's background

Originally, there was only one editor, the 'GNU ed editor'. As time went by, many editors were born, including vim, neovim, emacs, and other popular editors in existence today. Finally, GUI-based editors such as VSCode and JetBrains were born. However, line editors are no longer in use today. I thought, "Why not develop an extensible line editor? Wouldn't it be interesting to develop an extensible line editor?"

I made it!
GitHub Wiki is [here](https://github.com/haruki7049/hrtor/wiki).
