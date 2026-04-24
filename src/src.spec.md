# `src/`

## Purpose

Rust source for the `chum` binary. This folder owns CLI parsing, command dispatch, config/default handling, source discovery, spec/backmatter utilities, JSON output, and the provider boundary used by `chum swim`.

## Files

- `main.rs` - Parses top-level CLI arguments and dispatches through `commands::run`.
- `cli.rs` - Defines the public command-line interface and flags using `clap`.
- `config.rs` - Provides built-in defaults and optional `chum.config.yaml` merging.
- `discovery.rs` - Walks source trees, applies ignore rules/globs, and classifies docs/source files.
- `output.rs` - Shared JSON pretty-printer for machine-readable command output.
- `spec.rs` - Computes spec paths, hashes source files, and constructs standard backmatter.

## Subfolders

- `commands/` - Implementations for `init`, `check`, `archive`, and `swim`.
- `docs/` - Markdown frontmatter, chum backmatter, and link-rewriting utilities.
- `provider/` - Provider trait and OpenAI adapter for AI-backed swim.

## Dependencies / Contracts

- Uses `anyhow` for command-level error propagation.
- Uses `camino::Utf8PathBuf` internally for repo-relative path reporting and JSON output.
- Keeps filesystem traversal and provider calls separated so non-AI commands remain deterministic and offline.

<!-- chum:backmatter
schema: 1
kind: directory
target: src
spec_updated_at: 2026-04-24T01:35:55.621166Z
generated_by: chum swim --stubs
children:
- src/cli.rs.spec.md
- src/commands/commands.spec.md
- src/config.rs.spec.md
- src/discovery.rs.spec.md
- src/docs/docs.spec.md
- src/main.rs.spec.md
- src/output.rs.spec.md
- src/provider/provider.spec.md
- src/spec.rs.spec.md
todo: []
unknowns: []
verify: []
-->
