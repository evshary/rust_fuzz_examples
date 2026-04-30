# Rust Fuzzing and Property Testing Examples

This repository contains six small Rust testing tutorials. They fall into
two broad categories:

- fuzz testing: `cargo-fuzz`, `afl.rs`, `honggfuzz-rs`, and `libafl`
- input generation and property testing: `arbitrary` and `proptest`

Each example uses the same basic `parse_port` idea with an intentional bug on
input `0`, so you can compare the approaches without changing the target much.

## Category Difference

- Fuzz testing tools start from raw bytes, mutate them automatically,
  and use execution feedback such as coverage or crashes to decide what to try next.
- Input generation and property testing tools focus on describing useful input shapes or expected properties.
  They do not replace a fuzzer engine,
  but they help you generate more meaningful inputs or verify broader invariants.

## Examples

### Fuzz Testing

| Example | Tool | Example README | Upstream project |
| --- | --- | --- | --- |
| `cargo_fuzzy_example` | `cargo-fuzz` | [cargo_fuzzy_example/README.md](./cargo_fuzzy_example/README.md) | [rust-fuzz/cargo-fuzz](https://github.com/rust-fuzz/cargo-fuzz) |
| `afl_rs_example` | `afl.rs` | [afl_rs_example/README.md](./afl_rs_example/README.md) | [rust-fuzz/afl.rs](https://github.com/rust-fuzz/afl.rs) |
| `honggfuzz_rs_example` | `honggfuzz-rs` | [honggfuzz_rs_example/README.md](./honggfuzz_rs_example/README.md) | [rust-fuzz/honggfuzz-rs](https://github.com/rust-fuzz/honggfuzz-rs) |
| `libafl_example` | `LibAFL` | [libafl_example/README.md](./libafl_example/README.md) | [AFLplusplus/LibAFL](https://github.com/AFLplusplus/LibAFL) |

### Input Generation and Property Testing

| Example | Tool | Example README | Upstream project |
| --- | --- | --- | --- |
| `arbitrary_example` | `arbitrary` | [arbitrary_example/README.md](./arbitrary_example/README.md) | [rust-fuzz/arbitrary](https://github.com/rust-fuzz/arbitrary) |
| `proptest_example` | `proptest` | [proptest_example/README.md](./proptest_example/README.md) | [proptest-rs/proptest](https://github.com/proptest-rs/proptest) |

## Comparison

| Tool | Features | Advantages | Tradeoffs | When to use |
| --- | --- | --- | --- | --- |
| `cargo-fuzz` | Rust wrapper around libFuzzer with coverage-guided fuzzing and the standard `fuzz/` project layout | Usually the most familiar Rust fuzzing workflow, easy to add to a crate, good default choice for tutorials and CI experiments | Requires nightly Rust and is mostly centered on the libFuzzer style of harness | Use it when you want the most common Rust fuzzing setup or a simple place to start |
| `afl.rs` | Rust bindings for AFL/AFL++ with explicit input and output directories and seed-based mutation | Makes the corpus and crash workflow very visible, great for learning seed inputs and AFL-style fuzzing | Setup can be more system-dependent, and the workflow feels less like standard Cargo than `cargo-fuzz` | Use it when you want to learn AFL-style fuzzing or work with seed files and output queues directly |
| `honggfuzz-rs` | Rust bindings for Honggfuzz with built-in crash analysis workflow and `hfuzz_workspace/` output | Good if you want Honggfuzz specifically or want its reporting and debugger-oriented replay flow | Often needs extra native packages, and replay/debugging is a little more toolchain-dependent | Use it when you want to try Honggfuzz itself or compare a third workflow against `cargo-fuzz` and AFL |
| `LibAFL` | A modular fuzzing framework where you compose inputs, mutators, schedulers, feedback, and execution yourself | Extremely flexible and good for learning or building custom fuzzers | More moving parts than the other examples, so the learning curve is higher | Use it when you want to understand or customize the internals of a fuzzer instead of using a fixed workflow |
| `arbitrary` | Structured input generation from raw bytes with `Arbitrary` derives and `Unstructured` | Lets you model inputs as real Rust types and combine structure-aware generation with a fuzzer or test harness | Not a standalone fuzzer, and it does not provide coverage guidance by itself | Use it when raw bytes are too low-level and you want fuzzers or tests to exercise structured inputs |
| `proptest` | Property-based testing with strategies, shrinking, and persisted regression cases | Works on stable Rust, integrates directly with `cargo test`, and is great for checking invariants instead of only searching for crashes | Not coverage-guided fuzzing, and you need to define useful properties and generators yourself | Use it when you can describe expected behavior as properties and want minimal failing examples automatically |

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

`LibAFL`:

```bash
cd libafl_example
cargo run
cargo test
```

`proptest`:

```bash
cd proptest_example
cargo run
cargo test parse_port_matches_std -- --ignored
```

`arbitrary`:

```bash
cd arbitrary_example
cargo run
cargo test
```
