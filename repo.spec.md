# `./`

## Purpose

Repository root for the `chum` CLI. The repo contains the Rust implementation, the thin npm wrapper, workflow/design docs, tests, fixtures, and CI/release scaffolding for the filesystem-first documentation tool.

## Files

- `Cargo.toml` - Rust crate metadata, binary target, runtime dependencies, and test dependencies.
- `Cargo.lock` - Locked Rust dependency graph for reproducible builds.
- `README.md` - Project overview and local development commands.
- `AGENTS.template.md` - Template for repositories that adopt the chum documentation workflow.
- `LICENSE` - MIT license text.

## Subfolders

- `src/` - Rust CLI implementation.
- `npm/` - JavaScript package wrapper that delegates to the native Rust binary.
- `tests/` - Integration tests for CLI behavior.
- `fixtures/` - Small source trees used by tests and smoke checks.
- `design/` - Intent/design documentation for the package.
- `plan/` - Phased implementation plan and validation checklists.
- `reference/` - Initial scoping/reference notes.
- `.github/` - CI and release workflow definitions.

## Dependencies / Contracts

- The Rust crate is the behavioral source of truth; npm packaging must only locate and execute the native binary.
- `target/**`, `archive/**`, `.git/**`, and other generated or historical paths are excluded from source discovery by default.
- `repo.spec.md` is the root live spec and is validated by `chum check`.

<!-- chum:backmatter
schema: 1
kind: directory
target: .
spec_updated_at: 2026-04-24T01:35:55.621326Z
generated_by: chum swim --stubs
children:
- npm/npm.spec.md
- src/src.spec.md
todo: []
unknowns: []
verify: []
-->
