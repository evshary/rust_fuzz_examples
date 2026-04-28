# honggfuzz-rs tutorial example

This example shows a minimal `honggfuzz-rs` setup. The target function is
`parse_port` in `src/lib.rs`. It parses UTF-8 bytes as a port number and
intentionally aborts on input `0`.

## Files

```text
src/lib.rs
src/bin/parse_port.rs
hfuzz_workspace/parse_port/input/seed.txt
```

## Setup

```bash
cd honggfuzz_rs_example
cargo install honggfuzz
```

On Debian or Ubuntu, Honggfuzz usually also needs:

```bash
sudo apt install build-essential binutils-dev libunwind-dev libblocksruntime-dev liblzma-dev lldb
```

## Run

```bash
cargo run
cargo hfuzz run parse_port
```

Expected normal output:

```text
Parsed port: 8080
```

## Saved inputs

- `hfuzz_workspace/parse_port/input/seed.txt` is the initial seed input. Its
  contents are `8080`.
- `hfuzz_workspace/parse_port/` stores crash files and reports.
- `hfuzz_target/` stores instrumented build output.

Inspect a saved file with:

```bash
xxd hfuzz_workspace/parse_port/<crash-file>.fuzz
cat hfuzz_workspace/parse_port/<crash-file>.fuzz
```

## Replay

```bash
cargo hfuzz run-debug parse_port hfuzz_workspace/parse_port/*.fuzz
```

`run-debug` starts LLDB by default.
