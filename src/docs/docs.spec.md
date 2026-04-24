# `src/docs/`

## Purpose

Markdown parsing and rewriting helpers used by the command layer. This folder centralizes chum-specific document formats so command implementations do not manipulate Markdown and YAML ad hoc.

## Files

- `mod.rs` - Re-exports document helper modules.
- `backmatter.rs` - Parses, renders, replaces, and validates chum backmatter blocks.
- `frontmatter.rs` - Reads leading YAML frontmatter and extracts `change` ids for archive discovery.
- `links.rs` - Rewrites relative Markdown links when docs move into archive entries.

## Subfolders


## Dependencies / Contracts

- Backmatter YAML is the machine-readable state consumed by `check` and `swim`.
- Frontmatter is optional and only used where active change docs provide it.
- Link rewriting is intentionally lightweight and handles normal inline Markdown links, not a full Markdown AST.

<!-- chum:backmatter
schema: 1
kind: directory
target: src/docs
spec_updated_at: 2026-04-24T01:35:55.620774Z
generated_by: chum swim --stubs
children:
- src/docs/backmatter.rs.spec.md
- src/docs/frontmatter.rs.spec.md
- src/docs/links.rs.spec.md
- src/docs/mod.rs.spec.md
todo: []
unknowns: []
verify: []
-->
