# bolero tutorial example

This example shows a minimal `bolero` setup. `Bolero` sits between fuzzing and
property testing: you write a property-style test, then run it with the
`cargo-bolero` subcommand.

The target function parses a port number and intentionally panics on input `0`.

## Files

```text
src/lib.rs
src/main.rs
tests/parse_port_matches_std.rs
```

## Setup

```bash
cd bolero_example
cargo install cargo-bolero
rustup toolchain install nightly
```

On Debian or Ubuntu, the official `bolero` docs also mention:

```bash
sudo apt install binutils-dev libunwind-dev
```

## Run

```bash
cargo run
cargo test
cargo +nightly bolero test parse_port_matches_std
```

Expected normal output:

```text
Parsed port: 8080
```

## Saved inputs

If `bolero` finds a failure, it usually stores interesting cases under
`corpus/` or `crashes/`.

Inspect a saved file with:

```bash
xxd crashes/<file>
cat crashes/<file>
```

## Replay

```bash
cargo +nightly bolero test parse_port_matches_std
```

After a failure is saved, Bolero can replay it on later runs.
