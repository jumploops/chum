# Implementation Spec: Codex Auth

## Context

- Related design doc: [`design/codex-auth-provider.md`](../../design/codex-auth-provider.md)
- Existing provider code: [`src/provider/openai.rs`](../../src/provider/openai.rs)
- Swim command entry point: [`src/commands/swim.rs`](../../src/commands/swim.rs)

`chum swim` currently uses a direct OpenAI Responses API provider that requires
an API key in the environment. The next provider milestone is to make an
existing Codex login the default auth path while preserving direct API-key
fallback.

The implementation must treat Codex as the owner of ChatGPT login, token
refresh, credential storage, device auth, and `CODEX_API_KEY` handling. `chum`
must not read Codex credential files or OS keyrings directly.

## Objective

Make `chum swim --provider openai` resolve auth in this order:

1. Use a logged-in Codex CLI through `codex exec`.
2. Use `CODEX_API_KEY` through `codex exec` when Codex is available but not
   logged in.
3. Fall back to the direct Responses API provider with `CHUM_OPENAI_API_KEY`,
   `CODEX_OPENAI_API_KEY`, or `OPENAI_API_KEY`.
4. Fail with concise setup guidance when no supported auth path is available.

End-state:

- `chum swim` can generate specs using the user's Codex ChatGPT login.
- Direct API-key behavior still works for CI and non-Codex users.
- Missing auth errors are actionable and secret-free.
- Auth resolution and Codex execution are covered by tests using fakes.
- `chum` never reads or prints Codex auth cache contents.

## Current State

Relevant current implementation details:

- `SwimArgs.provider` defaults to `openai`.
- `write_provider_specs` rejects any provider name other than `openai`.
- `OpenAiProvider::from_environment()` reads `CODEX_OPENAI_API_KEY`, then
  `OPENAI_API_KEY`.
- `OpenAiProvider` calls `https://api.openai.com/v1/responses` directly.
- Provider calls are synchronous through the `ChumSwimProvider` trait.
- Existing tests already use fake providers for swim convergence behavior.

The Codex auth implementation should preserve the synchronous provider trait for
now. Do not introduce async runtime changes just for this work.

## Target Module Layout

Keep the provider code inside `src/provider/`:

```text
src/provider/
+-- mod.rs
+-- openai.rs        # OpenAI provider selection and shared prompts
+-- openai_auth.rs   # auth mode parsing, resolution, status
+-- openai_api.rs    # direct Responses API-key provider
+-- codex.rs         # Codex exec provider
```

If a smaller patch is clearer, `openai.rs` may continue to hold the direct API
provider during Phase 1, but the final boundary should separate:

- auth resolution
- direct API transport
- Codex subprocess transport
- shared prompt/schema construction

## Config Contract

Extend `swim` config with an optional nested `openai` block:

```yaml
swim:
  provider: openai
  openai:
    auth: auto # auto | codex | apiKey
    codexBinary: codex
    model: null
    reasoningEffort: null
```

Defaults:

- `auth: auto`
- `codexBinary: codex`
- `model: null`, which means use Codex's configured/default model for Codex
  exec and `CHUM_OPENAI_MODEL` or the direct provider default for API-key mode
- `reasoningEffort: null`

Serde requirements:

- Accept `apiKey` in YAML.
- Accept env spelling `api-key` and `apiKey`.
- Unknown auth values should produce a clear config error naming the invalid
  value.
- Missing nested `openai` config should preserve current default behavior except
  that Codex is now preferred before direct API keys.

## Environment Contract

Environment overrides:

| Variable | Applies To | Meaning |
| --- | --- | --- |
| `CHUM_OPENAI_AUTH` | resolver | `auto`, `codex`, `apiKey`, or `api-key` |
| `CHUM_CODEX_BINARY` | Codex exec | Codex binary path/name |
| `CHUM_CODEX_MODEL` | Codex exec | model passed as `--model` |
| `CHUM_CODEX_REASONING_EFFORT` | Codex exec | reasoning effort, if supported by current Codex config key |
| `CHUM_CODEX_STRICT_CHATGPT` | Codex exec | remove direct API-key env vars from child process when `1` |
| `CODEX_API_KEY` | Codex exec | documented Codex automation key |
| `CHUM_OPENAI_API_KEY` | direct API | preferred direct Responses API key |
| `CODEX_OPENAI_API_KEY` | direct API | backward-compatible key |
| `OPENAI_API_KEY` | direct API | standard OpenAI API key |
| `CHUM_OPENAI_MODEL` | direct API | direct Responses API model |

Resolution rules:

1. Config is loaded first.
2. Environment overrides config for auth mode, Codex binary, Codex model, Codex
   reasoning effort, and direct API model.
3. CLI flags may override both only where added explicitly.

## Auth Resolution Contract

Add types equivalent to:

```rust
pub enum OpenAiAuthMode {
    Auto,
    Codex,
    ApiKey,
}

pub enum OpenAiAuthResolution {
    CodexExec(CodexExecConfig),
    DirectApiKey(OpenAiApiKeyConfig),
    Missing(MissingAuthReport),
}
```

`OpenAiAuthResolution::Missing` should carry enough detail for diagnostics:

- requested mode
- whether a Codex binary was found
- whether `codex login status` succeeded
- whether `CODEX_API_KEY` was present
- whether any direct API-key env var was present
- setup guidance

Do not include secret values.

Auto mode:

1. If Codex binary lookup succeeds and either `codex login status` succeeds or
   `CODEX_API_KEY` exists, select `CodexExec`.
2. Otherwise, if a direct API key exists, select `DirectApiKey`.
3. Otherwise return `Missing`.

Forced Codex mode:

- Require Codex binary lookup to succeed.
- Select `CodexExec` when `codex login status` succeeds or `CODEX_API_KEY`
  exists.
- Return `Missing` without falling back to direct API keys.

Forced API-key mode:

- Require a direct API key.
- Return `Missing` without trying Codex.

## Codex Exec Contract

`CodexExecProvider` implements `ChumSwimProvider` by invoking:

```bash
codex exec \
  --ephemeral \
  --skip-git-repo-check \
  --sandbox read-only \
  --ask-for-approval never \
  --output-schema <schema.json> \
  --output-last-message <result.json> \
  -C <repo-root> \
  -
```

Additional arguments:

- `--model <model>` when config or env provides one.
- `-c model_reasoning_effort="<value>"` only after confirming the key works
  against the installed Codex CLI during implementation.

Input and output:

- Write the complete provider prompt to stdin.
- Write the output schema to a temporary file.
- Read the final message from the output file.
- Parse a JSON object with a required `markdown` string.
- Return `SpecDraft { markdown }`.

Do not pass source text through argv.

## Structured Output Schema

Codex exec should be constrained to this shape:

```json
{
  "type": "object",
  "properties": {
    "markdown": {
      "type": "string"
    }
  },
  "required": ["markdown"],
  "additionalProperties": false
}
```

If Codex returns plain Markdown despite the schema, the first implementation may
optionally accept it as a fallback only when the result file is not valid JSON
and the text contains a valid chum backmatter block after normal swim
post-processing.

## Security Requirements

- Never read `CODEX_HOME/auth.json`.
- Never call OS keyrings for Codex credentials.
- Never print API keys, access tokens, refresh tokens, or auth file contents.
- Never include full provider prompts in error output.
- Redact secret-looking env values from command diagnostics.
- Keep Codex in `read-only` sandbox mode.
- Use `--ephemeral` so provider runs do not create persistent Codex sessions for
  each generated spec.
- `chum` remains the only writer of generated spec files.

## CLI UX

Add:

```bash
chum swim --auth-status
```

Behavior:

- Print auth resolution status and exit without generating specs.
- Support `--json`.
- Never require a successful provider call.
- Exit zero when status can be computed, even if auth is missing.

Human output example:

```text
provider: openai
auth mode: codex
codex: /path/to/codex
codex status: logged in using ChatGPT
direct api env: not used
```

JSON output should include:

- `provider`
- `requestedAuthMode`
- `resolvedAuthMode`
- `codexBinary`
- `codexStatus`
- `codeXApiKeyPresent`
- `directApiKeyPresent`
- `guidance`

Use a stable field name such as `codexApiKeyPresent`; do not leak the exact env
var value.

## Error UX

Provider errors should distinguish:

- Codex binary not found.
- Codex auth missing.
- Codex command failed.
- Codex output schema parse failed.
- Direct OpenAI API key missing.
- Direct OpenAI API request failed.

Every auth-missing error should include this guidance:

```text
Run codex login, run codex login --device-auth, set CODEX_API_KEY for codex exec, or set OPENAI_API_KEY for direct API fallback.
```

When forced auth mode is used, guidance should mention the forced mode and not
suggest an ignored fallback as if it were active.

## Testing Strategy

Tests must not require real OpenAI or Codex credentials.

Use:

- unit tests for env/config auth resolution
- fake command runner for `codex login status` and `codex exec`
- temporary fake `codex` binary on `PATH` for CLI-level tests
- mocked HTTP or transport abstraction for direct API-key provider tests
- redaction tests for stderr/error formatting

Test both Git and non-Git directories where relevant. Codex exec must include
`--skip-git-repo-check` in command construction.

## Phase Breakdown

- Phase 1: config and auth resolver.
- Phase 2: direct API provider cleanup.
- Phase 3: Codex exec provider.
- Phase 4: swim integration and auth status UX.
- Phase 5: docs, validation, and rollout cleanup.

## Acceptance Criteria

- [ ] `chum swim --provider openai` uses Codex auth by default when Codex is
      available and authenticated.
- [ ] `CODEX_API_KEY` can drive `codex exec` without direct Responses API use.
- [ ] `OPENAI_API_KEY` fallback still works when Codex is unavailable.
- [ ] Forced Codex mode does not fall back to direct API keys.
- [ ] Forced API-key mode does not invoke Codex.
- [ ] `chum swim --auth-status` reports secret-free status in human and JSON
      forms.
- [ ] `chum` never reads or parses Codex credential files.
- [ ] Tests cover auth resolution, fake Codex exec success, fake Codex exec
      failure, direct API fallback, missing auth, and redaction.
- [ ] Existing `cargo fmt`, `cargo clippy`, `cargo test`, and `chum check`
      validation still pass.
