# Rust Fuzzing Examples

This repository contains three small Rust fuzzing tutorials. Each example uses
the same `parse_port` function with an intentional bug on input `0`, so you can
compare the tooling without changing the target code much.

## Examples

| Example | Tool | Example README | Upstream project |
| --- | --- | --- | --- |
| `cargo_fuzzy_example` | `cargo-fuzz` | [cargo_fuzzy_example/README.md](./cargo_fuzzy_example/README.md) | [rust-fuzz/cargo-fuzz](https://github.com/rust-fuzz/cargo-fuzz) |
| `afl_rs_example` | `afl.rs` | [afl_rs_example/README.md](./afl_rs_example/README.md) | [rust-fuzz/afl.rs](https://github.com/rust-fuzz/afl.rs) |
| `honggfuzz_rs_example` | `honggfuzz-rs` | [honggfuzz_rs_example/README.md](./honggfuzz_rs_example/README.md) | [rust-fuzz/honggfuzz-rs](https://github.com/rust-fuzz/honggfuzz-rs) |

## Comparison

| Tool | Features | Advantages | Tradeoffs | When to use |
| --- | --- | --- | --- | --- |
| `cargo-fuzz` | Rust wrapper around libFuzzer with coverage-guided fuzzing and the standard `fuzz/` project layout | Usually the most familiar Rust fuzzing workflow, easy to add to a crate, good default choice for tutorials and CI experiments | Requires nightly Rust and is mostly centered on the libFuzzer style of harness | Use it when you want the most common Rust fuzzing setup or a simple place to start |
| `afl.rs` | Rust bindings for AFL/AFL++ with explicit input and output directories and seed-based mutation | Makes the corpus and crash workflow very visible, great for learning seed inputs and AFL-style fuzzing | Setup can be more system-dependent, and the workflow feels less like standard Cargo than `cargo-fuzz` | Use it when you want to learn AFL-style fuzzing or work with seed files and output queues directly |
| `honggfuzz-rs` | Rust bindings for Honggfuzz with built-in crash analysis workflow and `hfuzz_workspace/` output | Good if you want Honggfuzz specifically or want its reporting and debugger-oriented replay flow | Often needs extra native packages, and replay/debugging is a little more toolchain-dependent | Use it when you want to try Honggfuzz itself or compare a third workflow against `cargo-fuzz` and AFL |

## Quick Start

`cargo-fuzz`:

```bash
cd cargo_fuzzy_example
cargo run
cargo +nightly fuzz run parse_port
```

`afl.rs`:

```bash
cd afl_rs_example
cargo run
cargo afl build --features afl-harness --bin fuzz_parse_port
cargo afl fuzz -i in -o out target/debug/fuzz_parse_port
```

`honggfuzz-rs`:

```bash
cd honggfuzz_rs_example
cargo run
cargo hfuzz run parse_port
```
