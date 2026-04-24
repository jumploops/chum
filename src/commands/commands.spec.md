# `src/commands/`

## Purpose

Command implementations and top-level dispatch for the `chum` binary. Each file translates parsed CLI arguments into filesystem operations, reports human or JSON output, and keeps shared behavior in lower-level modules.

## Files

- `mod.rs` - Dispatches parsed `Command` variants and handles `check` exit status.
- `init.rs` - Initializes workflow directories, config, archive README, and optional AGENTS snippets.
- `check.rs` - Validates source/spec coverage, backmatter, hashes, markers, and unresolved work.
- `archive.rs` - Plans and performs Markdown-only archive moves for completed change docs.
- `swim.rs` - Generates/repairs inline specs through stubs or provider-backed generation.

## Subfolders


## Dependencies / Contracts

- Mutating commands must respect `--dry-run`.
- JSON output is part of the CLI contract for automation.
- `archive` must never move live `*.spec.md` files.
- `swim --stubs` must work without network or provider credentials.

<!-- chum:backmatter
schema: 1
kind: directory
target: src/commands
spec_updated_at: 2026-04-24T01:35:55.620623Z
generated_by: chum swim --stubs
children:
- src/commands/archive.rs.spec.md
- src/commands/check.rs.spec.md
- src/commands/init.rs.spec.md
- src/commands/mod.rs.spec.md
- src/commands/swim.rs.spec.md
todo: []
unknowns: []
verify: []
-->
