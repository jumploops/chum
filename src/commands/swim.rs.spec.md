# `src/commands/swim.rs`

## Purpose

Implementation of `chum swim`. It walks source trees, creates or repairs inline specs either through deterministic stubs or an AI provider, tracks unresolved items, and reports created/updated/skipped work.

## Key Exports

- `run` - Executes stub or provider-backed swim from `SwimArgs`.

## Dependencies / Contracts

- `--stubs` is offline and intentionally creates unresolved TODO/unknown entries for incomplete docs.
- Provider-backed swim currently supports `--provider openai`.
- Existing complete file specs are skipped when their source hash matches and open lists are empty.
- Existing complete directory specs are skipped when open lists are empty.
- Directory specs are processed deepest-first after file specs so child specs are available.
- Provider repair passes run up to `swim.maxPasses` when writing.
- Non-stub swim fails if unresolved TODO/unknown/verify items remain, except verify items allowed by config/flag.

<!-- chum:backmatter
schema: 1
kind: file
target: src/commands/swim.rs
source_hash: sha256:7a62425e567d9cec8e9177602192ab93874b7e93863004c34afe1a634a6ea177
source_updated_at: 2026-04-24T02:07:43.882832326Z
spec_updated_at: 2026-04-24T02:08:30.053242Z
generated_by: chum swim --stubs
todo: []
unknowns: []
verify: []
-->
