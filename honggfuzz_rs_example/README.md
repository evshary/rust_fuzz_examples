# honggfuzz-rs tutorial example

This example lives in the `honggfuzz_rs_example/` folder. It contains a very
small Rust crate plus a `honggfuzz-rs` harness.

The function under test is `parse_port` in `src/lib.rs`. It accepts bytes,
tries to interpret them as UTF-8 text, and parses a port number from that
text. There is one intentional bug:

- the input `0` aborts the process

That makes it easy for `honggfuzz-rs` to find a crash quickly.

## Project layout

```text
src/lib.rs                              # code we want to fuzz
src/bin/parse_port.rs                   # honggfuzz-rs harness
hfuzz_workspace/parse_port/input/seed.txt
```

## Enter the example directory

Run commands from inside `honggfuzz_rs_example/`:

```bash
cd honggfuzz_rs_example
```

## Prerequisites

Install the `honggfuzz` cargo subcommand if you do not already have it:

```bash
cargo install honggfuzz
```

You will need some native build tools, for example, in Ubuntu:

```bash
sudo apt install build-essential binutils-dev libunwind-dev libblocksruntime-dev liblzma-dev lldb
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

```bash
cargo hfuzz run parse_port
```

`cargo hfuzz` builds an instrumented binary and repeatedly calls
`honggfuzz_rs_example::parse_port`. Since this example only needs a one-byte
input to trigger the bug, it usually reaches the crashing input `0` quickly.

## What the generated files mean

- `hfuzz_workspace/parse_port/input/` contains the seed corpus that Honggfuzz
  mutates while fuzzing.
- `hfuzz_workspace/parse_port/input/seed.txt` is the sample input included in
  this tutorial. Its contents are:

```text
8080
```

This gives the fuzzer a valid numeric starting point, and from there it can
mutate the input into nearby values such as `0`.

- `hfuzz_workspace/parse_port/` also stores crash files and other fuzzing
  artifacts produced during the run.
- `hfuzz_target/` contains instrumented build output created by
  `cargo hfuzz`.

## How to see the crashing input

Crash files are saved as raw bytes, usually as `*.fuzz` files under
`hfuzz_workspace/parse_port/`. You can inspect them with:

```bash
xxd hfuzz_workspace/parse_port/<crash-file>.fuzz
cat hfuzz_workspace/parse_port/<crash-file>.fuzz
```

If the crash input is valid UTF-8 text, `cat` may show something readable such
as `0`. Otherwise, `xxd` is the safer way to inspect it.

## Reproduce the case

Once Honggfuzz saves a crash file, you can replay it in a debug run with:

```bash
cargo hfuzz run-debug parse_port hfuzz_workspace/parse_port/*.fuzz
```

`cargo hfuzz run-debug` starts a debugger automatically. By default,
`honggfuzz-rs` uses `lldb`.
