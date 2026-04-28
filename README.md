# cargo-fuzz tutorial example

This repository contains a very small Rust crate plus a `cargo-fuzz` target.
The example is intentionally simple so it can be used in a tutorial.

The function under test is `parse_port` in `src/lib.rs`. It accepts bytes,
tries to interpret them as UTF-8 text, and parses a port number from that
text. There is also one intentional bug:

- the input `0` causes a panic

That makes it easy to demonstrate how fuzzing finds a crash automatically.

## Project layout

```text
src/lib.rs                  # code we want to fuzz
fuzz/Cargo.toml             # cargo-fuzz package
fuzz/fuzz_targets/parse_port.rs
```

## Prerequisites

Install `cargo-fuzz` if you do not already have it:

```bash
cargo install cargo-fuzz
```

Install the nightly Rust toolchain once (Require the nightly feature for the fuzzy test):

```bash
rustup toolchain install nightly
```

## Run the normal program

```bash
cargo run
```

Expected output:

```text
Parsed port: 8080
```

## Run the fuzzer

Start fuzzing with:

```bash
cargo +nightly fuzz run parse_port
```

`cargo-fuzz` will generate many different byte inputs and call
`rust_fuzzy::parse_port` with them. After enough mutations, it should discover
that the string `0` triggers a panic and stop with a crash report.

## What the generated files mean

While fuzzing runs, `cargo-fuzz` stores interesting inputs on disk.

- `fuzz/corpus/parse_port/` contains saved inputs that are useful for future
  fuzzing runs. These are not necessarily failures. They are just inputs the
  fuzzer decided to keep because they help explore new behavior.
- `fuzz/artifacts/parse_port/` contains inputs that triggered a failure, such as
  a panic or crash.

You can view the files with

```bash
# Raw bytes
xxd fuzz/xxxx/parse_port/<file>
# If readable
cat fuzz/xxxx/parse_port/<file>
```

## Reproduce the case

You can rerun the exact input with

```bash
# crash
cargo +nightly fuzz run parse_port fuzz/artifacts/parse_port/<crash-file>
# non-crash
cargo +nightly fuzz run parse_port fuzz/corpus/parse_port/<file>
```
