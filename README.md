# Rust Fuzzing Examples

This repository contains three small Rust fuzzing tutorials. Each example uses a
simple `parse_port` function with an intentional bug on input `0`, but they use
different fuzzing tools so you can compare the workflows.

## Examples

- [cargo_fuzzy_example](./cargo_fuzzy_example/README.md): a `cargo-fuzz`
  example using `cargo +nightly fuzz run`.
- [afl_rs_example](./afl_rs_example/README.md): an `afl.rs` example using
  `cargo afl`.
- [honggfuzz_rs_example](./honggfuzz_rs_example/README.md): a `honggfuzz-rs`
  example using `cargo hfuzz run`.

## Which one to use

- Choose `cargo_fuzzy_example` if you want the more common libFuzzer-style Rust
  workflow with a dedicated `fuzz/` directory.
- Choose `afl_rs_example` if you want to learn the AFL-style workflow with seed
  inputs in `in/` and generated results in `out/`.
- Choose `honggfuzz_rs_example` if you want a Honggfuzz-based workflow with the
  default `hfuzz_workspace/` and `hfuzz_target/` directories.

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

For `honggfuzz-rs`:

```bash
cd honggfuzz_rs_example
cargo run
cargo hfuzz run parse_port
```

## Notes

- Each example has its own `README.md` with more detail.
- The `cargo-fuzz` example requires nightly Rust for fuzzing.
- The `afl.rs` example may require AFL++ system setup depending on your Linux
  environment.
- The `honggfuzz-rs` example may require native system packages depending on
  your platform.
