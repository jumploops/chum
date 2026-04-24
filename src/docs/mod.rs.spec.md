# `src/docs/mod.rs`

## Purpose

Module declarations for chum document helpers. It exposes the backmatter, frontmatter, and link-rewriting modules to command code.

## Key Exports

- `backmatter` - Chum-owned spec metadata parsing and rendering.
- `frontmatter` - Active change doc frontmatter parsing.
- `links` - Markdown link rewriting during archive moves.

## Dependencies / Contracts

- Command modules should import document helpers through this module rather than declaring submodules themselves.

<!-- chum:backmatter
schema: 1
kind: file
target: src/docs/mod.rs
source_hash: sha256:0840b1533c16f01ad1c08ee037d9ee8c28bc0b0114f3e4b5856d18161658f78d
source_updated_at: 2026-04-24T01:26:45.039908206Z
spec_updated_at: 2026-04-24T01:35:55.619387Z
generated_by: chum swim --stubs
todo: []
unknowns: []
verify: []
-->
