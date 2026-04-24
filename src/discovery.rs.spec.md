# `src/discovery.rs`

## Purpose

Ignore-aware filesystem discovery for source files, source directories, live specs, active docs, and archive docs. Commands use this as the shared view of a target tree.

## Key Exports

- `SourceFile` - Source file with repo-relative path, absolute path, and expected inline spec path.
- `SourceDir` - Source directory with repo-relative path, absolute path, and expected directory spec path.
- `Discovery` - Aggregated discovery result including docs, source nodes, and ignored counts.
- `DiscoverOptions` - Per-command switches for explicit source includes and archive inclusion.
- `discover` - Walks the tree, classifies files, and computes source directory/spec targets.

## Dependencies / Contracts

- Uses `ignore::WalkBuilder`, Git ignore settings, and configured custom ignore filenames.
- Live specs are any path ending in `.spec.md`.
- Active docs are Markdown files under configured active dirs.
- Archive docs are Markdown files under the configured archive dir and are ignored unless requested.
- Explicit include globs narrow source discovery to matching paths and can bypass normal include/exclude decisions for selected files.

<!-- chum:backmatter
schema: 1
kind: file
target: src/discovery.rs
source_hash: sha256:c723b0a33483df785db7de489a2b4e06525f017f9c8adbad92f3e80fe0d42db7
source_updated_at: 2026-04-24T02:08:11.838384649Z
spec_updated_at: 2026-04-24T02:08:30.054703Z
generated_by: chum swim --stubs
todo: []
unknowns: []
verify: []
-->
