# Phase 5: Packaging

## Goal

Ship `chum` through Cargo and npm/pnpm with native binaries for macOS and Linux.

## Scope

In scope:

- Cargo package metadata
- release build workflow
- macOS arm64 binary
- macOS x64 binary
- Linux arm64 binary
- Linux x64 binary
- npm package `@magicloops/chum`
- npm wrapper that invokes the native binary
- install and smoke tests for Cargo, npm, and pnpm

Out of scope:

- Homebrew formula or tap
- Windows binaries
- signed installers
- package-manager analytics

## Implementation Notes

### Cargo

Prepare crate metadata:

- name: `chum`
- binary: `chum`
- license
- repository
- README
- include/exclude package settings

`cargo install chum` should install a working binary after publish.

### Native Releases

Release artifacts:

```text
chum-aarch64-apple-darwin.tar.gz
chum-x86_64-apple-darwin.tar.gz
chum-aarch64-unknown-linux-gnu.tar.gz
chum-x86_64-unknown-linux-gnu.tar.gz
```

Each archive should include:

- `chum` binary
- license
- checksum

### npm

Publish `@magicloops/chum` as a thin wrapper. Preferred approach:

- main package exposes `bin/chum.js`
- install script resolves the correct release artifact
- package caches or places the binary inside the package directory
- runtime wrapper execs the native binary

The wrapper must not implement command behavior.

Failure behavior:

- unsupported platform: explain supported platforms
- missing binary: explain reinstall or `cargo install chum`
- download failure: explain network issue and fallback

### Smoke Tests

Run smoke tests against built artifacts:

- `chum --help`
- `chum init --dry-run`
- `chum check --json` in a fixture repo
- `pnpm exec chum --help`
- `npx @magicloops/chum --help`

## Acceptance Criteria

- [x] Cargo package metadata is complete.
- [x] Release workflow is configured for all four v1 targets.
- [x] Release workflow is configured to produce checksums for release artifacts.
- [x] npm wrapper selects the correct platform binary.
- [x] npm wrapper produces clear unsupported-platform errors.
- [ ] `npm install -D @magicloops/chum` can run `chum --help`.
- [ ] `pnpm add -D @magicloops/chum` can run `pnpm exec chum --help`.
- [x] Packaging docs exclude Homebrew from v1.

## Dependencies

- Phases 0 through 4.
