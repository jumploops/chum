# `src/provider/mod.rs`

## Purpose

Provider abstraction for AI-backed `chum swim`. It defines the input and output contracts between the filesystem traversal engine and concrete generation providers.

## Key Exports

- `openai` - OpenAI provider implementation module.
- `FileSpecInput` - Source file target, source text, and optional existing spec.
- `DirectorySpecInput` - Directory target, child spec text, and optional existing spec.
- `RepairSpecInput` - Current spec plus local context for resolving open items.
- `SpecDraft` - Provider-returned Markdown draft.
- `ChumSwimProvider` - Trait for file generation, directory generation, and spec repair.

## Dependencies / Contracts

- Providers should not write files or perform traversal.
- Core swim code normalizes/validates returned backmatter before writing.
- Trait methods are synchronous in the current implementation.

<!-- chum:backmatter
schema: 1
kind: file
target: src/provider/mod.rs
source_hash: sha256:305a4e001b0b8dfc04953d31546d5e4ce3c2e81589cb622caa55bf2808be4be5
source_updated_at: 2026-04-24T01:35:37.060236943Z
spec_updated_at: 2026-04-24T01:35:55.619838Z
generated_by: chum swim --stubs
todo: []
unknowns: []
verify: []
-->
