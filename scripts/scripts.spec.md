# `scripts/`

## Purpose

Deterministic processor used by the `chum` skill. The script handles repository
mechanics that should not depend on model judgment.

## Files

- `chum.py` - Standard-library Python CLI for target discovery, whole-repo
  checks, spec normalization, focused validation, workflow initialization, and
  archive movement.

## Dependencies / Contracts

- `scripts/chum.py` is non-interactive and safe to run through
  `uv run scripts/chum.py ...`.
- The script does not call an LLM or `codex exec`.
- JSON output is written to stdout; diagnostics and warnings go to stderr.

<!-- chum:backmatter
schema: 1
kind: directory
target: scripts
spec_updated_at: 2026-06-08T21:28:10Z
generated_by: chum skill
children:
- scripts/chum.py.spec.md
todo: []
unknowns: []
verify: []
-->
