# `src/provider/mod.rs`

## Purpose

Defines the provider module surface for AI-backed `chum swim`. The module
exports provider implementations and the common request/response types that keep
the swim traversal independent from any specific model transport.

## Key Exports

- `codex`, `openai`, `openai_api`, and `openai_auth` provider submodules.
- `FileSpecInput`, `DirectorySpecInput`, and `RepairSpecInput` describe bounded
  provider requests.
- `SpecDraft` carries generated Markdown back to the swim command.
- `ChumSwimProvider` is the synchronous generation/repair trait implemented by
  Codex exec and direct OpenAI API providers.

## Dependencies / Contracts

- Provider inputs are serializable for tests and future diagnostics, but they
  should not be logged wholesale because they may contain source text.
- Providers return Markdown only; `src/commands/swim.rs` owns target-specific
  backmatter normalization and filesystem writes.
- The trait is synchronous to match the current CLI architecture.

<!-- chum:backmatter
schema: 1
kind: file
target: src/provider/mod.rs
source_hash: sha256:c4ea3800c11211fd2136e1081a3c2ddcb1961e1b565ebd12666e8ab1492d138b
source_updated_at: 2026-04-24T02:47:50.186580253Z
spec_updated_at: 2026-04-24T02:54:29.284213Z
generated_by: chum swim --stubs
todo: []
unknowns: []
verify: []
-->
