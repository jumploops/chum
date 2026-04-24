# `src/spec.rs`

## Purpose

Spec-path and source-freshness utilities. This module encodes inline spec placement rules, source hashing, timestamp formatting, and standard backmatter construction.

## Key Exports

- `file_spec_path` - Maps a source file path to `<path>.spec.md` under the root.
- `dir_spec_path` - Maps a source directory to `<dir>/<basename>.spec.md`, with root using `repo.spec.md`.
- `path_to_slash` - Normalizes UTF-8 paths to slash-separated targets for backmatter.
- `sha256_file` - Computes `sha256:<hex>` for source freshness checks.
- `modified_time` - Converts filesystem modified time to RFC3339 when available.
- `now` - Current UTC timestamp in RFC3339 format.
- `file_backmatter` - Creates file backmatter with hash and timestamps.
- `directory_backmatter` - Creates directory backmatter with child spec references.

## Dependencies / Contracts

- Inline placement is the only v1 placement mode.
- File backmatter hashes the actual source file immediately.
- Directory backmatter has no source hash because directories are validated through child specs and open-work lists.

<!-- chum:backmatter
schema: 1
kind: file
target: src/spec.rs
source_hash: sha256:411deaa4eacc64a9b143c9497ef0b004b1217b2b34e95936d6745a815f7daeab
source_updated_at: 2026-04-24T01:32:14.348653709Z
spec_updated_at: 2026-04-24T01:35:55.620332Z
generated_by: chum swim --stubs
todo: []
unknowns: []
verify: []
-->
