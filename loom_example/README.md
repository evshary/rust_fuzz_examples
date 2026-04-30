# loom tutorial example

This example shows a minimal `loom` setup. `Loom` is not a fuzzer. It is a
concurrency testing tool that explores different thread interleavings for a
small concurrent program.

The example models a tiny "publish then observe" pattern using atomics.

## Files

```text
src/lib.rs
src/main.rs
tests/loom_publish.rs
```

## Setup

```bash
cd loom_example
```

## Run

```bash
cargo run
cargo test
RUSTFLAGS="--cfg loom" cargo test --test loom_publish --release
```

Expected normal output:

```text
Published value: 1
```

## What it demonstrates

- a normal implementation in `src/lib.rs`
- a Loom-specific concurrency model in `tests/loom_publish.rs`
- how `loom::model` explores many thread schedules for the same test

## Key idea

Unlike fuzzing tools, Loom does not mutate inputs. Instead, it explores
possible thread schedules to catch concurrency bugs that may only happen under
rare interleavings.
