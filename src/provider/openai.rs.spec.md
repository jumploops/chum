# `src/provider/openai.rs`

## Purpose

OpenAI-backed implementation of the swim provider trait. It calls the Responses API to generate or repair current-state Markdown specs from bounded local context.

## Key Exports

- `OpenAiProvider` - Holds API key, model name, and blocking reqwest client.
- `OpenAiProvider::from_environment` - Loads `CODEX_OPENAI_API_KEY` or `OPENAI_API_KEY` and optional `CHUM_OPENAI_MODEL`.
- `ChumSwimProvider` implementation - Generates file specs, directory specs, and repair drafts.

## Dependencies / Contracts

- Uses `https://api.openai.com/v1/responses`.
- Default model is `gpt-4.1-mini` unless `CHUM_OPENAI_MODEL` is set.
- Missing credentials return an actionable error that mentions Codex login/API key fallback.
- The provider returns Markdown text only; backmatter normalization remains in `commands::swim`.

<!-- chum:backmatter
schema: 1
kind: file
target: src/provider/openai.rs
source_hash: sha256:3300e2370ceffb60c80a3b32f270e74a16320f47bcebb6c41ef60ba8c034f725
source_updated_at: 2026-04-24T01:32:14.348235332Z
spec_updated_at: 2026-04-24T01:35:55.620098Z
generated_by: chum swim --stubs
todo: []
unknowns: []
verify: []
-->
