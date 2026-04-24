# `src/commands/init.rs`

## Purpose

Implementation of `chum init`. It creates the standard chum workflow directories and files, optionally appends a workflow snippet to an AGENTS file, and reports the planned or written changes.

## Key Exports

- `run` - Executes initialization from `InitArgs`.

## Dependencies / Contracts

- `--dry-run` prevents all writes.
- Without `--dry-run`, init writes by default even if `--write` is omitted.
- Creates `design/`, `plan/`, `debug/`, `review/`, and `archive/` when missing.
- Creates `archive/README.md` and `chum.config.yaml` only when missing.
- AGENTS snippet insertion is idempotent and guarded by the `## chum Documentation Workflow` heading.

<!-- chum:backmatter
schema: 1
kind: file
target: src/commands/init.rs
source_hash: sha256:d5f67e07c2b9c2cd2aeb23e07373afa239d4e34d3ad772a91d568e10ebc28df6
source_updated_at: 2026-04-24T01:40:19.24457622Z
spec_updated_at: 2026-04-24T01:40:42.596034Z
generated_by: chum swim --stubs
todo: []
unknowns: []
verify: []
-->
