# cargo-fuzz tutorial example

This example shows a minimal `cargo-fuzz` setup. The target function is
`parse_port` in `src/lib.rs`. It parses UTF-8 bytes as a port number and
intentionally panics on input `0`.

## Files

```text
src/lib.rs
fuzz/Cargo.toml
fuzz/fuzz_targets/parse_port.rs
```

## Setup

```bash
cd cargo_fuzz_example
cargo install cargo-fuzz
rustup toolchain install nightly
```

## Run

```bash
cargo run
cargo +nightly fuzz run parse_port
```

Expected normal output:

```text
Parsed port: 8080
```

## Saved inputs

- `fuzz/corpus/parse_port/` stores interesting non-crashing inputs.
- `fuzz/artifacts/parse_port/` stores crashing inputs.

Inspect a saved file with:

```bash
xxd fuzz/artifacts/parse_port/<file>
cat fuzz/artifacts/parse_port/<file>
```

## Replay

```bash
cargo +nightly fuzz run parse_port fuzz/artifacts/parse_port/<crash-file>
```
