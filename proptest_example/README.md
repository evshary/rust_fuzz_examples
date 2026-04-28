# proptest tutorial example

This example shows a minimal `proptest` setup. Unlike the other examples in
this repository, this is property-based testing rather than fuzz testing.

The target function is `parse_port` in `src/lib.rs`. It parses a string as a
port number and intentionally panics on input `0`.

## Files

```text
src/lib.rs
src/main.rs
```

## Setup

```bash
cd proptest_example
```

## Run

```bash
cargo run
cargo test
cargo test parse_port_matches_std -- --ignored
```

Expected normal output:

```text
Parsed port: 8080
```

`cargo test` passes because the property test is marked `#[ignore]`. Run the
ignored test explicitly when you want Proptest to generate inputs from the
strategy and shrink the bug to a minimal case.

## Saved inputs

If Proptest finds a failure, it writes a minimized regression case under
`proptest-regressions/`. In this example, the file is typically:

```text
proptest-regressions/lib.txt
```

Inspect a saved file with:

```bash
cat proptest-regressions/lib.txt
```

## Replay

```bash
cargo test parse_port_matches_std -- --ignored
```

After a failure is saved, Proptest replays the cases from
`proptest-regressions/lib.txt` first on later test runs.
