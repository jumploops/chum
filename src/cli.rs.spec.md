# `src/cli.rs`

## Purpose

Public command-line shape for the `chum` binary. This file defines all command names, positional arguments, flags, defaults, and the typed argument structs consumed by command implementations.

## Key Exports

- `Cli` - Root `clap::Parser` containing the selected subcommand.
- `Command` - Subcommand enum for `init`, `check`, `archive`, and `swim`.
- `InitArgs` - Flags for workflow initialization and optional AGENTS updates.
- `CheckArgs` - Validation flags including JSON output, archive inclusion, stale allowance, and external verify handling.
- `ArchiveArgs` - Change id, include globs, optional PR/source metadata, dry-run, JSON, and target path.
- `SwimArgs` - Target path and generation controls for stub/provider-backed spec creation.

## Dependencies / Contracts

- Command names are user-facing API and should stay aligned with `AGENTS.template.md`, design docs, and plan docs.
- Default target path for commands that operate on a tree is `.`.
- `archive` takes `<change-id>` directly; there is no `archive-change` command.

<!-- chum:backmatter
schema: 1
kind: file
target: src/cli.rs
source_hash: sha256:6277a1609eaa53bd515a9c36067f66ffaa95355a8a252669856f58983eb68184
source_updated_at: 2026-04-24T01:25:17.000182881Z
spec_updated_at: 2026-04-24T01:35:55.615828Z
generated_by: chum swim --stubs
todo: []
unknowns: []
verify: []
-->
