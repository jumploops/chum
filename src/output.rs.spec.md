# `src/output.rs`

## Purpose

Shared output helper for machine-readable command responses.

## Key Exports

- `print_json` - Pretty-prints any serializable value to stdout as JSON.

## Dependencies / Contracts

- Uses `serde_json::to_string_pretty`; callers are responsible for stable response structs.
- Prints to stdout and returns serialization errors through `anyhow::Result`.

<!-- chum:backmatter
schema: 1
kind: file
target: src/output.rs
source_hash: sha256:f1c3e8c0a062f176fc17bf475cb4dfe54a7a66916c78a03c56bc1c5b97724660
source_updated_at: 2026-04-24T01:25:17.025412225Z
spec_updated_at: 2026-04-24T01:35:55.619669Z
generated_by: chum swim --stubs
todo: []
unknowns: []
verify: []
-->
