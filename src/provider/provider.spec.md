# `src/provider/`

## Purpose

Provider boundary for AI-backed `chum swim`. The core swim traversal owns filesystem state and validation; providers only turn bounded inputs into Markdown spec drafts.

## Files

- `mod.rs` - Defines provider input/output structs and the `ChumSwimProvider` trait.
- `openai.rs` - Implements the provider trait with the OpenAI Responses API.

## Subfolders


## Dependencies / Contracts

- Provider implementations must return Markdown; core swim code is responsible for adding or normalizing backmatter before writing.
- The OpenAI adapter reads credentials from `CODEX_OPENAI_API_KEY` or `OPENAI_API_KEY`.
- Additional providers should implement the trait without changing traversal or validation logic.

<!-- chum:backmatter
schema: 1
kind: directory
target: src/provider
spec_updated_at: 2026-04-24T01:35:55.620918Z
generated_by: chum swim --stubs
children:
- src/provider/mod.rs.spec.md
- src/provider/openai.rs.spec.md
todo: []
unknowns: []
verify: []
-->
