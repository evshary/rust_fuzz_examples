# Rust Fuzzing Examples

This repository contains two small Rust fuzzing tutorials. Both examples use a
simple `parse_port` function with an intentional bug on input `0`, but they use
different fuzzing tools so you can compare the workflows.

## Examples

- [cargo_fuzzy_example](./cargo_fuzzy_example/README.md): a `cargo-fuzz`
  example using `cargo +nightly fuzz run`.
- [afl_rs_example](./afl_rs_example/README.md): an `afl.rs` example using
  `cargo afl`.

## Which one to use

- Choose `cargo_fuzzy_example` if you want the more common libFuzzer-style Rust
  workflow with a dedicated `fuzz/` directory.
- Choose `afl_rs_example` if you want to learn the AFL-style workflow with seed
  inputs in `in/` and generated results in `out/`.

## Quick start

For `cargo-fuzz`:

```bash
cd cargo_fuzzy_example
cargo run
cargo +nightly fuzz run parse_port
```

For `afl.rs`:

```bash
cd afl_rs_example
cargo run
cargo afl build --features afl-harness --bin fuzz_parse_port
cargo afl fuzz -i in -o out target/debug/fuzz_parse_port
```

## Notes

- Each example has its own `README.md` with more detail.
- The `cargo-fuzz` example requires nightly Rust for fuzzing.
- The `afl.rs` example may require AFL++ system setup depending on your Linux
  environment.
