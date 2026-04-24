# `npm/bin/chum.js`

## Purpose

Node executable shim for the npm package. It resolves the bundled native binary, forwards all CLI arguments and stdio, and exits with the native process status.

## Key Exports

- Top-level script body invoked through the npm `bin` mapping.

## Dependencies / Contracts

- Requires `../scripts/resolve-binary`.
- Uses `spawnSync` with inherited stdio so command output behaves like the native binary.
- On execution errors, prints a reinstall or `cargo install chum` fallback.
- Must remain behavior-free beyond binary resolution and process forwarding.

<!-- chum:backmatter
schema: 1
kind: file
target: npm/bin/chum.js
source_hash: sha256:4a618442b47efdc94a9d26521967546a8b0d44d740519f35023fbf961a962f15
source_updated_at: 2026-04-24T01:31:48.832713985Z
spec_updated_at: 2026-04-24T01:35:55.614907Z
generated_by: chum swim --stubs
todo: []
unknowns: []
verify: []
-->
