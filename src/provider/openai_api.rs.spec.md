# `src/provider/openai_api.rs`

## Purpose

Implements the direct OpenAI Responses API provider for users and CI jobs that
authenticate with an API key instead of Codex-managed login.

## Key Exports

- `OpenAiApiKeyProvider` implements `ChumSwimProvider` using a bearer API key
  and model from `OpenAiApiKeyConfig`.
- `extract_response_text` extracts Markdown from either top-level
  `output_text` or nested Responses API output content.

## Dependencies / Contracts

- Auth resolution supplies the API key; this provider does not inspect Codex
  credentials.
- Requests use the shared prompt builders from `src/provider/openai.rs`.
- Errors include API/request context but never request bodies or auth headers.
- Response parsing accepts both supported text shapes and fails closed when no
  Markdown text is present.

<!-- chum:backmatter
schema: 1
kind: file
target: src/provider/openai_api.rs
source_hash: sha256:e30cf8a755e1b192b140d56cad1cd9de7ecf094f3fb466d62eaefa422e985f7f
source_updated_at: 2026-04-24T02:47:50.149587357Z
spec_updated_at: 2026-04-24T02:54:29.284916Z
generated_by: chum swim --stubs
todo: []
unknowns: []
verify: []
-->
