# Rune as a prototyping tool
Using [Rune](https://rune-rs.github.io) as a way to iterate quickly in Rust.

Here are some examples solving [Advent of Code](https://adventofcode.com) in Rune and then oxidize them to Rust.

## The idea
> Can we write our program in Rune to get a battery included, fast complied with simple error and memory management?

1. Write code in Rune where
    - no need to add dependencies
    - no need to handle memory
    - dynamic types
    - runtime in WebAssembly to run in browser
1. Copy the code into Rust-file and solve the errors that emerge.
1. Profit!

## Try it
```sh
# Run the Rune example which loads a script and runs it in Rust (could be run from cargo-rune)
cargo run --bin rune
# Run the re-written Rune to Rust code.
cargo run --bin rust
```