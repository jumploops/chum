# `src/config.rs`

## Purpose

Configuration model and default values for `chum`. It provides the built-in zero-config behavior, optional `chum.config.yaml` loading, and path normalization used before discovery and command execution.

## Key Exports

- `Config` - Full resolved config for active dirs, archive dir, spec patterns, source filters, markers, and swim settings.
- `SourceConfig`, `SpecConfig`, `MarkerConfig`, `SwimConfig` - Structured sections serialized with camelCase where needed.
- `Config::default` - Canonical v1 defaults, including source extension globs and default exclusions.
- `Config::load` - Reads `chum.config.yaml` when present and merges partial overrides into defaults.
- `Config::default_yaml` - Emits the default config for `chum init`.
- `normalize_root` - Resolves a filesystem path into a UTF-8 root path.

## Dependencies / Contracts

- Defaults must keep `chum check` useful without requiring a config file.
- `.gitignore` and `.chumignore` are both configured as ignore files by default.
- `target/**`, generated output, archive history, tests, fixtures, scripts, migrations, and config files are excluded from source discovery by default.
- Partial config loading replaces whole vector fields when supplied.

<!-- chum:backmatter
schema: 1
kind: file
target: src/config.rs
source_hash: sha256:982c8708678737ac5c05194d91dd1c956a1ff7c80ee8de52890148b293867d0f
source_updated_at: 2026-04-24T01:33:20.803723268Z
spec_updated_at: 2026-04-24T01:35:55.618219Z
generated_by: chum swim --stubs
todo: []
unknowns: []
verify: []
-->
