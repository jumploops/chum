# `src/provider/openai_auth.rs`

## Purpose

Resolves OpenAI-backed swim authentication into Codex exec, direct API-key
mode, or a missing-auth diagnostic. This module is the only place that decides
provider auth precedence.

## Key Exports

- `EnvLookup`, `SystemEnv`, `CodexStatusProbe`, and
  `SystemCodexStatusProbe` isolate environment and Codex CLI probing for tests.
- `CodexLoginStatus`, `CodexExecConfig`, and `OpenAiApiKeyConfig` carry
  provider-specific configuration selected by the resolver.
- `OpenAiAuthStatus` is the secret-free status object used by
  `chum swim --auth-status`.
- `OpenAiAuthResolution` selects Codex exec, direct API-key mode, or missing
  auth.
- `resolve_openai_auth`, `detect_direct_api_key`, and `missing_auth_error`
  implement resolution and user-facing guidance.

## Dependencies / Contracts

- Auto mode prefers a usable Codex binary with either successful
  `codex login status` or `CODEX_API_KEY`, then falls back to direct API keys.
- Forced Codex mode never falls back to direct API keys.
- Forced API-key mode never probes Codex for generation.
- Direct API-key precedence is `CHUM_OPENAI_API_KEY`,
  `CODEX_OPENAI_API_KEY`, then `OPENAI_API_KEY`.
- Status objects may include env var names but must never include secret values.
- The resolver does not read Codex credential files or OS keyrings.

<!-- chum:backmatter
schema: 1
kind: file
target: src/provider/openai_auth.rs
source_hash: sha256:89335ba86c1ce6964dff19632b8065d44ddc7934b457ad60985e09f7990c3f84
source_updated_at: 2026-04-24T02:57:24.403705063Z
spec_updated_at: 2026-04-24T02:57:35.366829Z
generated_by: chum swim --stubs
todo: []
unknowns: []
verify: []
-->
