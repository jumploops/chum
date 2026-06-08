# Phase 1: Python Core

## Goal

Port the deterministic repository model from Rust into `scripts/chum.py`.

This phase should not implement the full agent command surface yet. It should
build the internal primitives that later commands use.

## Scope

In scope:

- config defaults and `chum.config.yaml` loading
- filesystem root normalization
- ignore handling for `.gitignore` and `.chumignore`
- source discovery
- source classification and default exclusions
- inline spec path matching
- source hashing and timestamp collection
- frontmatter parsing for active docs
- chum backmatter parsing/rendering
- unresolved marker detection
- archive directory exclusion
- unit tests for primitives

Out of scope:

- skill instructions
- archive file movement
- full repo check output
- Rust cleanup

## Implementation Notes

Keep `scripts/chum.py` self-contained. Avoid creating a Python package or
requiring a project install.

Recommended internal sections:

```text
models
config loading
path utilities
ignore handling
source discovery
spec path matching
frontmatter parser
backmatter parser/writer
validation primitives
command dispatch
```

If the single script grows too large, split support files under `scripts/lib/`
only after the single-file version becomes hard to maintain. Prefer one obvious
runtime command for agents.

## Config Defaults

Match the existing workflow defaults unless this plan explicitly changes them:

- active change dirs: `design`, `plan`, `debug`, `review`
- archive dir: `archive`
- live spec glob: `**/*.spec.md`
- inline spec placement only
- ignore `archive/**`
- respect `.gitignore`
- respect optional `.chumignore`
- exclude tests, fixtures, scripts, generated files, migrations, config,
  Markdown, plaintext, media, binaries, lockfiles, and build output by default
- allow explicit includes to override default exclusions

## Backmatter Contract

Parse and render exactly one block:

```markdown
<!-- chum:backmatter
schema: 1
kind: file
target: src/foo.py
source_hash: sha256:...
source_updated_at: 2026-04-24T12:00:00Z
spec_updated_at: 2026-04-24T12:03:00Z
generated_by: chum skill
todo: []
unknowns: []
verify: []
-->
```

Requirements:

- invalid YAML reports file and line context when possible
- missing backmatter is distinguishable from invalid backmatter
- unknown fields are preserved when practical
- `todo`, `unknowns`, and `verify` normalize to lists
- source hashes use SHA-256 with `sha256:` prefix
- generated backmatter uses `generated_by: chum skill`

## Source Discovery

Use the script's standard-library ignore matcher for v1. It must cover the
documented defaults, root `.gitignore`, and root `.chumignore` behavior without
requiring dependency installation.

Discovery should produce a stable sorted model:

- source files
- source directories that require directory specs
- expected spec paths
- ignored path counts
- warnings for unreadable files

Directory specs should be included when a directory contains source files or
source subdirectories after exclusions.

## Tests

Port fixture coverage from the Rust tests where practical:

- plain non-Git directory
- Git repo with `.gitignore`
- directory with `.chumignore`
- missing file specs
- missing directory specs
- stale source hash
- TODO, unknowns, and verify backmatter
- excluded tests, fixtures, scripts, generated files, migrations, and config

## Acceptance Criteria

- [x] Config defaults load without a config file.
- [x] `chum.config.yaml` overrides defaults.
- [x] `.gitignore` and `.chumignore` are respected.
- [x] Source discovery output is deterministic.
- [x] Inline spec paths match the Rust behavior.
- [x] Backmatter parser and writer round-trip valid specs.
- [x] Invalid backmatter failures include useful context.
- [x] Unit tests cover core primitives.

## Dependencies

- Phase 0 skill scaffold.
