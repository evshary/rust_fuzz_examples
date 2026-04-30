# libafl tutorial example

This example shows a small `LibAFL` setup. It is not a full production fuzzer.
Instead, it demonstrates the core idea behind `LibAFL`: assemble fuzzing
building blocks such as inputs, mutation, and a target function.

The target code parses a port number and intentionally panics on input `0`.

## Files

```text
src/lib.rs
src/main.rs
```

## Setup

```bash
cd libafl_example
```

## Run

```bash
cargo run
cargo test
```

Expected normal output includes:

```text
Seed input: 8080
```

## What it demonstrates

- `BytesInput` as a fuzzing input type
- a LibAFL mutator producing new byte inputs from a seed
- a target function that runs on each mutated input

## Key idea

Unlike `cargo-fuzz`, this example does not try to hide the framework pieces.
The point of `LibAFL` is that you can combine the building blocks yourself.
