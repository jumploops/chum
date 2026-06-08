# `scripts/chum.py`

## Purpose

Standard-library Python processor for the `chum` skill. It provides the
deterministic filesystem mechanics that the agent-led workflow relies on:
target discovery, spec validation, backmatter normalization, workflow
initialization, and archive movement.

## Command Surface

- `targets` reports missing, stale, invalid, and incomplete specs.
- `check` validates all discovered source files and source directories.
- `normalize` inserts or updates chum backmatter for provided Markdown.
- `validate` checks one target/spec pair.
- `init` creates workflow scaffolding and optional agent guidance.
- `archive` moves completed active Markdown docs into archive history.

## Dependencies / Contracts

- The script must remain self-contained and non-interactive.
- It must not call an LLM or run `codex exec`.
- JSON output goes to stdout; diagnostics and warnings go to stderr.
- Backmatter parsing is intentionally small and supports only the YAML subset
  implemented in this file.

<!-- chum:backmatter
schema: 1
kind: file
target: scripts/chum.py
source_hash: sha256:21e3a403cc8c09f62ff1315ea00a2cf1924972b5cadeff9092a960f632ebc52b
source_updated_at: 2026-04-28T20:49:34Z
spec_updated_at: 2026-06-08T21:28:10Z
generated_by: chum skill
todo: []
unknowns: []
verify: []
-->
