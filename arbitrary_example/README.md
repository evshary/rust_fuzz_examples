# arbitrary tutorial example

This example shows a minimal `arbitrary` setup. Unlike the fuzzing examples,
`arbitrary` is not a fuzzer by itself. It turns raw bytes into structured Rust
values that a fuzzer or test can use.

The target code parses a port number and intentionally panics on input `0`.

## Files

```text
src/lib.rs
src/main.rs
```

## Setup

```bash
cd arbitrary_example
```

## Run

```bash
cargo run
cargo test
```

Expected normal output:

```text
Generated input: PortInput { port: 8080, padded: false }
Parsed port: 8080
```

## What it demonstrates

- `#[derive(Arbitrary)]` on a structured input type
- `arbitrary::Unstructured` turning a byte slice into a Rust value
- a helper that feeds structured inputs into the target function

## Key idea

This example does not fuzz by itself. Instead, it shows how to convert raw
bytes into a structured `PortInput`. That same pattern is commonly combined
with fuzzers such as `cargo-fuzz`, AFL, or Honggfuzz.
