# `src/commands/mod.rs`

## Purpose

Command module registry and dispatcher. It connects parsed CLI subcommands to their implementations and centralizes the special exit behavior for `chum check`.

## Key Exports

- `archive`, `check`, `init`, `swim` modules - Public command implementation modules.
- `run` - Matches `cli::Command` variants and invokes the appropriate command.

## Dependencies / Contracts

- `check` is the only command whose dispatcher branch explicitly exits with status 1 when validation failures exist.
- JSON-vs-human check output is selected here using the original `CheckArgs`.
- Other command implementations own their own output and error behavior.

<!-- chum:backmatter
schema: 1
kind: file
target: src/commands/mod.rs
source_hash: sha256:b5ca8ca1495baaa802d5dede5ddbeb3ce54e69d35974b98ef70f710b14d9135c
source_updated_at: 2026-04-24T01:29:57.434197467Z
spec_updated_at: 2026-04-24T01:35:55.617021Z
generated_by: chum swim --stubs
todo: []
unknowns: []
verify: []
-->
