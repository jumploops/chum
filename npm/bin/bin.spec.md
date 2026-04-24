# `npm/bin/`

## Purpose

Executable entrypoint directory for the npm package. Files here are exposed through the npm `bin` field and should remain thin process wrappers.

## Files

- `chum.js` - Resolves the platform-specific native binary and forwards CLI arguments/stdio to it.

## Subfolders


## Dependencies / Contracts

- Must preserve argument order and inherited stdio behavior.
- Must exit with the native binary's status code.
- Must not reimplement Rust command behavior.

<!-- chum:backmatter
schema: 1
kind: directory
target: npm/bin
spec_updated_at: 2026-04-24T01:35:55.620481Z
generated_by: chum swim --stubs
children:
- npm/bin/chum.js.spec.md
todo: []
unknowns: []
verify: []
-->
