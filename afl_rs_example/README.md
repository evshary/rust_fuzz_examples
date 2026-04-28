# afl.rs tutorial example

This example lives in the `afl_rs_example/` folder. It contains a very small
Rust crate plus an AFL harness built with `afl.rs`.

The function under test is `parse_port` in `src/lib.rs`. It accepts bytes,
tries to interpret them as UTF-8 text, and parses a port number from that
text. There is one intentional bug:

- the input `0` aborts the process

That makes it easy for AFL to find a crash quickly.

## Project layout

```text
src/lib.rs                  # code we want to fuzz
src/bin/fuzz_parse_port.rs  # afl.rs harness
in/seed.txt                 # initial seed input
```

## Enter the example directory

Run commands from inside `afl_rs_example/`:

```bash
cd afl_rs_example
```

## Prerequisites

Install `cargo-afl` if you do not already have it:

```bash
cargo install cargo-afl
```

Depending on your environment, you may also need AFL++ tooling installed. If
`cargo afl` tells you something is missing, install AFL++ first and rerun the
commands below.

## Run the normal program

```bash
cargo run
```

Expected output:

```text
Parsed port: 8080
```

## Build the AFL harness

```bash
cargo afl build --features afl-harness --bin fuzz_parse_port
```

## Run AFL

```bash
cargo afl fuzz -i in -o out target/debug/fuzz_parse_port
```

`cargo-afl` will mutate the files from `in/` and repeatedly call
`afl_rs_example::parse_port`. Since the seed corpus already contains a simple
numeric input, AFL usually reaches the crashing input `0` quickly.

`cargo build` and `cargo run` work without AFL because the fuzzing harness is
behind the optional `afl-harness` feature and is only built when requested.

## Troubleshooting

If AFL stops with a `core_pattern` error like:

```text
PROGRAM ABORT : Pipe at the beginning of 'core_pattern'
```

your Linux system is forwarding crashes to an external crash handler. `cargo-afl`
documents a helper command for this setup step:

```bash
cargo afl system-config
```

This usually requires `sudo` and adjusts the system so AFL can detect crashes
reliably.

## What the generated files mean

- `in/` contains the initial seed inputs. These are the starting files AFL uses
  before it begins mutating inputs.
- `in/seed.txt` is the one sample input included in this tutorial. Its contents
  are:

```text
8080
```

This gives AFL a valid numeric starting point. From there, AFL mutates the
input into nearby values, and it can often reach the crashing input `0` quickly.

- `out/` contains AFL's working directory, including queue files, statistics,
  and crashes that it discovers.
- `out/default/crashes/` contains inputs that triggered a crash.

## How to see the crashing input

The saved files are raw bytes. A few easy ways to inspect them are:

```bash
xxd out/default/crashes/<crash-file>
cat out/default/crashes/<crash-file>
```

If the crash input is valid UTF-8 text, `cat` may show something readable, such
as `0`. Otherwise, `xxd` is the safer way to inspect it.

## Reproduce the case

You can replay a saved input by piping it into the AFL harness directly:

```bash
target/debug/fuzz_parse_port < out/default/crashes/<crash-file>
```
