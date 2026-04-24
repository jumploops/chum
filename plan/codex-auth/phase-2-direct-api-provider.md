# Phase 2: Direct API Provider Cleanup

## Goal

Make the current direct Responses API provider an explicit API-key transport so
it can coexist cleanly with Codex exec.

## Scope

In scope:

- rename/internal split from generic `OpenAiProvider` to API-key-specific naming
- direct API-key env precedence
- shared prompt construction extraction
- direct provider missing-auth error cleanup
- tests for env precedence and response parsing

Out of scope:

- Codex exec implementation
- changing `ChumSwimProvider`
- changing swim traversal behavior

## Implementation Notes

### Provider Naming

Use one of these approaches:

- preferred: move direct API code to `src/provider/openai_api.rs` as
  `OpenAiApiKeyProvider`
- acceptable interim: keep file path `openai.rs` but rename the struct and
  separate resolver code into `openai_auth.rs`

The code should make it obvious when a path uses direct API keys instead of
Codex-managed auth.

### Direct API-Key Precedence

Check:

1. `CHUM_OPENAI_API_KEY`
2. `CODEX_OPENAI_API_KEY`
3. `OPENAI_API_KEY`

Return the env var name used for diagnostics, but never expose the value.

### Shared Prompt Builder

Extract shared prompt text so both direct API and Codex exec providers generate
equivalent requests:

- system instruction
- file spec instruction
- directory spec instruction
- repair instruction
- context formatting for source files and child specs

The generated prompt must still ask for current-state specs and complete chum
backmatter.

### Direct Provider Errors

Missing direct API key should say that direct API-key auth is unavailable and
point to Codex auth when appropriate:

```text
Direct OpenAI API key not found. Set CHUM_OPENAI_API_KEY or OPENAI_API_KEY, or use Codex auth with `codex login`.
```

Network/API errors should retain current context but must not include request
bodies or auth headers.

## Acceptance Criteria

- [ ] Direct provider uses `CHUM_OPENAI_API_KEY` before legacy env vars.
- [ ] Error text distinguishes direct API-key auth from Codex auth.
- [ ] No API key value appears in error output.
- [ ] Shared prompt builder is used by the direct provider.
- [ ] Existing fake-provider swim tests still pass.
- [ ] Direct response parsing behavior remains covered.

## Dependencies

- Phase 1 auth config types may exist, but this cleanup can be implemented
  independently if needed.
