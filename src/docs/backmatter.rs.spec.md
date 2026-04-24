# `src/docs/backmatter.rs`

## Purpose

Parser and renderer for the machine-readable chum backmatter block stored at the end of chum-owned specs. This module defines the schema used by `check` and `swim`.

## Key Exports

- `SpecKind` - Backmatter target kind, either `file` or `directory`.
- `Backmatter` - Serializable schema with target, hashes, timestamps, children, and open-work lists.
- `ParsedBackmatter` - Parsed schema plus byte range of the original block.
- `parse_file` - Reads and parses backmatter from a file.
- `parse` - Finds exactly one backmatter block in a string and parses the YAML body.
- `replace_or_append` - Replaces an existing block or appends a new one.
- `render` - Converts `Backmatter` into the HTML-comment fenced YAML block.

## Dependencies / Contracts

- Backmatter starts with the chum HTML-comment fence and ends at the next HTML comment close.
- Multiple backmatter blocks are invalid.
- YAML parsing is handled by `serde_yaml`.
- Rewriting trims the old block location and appends the rendered replacement with a blank line separator.

<!-- chum:backmatter
schema: 1
kind: file
target: src/docs/backmatter.rs
source_hash: sha256:9aab66c675365eb52626d818e0507223303a42c9a8dbb10c5ae8a52189a36e84
source_updated_at: 2026-04-24T01:32:14.347331744Z
spec_updated_at: 2026-04-24T01:35:55.618786Z
generated_by: chum swim --stubs
todo: []
unknowns: []
verify: []
-->
