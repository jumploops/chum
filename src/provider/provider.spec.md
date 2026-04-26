# `src/provider/`

## Purpose

Provider boundary for AI-backed `chum swim`. The core swim traversal owns filesystem state and validation; providers only turn bounded inputs into Markdown spec drafts.

## Files

- `mod.rs` - Defines provider input/output structs and the `ChumSwimProvider` trait.
- `openai.rs` - Builds shared OpenAI/Codex prompts.
- `openai_auth.rs` - Resolves Codex exec vs direct API-key auth.
- `codex.rs` - Implements the Codex subprocess provider.
- `openai_api.rs` - Implements the direct Responses API provider.

## Subfolders


## Dependencies / Contracts

- Provider implementations must return Markdown; core swim code is responsible for adding or normalizing backmatter before writing.
- The OpenAI adapter prefers Codex auth through `codex exec` and falls back to direct API keys.
- Provider code must not read Codex credential files or print secret values.
- Additional providers should implement the trait without changing traversal or validation logic.

<!-- chum:backmatter
schema: 1
kind: directory
target: src/provider
spec_updated_at: 2026-04-24T01:35:55.620918Z
generated_by: chum swim --stubs
children:
- src/provider/codex.rs.spec.md
- src/provider/mod.rs.spec.md
- src/provider/openai_api.rs.spec.md
- src/provider/openai_auth.rs.spec.md
- src/provider/openai.rs.spec.md
todo: []
unknowns: []
verify: []
-->
