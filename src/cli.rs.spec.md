# `src/cli.rs`

## Purpose

Defines the public command-line shape for the `chum` binary using `clap`
derive structs. The module is intentionally data-only: it names commands,
arguments, flags, defaults, and positional paths, while command modules own all
behavior.

## Key Exports

- `Cli` is the top-level parser and contains one `Command`.
- `Command` enumerates `init`, `check`, `archive`, and `swim`.
- `InitArgs`, `CheckArgs`, `ArchiveArgs`, and `SwimArgs` hold typed arguments
  for their corresponding command modules.
- `SwimArgs.auth_status` enables secret-free OpenAI/Codex auth diagnostics
  without generating specs.

## Dependencies / Contracts

- The module depends on `clap` derive macros and `PathBuf` only.
- Defaults here are user-facing CLI defaults; config defaults live in
  `src/config.rs`.
- `swim --provider` defaults to `openai`, and `swim --auth-status` is consumed
  by the swim command before source discovery or provider generation.

<!-- chum:backmatter
schema: 1
kind: file
target: src/cli.rs
source_hash: sha256:e0c2ed36fcfba6ab6517b254aae54d5a5c1a789d154781ce4609a7ec81c71e69
source_updated_at: 2026-04-24T02:49:39.669922412Z
spec_updated_at: 2026-04-24T02:54:29.274952Z
generated_by: chum swim --stubs
todo: []
unknowns: []
verify: []
-->
