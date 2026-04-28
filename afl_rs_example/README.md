# afl.rs tutorial example

This example shows a minimal `afl.rs` setup. The target function is
`parse_port` in `src/lib.rs`. It parses UTF-8 bytes as a port number and
intentionally aborts on input `0`.

## Files

```text
src/lib.rs
src/bin/fuzz_parse_port.rs
in/seed.txt
```

## Setup

```bash
cd afl_rs_example
cargo install cargo-afl
```

If `cargo afl` reports missing system setup, install AFL++ and run:

```bash
cargo afl system-config
```

## Run

```bash
cargo run
cargo afl build --features afl-harness --bin fuzz_parse_port
cargo afl fuzz -i in -o out target/debug/fuzz_parse_port
```

Expected normal output:

```text
Parsed port: 8080
```

## Saved inputs

- `in/seed.txt` is the initial seed input. Its contents are `8080`.
- `out/default/crashes/` stores crashing inputs.

Inspect a saved file with:

```bash
xxd out/default/crashes/<crash-file>
cat out/default/crashes/<crash-file>
```

## Replay

```bash
target/debug/fuzz_parse_port < out/default/crashes/<crash-file>
```
