# `src/docs/links.rs`

## Purpose

Relative Markdown link rewriting for archive moves. It adjusts links in moved Markdown files so references to moved docs and live repo files remain usable from the archived location.

## Key Exports

- `LinkRewrite` - Rewritten content plus warning messages.
- `rewrite_markdown_links` - Scans inline Markdown links and rewrites local relative targets based on an old-to-new move map.

## Dependencies / Contracts

- Skips empty, anchor-only, HTTP(S), and mailto targets.
- Preserves fragment suffixes such as `#section`.
- Links to files moved in the same archive operation are rewritten to their new archive paths.
- Existing local assets that are not moved are left in place and produce warnings.
- This is a lightweight scanner for normal inline links, not a complete Markdown parser.

<!-- chum:backmatter
schema: 1
kind: file
target: src/docs/links.rs
source_hash: sha256:8a443796f7cf31271f12e918aae2904226f3829d91519b87ab51c6bbc13b30fc
source_updated_at: 2026-04-24T01:32:14.347753496Z
spec_updated_at: 2026-04-24T01:35:55.619226Z
generated_by: chum swim --stubs
todo: []
unknowns: []
verify: []
-->
