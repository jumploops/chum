# `src/docs/frontmatter.rs`

## Purpose

Minimal YAML frontmatter reader for active change docs. Archive discovery uses it to prefer explicit `change: <id>` metadata over folder or filename heuristics.

## Key Exports

- `parse` - Parses leading `---` frontmatter into a YAML value when present.
- `change_id` - Extracts a string `change` value from frontmatter.

## Dependencies / Contracts

- Only frontmatter at the very start of the file is recognized.
- Missing or unterminated frontmatter is treated as absent, not as a hard error.
- Invalid YAML inside recognized frontmatter is an error.

<!-- chum:backmatter
schema: 1
kind: file
target: src/docs/frontmatter.rs
source_hash: sha256:2ec4aba7ff1b73d7944f5fa5a8c32bdd166417a74fb9bbfea4e3c6c174f693c8
source_updated_at: 2026-04-24T01:26:45.073692471Z
spec_updated_at: 2026-04-24T01:35:55.61897Z
generated_by: chum swim --stubs
todo: []
unknowns: []
verify: []
-->
