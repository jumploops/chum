# `npm/`

## Purpose

Thin npm package for installing and invoking the native `chum` Rust binary from JavaScript package managers. It exists for distribution convenience and must not duplicate CLI behavior.

## Files

- `package.json` - npm metadata, `chum` bin mapping, postinstall hook, and published file list.
- `scripts/install.js` - Postinstall check for supported platforms and bundled binary placement.
- `scripts/resolve-binary.js` - Maps Node platform/architecture to the expected release target and binary path.

## Subfolders

- `bin/` - Executable Node shim exposed as the package binary.

## Dependencies / Contracts

- Runtime command behavior lives in Rust; the Node wrapper only resolves and executes the binary.
- Supported v1 targets are macOS and Linux on arm64/x64.
- Missing binaries should produce a clear fallback message pointing to `cargo install chum`.

<!-- chum:backmatter
schema: 1
kind: directory
target: npm
spec_updated_at: 2026-04-24T01:35:55.621043Z
generated_by: chum swim --stubs
children:
- npm/bin/bin.spec.md
todo: []
unknowns: []
verify: []
-->
