# Phase 1: Workflow Core

## Goal

Implement the shared filesystem, config, markdown, backmatter, source discovery, `chum init`, and `chum check` behavior that all later commands depend on.

## Scope

In scope:

- config defaults and `chum.config.yaml` loading
- ignore-aware traversal using `.gitignore` and `.chumignore`
- source include and exclude matching
- inline spec path matching
- frontmatter and backmatter parsing
- legacy `SPEC:TODO`, `SPEC:UNKNOWN`, and `SPEC:VERIFY` marker detection
- `chum init`
- `chum check`
- human and JSON output contracts

Out of scope:

- archive file movement
- swim spec generation
- OpenAI provider integration
- release packaging

## Implementation Notes

### Config

The config loader should merge three layers:

1. built-in defaults
2. `chum.config.yaml` overrides when present
3. command-line flags

Default config must work when no config file exists.

### Discovery

Build one reusable discovery pipeline:

1. resolve root path
2. load ignore rules
3. walk files
4. classify files as source, live specs, active docs, archive docs, ignored files, or other files
5. compute expected spec paths for source files and source directories

Use the same discovery output for `check`, `archive`, and `swim`.

### Backmatter

Implement parser and writer in a central module. `check` should report:

- missing backmatter
- invalid YAML
- wrong `kind`
- wrong `target`
- stale `source_hash`
- unresolved `todo`, `unknowns`, or `verify`

External verify behavior:

- default: `verify` items fail
- `--allow-external-verify`: verify items do not fail only when they explicitly describe evidence outside the local repository
- TODO and unknown items always fail

### `chum init`

Responsibilities:

- detect existing workflow directories and spec conventions
- create `archive/README.md` when writing
- create `chum.config.yaml` only when requested or when defaults are insufficient
- optionally update an AGENTS file
- support `--dry-run`, `--write`, `--agent-doc`, and `--no-agent-doc`

### `chum check`

Responsibilities:

- validate expected file and directory specs
- ignore `archive/**` unless `--include-archive`
- respect source exclusions
- print a concise human summary
- emit structured JSON with failures, warnings, ignored counts, and discovered paths

## Acceptance Criteria

- [x] Config defaults are tested without a config file.
- [x] `.gitignore` and `.chumignore` behavior is covered by fixtures.
- [x] Tests, fixtures, scripts, generated files, migrations, config files, and `target/**` build output are excluded by default.
- [x] `chum init --dry-run` does not write files.
- [x] `chum init --write` creates expected workflow files.
- [x] `chum check` fails on missing specs.
- [x] `chum check` fails on TODO and unknown items.
- [x] `chum check` fails on verify items by default.
- [x] `chum check --allow-external-verify` allows verify items.
- [x] `chum check --json` is valid JSON.

## Dependencies

- Phase 0 scaffold.
