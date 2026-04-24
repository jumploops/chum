# `src/commands/archive.rs`

## Purpose

Implementation of `chum archive <change-id>`. It discovers completed active Markdown docs, plans or performs a Markdown-only move into `archive/<change-id>/`, rewrites links, writes an archive manifest, and warns without blocking when `chum check` fails.

## Key Exports

- `run` - Executes archive dry-runs and real moves from `ArchiveArgs`.

## Dependencies / Contracts

- Discovery priority is frontmatter `change`, then change-id folder, then filename, plus explicit `--include` globs.
- Automatic folder and filename matches together are treated as ambiguous and fail closed.
- `*.spec.md` files are never archived.
- Only Markdown active docs move; linked assets remain in place and produce warnings.
- A failed pre-archive `chum check` is recorded as a warning and does not block archive.
- Archive manifests are written to `archive/<change-id>/README.md`.

<!-- chum:backmatter
schema: 1
kind: file
target: src/commands/archive.rs
source_hash: sha256:5d74a24dad544296521af346d0c151e9eadb14ccf7341868ed8402b550786f30
source_updated_at: 2026-04-24T02:09:29.585854827Z
spec_updated_at: 2026-04-24T02:10:09.456652Z
generated_by: chum swim --stubs
todo: []
unknowns: []
verify: []
-->
