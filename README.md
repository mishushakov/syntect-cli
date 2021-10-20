# syntect-cli

Minimal CLI for syntect designed for embedding syntect syntax highlighting into other projects

## Usage

`syntect-cli` accepts STDIN input with code and writes the result to STDOUT

```sh
syntect-cli --lang <lang> --theme <theme>
```

Example

```sh
cat src/main.rs | syntect-cli --lang rust --theme base16-ocean.dark > index.html
```

## Build

1. [Get Rust](https://www.rust-lang.org/learn/get-started)
2. Clone the repository
3. Run `cargo build --release`
