# Phase 0: Scaffold

## Goal

Create the project skeleton needed to implement and test the Rust CLI without committing to the full command behavior yet.

## Scope

In scope:

- Rust crate named `chum`
- initial `clap` command structure
- module layout
- fixture directory for command tests
- npm wrapper package skeleton under `npm/`
- basic CI placeholders for test and release work
- repository live specs for the created source folders

Out of scope:

- full config semantics
- archive movement
- swim generation
- actual release publishing

## Implementation Notes

Create the Rust crate with a single binary target:

```text
Cargo.toml
src/main.rs
src/cli.rs
src/commands/
```

Initial commands should parse and return a clear "not implemented" error where behavior is not ready. This lets later phases wire functionality without changing the public CLI shape.

Recommended dependencies:

- `clap`
- `anyhow`
- `thiserror`
- `serde`
- `serde_yaml`
- `serde_json`
- `ignore`
- `globset`
- `sha2`
- `camino`
- `time`

Test dependencies:

- `assert_cmd`
- `predicates`
- `tempfile`
- `insta` if snapshots are useful

The npm wrapper should establish the intended interface only:

```text
npm/package.json
npm/bin/chum.js
npm/scripts/resolve-binary.js
```

The wrapper may fail with a clear message until Phase 5 wires real release artifacts.

## Acceptance Criteria

- [x] `cargo build` succeeds.
- [x] `cargo test` succeeds with at least one command parsing test.
- [x] `cargo run -- --help` lists `init`, `check`, `archive`, and `swim`.
- [x] `cargo run -- archive --help` shows `<change-id>`.
- [x] npm wrapper files exist and document that they delegate to the native binary.
- [x] New source folders have matching `*.spec.md` files.

## Dependencies

- None.
