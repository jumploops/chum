# `src/main.rs`

## Purpose

Binary entrypoint for `chum`. It wires together the crate modules, parses CLI arguments with `clap`, and hands execution to the command dispatcher.

## Key Exports

- `main` - Parses `cli::Cli` and returns the `anyhow::Result` from `commands::run`.

## Dependencies / Contracts

- The top-level process exit behavior for most commands is delegated to `commands::run`.
- Any new top-level module must be declared here to participate in the binary build.

<!-- chum:backmatter
schema: 1
kind: file
target: src/main.rs
source_hash: sha256:ecb5a6e7d4c0c7a76624c324702b4c8543216fa1edead1b608b306d6255ed04d
source_updated_at: 2026-04-24T01:25:16.984908341Z
spec_updated_at: 2026-04-24T01:35:55.619528Z
generated_by: chum swim --stubs
todo: []
unknowns: []
verify: []
-->
