# Design: Codex Auth Provider

> Superseded by the skill-first implementation in [`plan/skill/`](../plan/skill/).
> The final skill keeps the active agent session as the shared-context analyst
> and does not use `codex exec` as a per-file provider.

## Context

`chum swim` currently has a direct OpenAI Responses API provider that reads an
API key from the environment. The package design calls for a better default:
use the user's existing Codex login when available, with API-key fallback for
headless and CI use.

This document captures the Codex auth research as of April 24, 2026 and scopes
the provider changes needed to integrate it into `chum`.

## Research Findings

Official Codex docs describe two OpenAI sign-in methods:

- Sign in with ChatGPT for subscription access.
- Sign in with an API key for usage-based access.

The CLI and IDE extension support both methods, and Sign in with ChatGPT is the
CLI default when no valid session is available. The first CLI run prompts users
to authenticate with either a ChatGPT account or an API key.

Relevant sources:

- Codex CLI setup: https://developers.openai.com/codex/cli
- Codex authentication: https://developers.openai.com/codex/auth
- Codex non-interactive mode: https://developers.openai.com/codex/noninteractive
- Codex CLI command reference: https://developers.openai.com/codex/cli/reference
- Codex app-server auth surface: https://github.com/openai/codex/blob/main/codex-rs/app-server/README.md

Important details from the current docs:

- Codex caches login details locally and reuses them across CLI and extension
  starts.
- Cached credentials may live in either `CODEX_HOME/auth.json` or the
  OS-specific credential store, depending on `cli_auth_credentials_store`.
- File-based `auth.json` contains access tokens and must be treated like a
  password.
- ChatGPT sign-in sessions are refreshed automatically by Codex during use.
- `codex login --device-auth` supports headless environments when browser
  callback login is not practical.
- `codex exec` reuses saved CLI authentication by default.
- `CODEX_API_KEY` is documented for `codex exec` automation and is only
  supported by that command.
- API keys remain the recommended default for CI, while ChatGPT-managed auth in
  CI is advanced and requires protecting persisted Codex credentials.
- `codex exec` is stable and supports stdin prompts, read-only sandboxing,
  `--ephemeral`, `--output-last-message`, `--output-schema`, and
  `--skip-git-repo-check`.
- The local `codex app-server` command exists and has account auth methods, but
  the command reference marks app-server as experimental.

Local CLI inspection matched the docs. On this machine, `codex-cli 0.124.0`
reports `Logged in using ChatGPT`, `codex login` supports `--with-api-key` and
`--device-auth`, and `codex exec` exposes the automation flags listed above.

## Goals

- Let default `chum swim` use an existing Codex login without asking the user
  for an API key.
- Keep `chum` from reading, parsing, storing, or printing Codex access tokens.
- Preserve a direct API-key fallback for users without the Codex CLI, users who
  prefer explicit API billing, and CI jobs.
- Keep `chum` usable in non-Git directories.
- Keep the provider interface testable without real OpenAI or Codex credentials.

## Non-Goals

- Do not implement a standalone ChatGPT OAuth client in `chum`.
- Do not read `CODEX_HOME/auth.json` directly.
- Do not call OS keyrings directly for Codex credentials.
- Do not depend on the experimental app-server auth API for v1.
- Do not require a Git repository for provider operation.

## Decision

Use a Codex subprocess provider as the default OpenAI-backed `swim` path.

`chum` should treat Codex CLI as the owner of ChatGPT login, token refresh,
credential storage, API-key login, and device-code login. For spec generation,
`chum` invokes `codex exec` with a narrow prompt and machine-readable output
schema. This gives `chum` access to the user's existing Codex auth while keeping
secret handling inside Codex.

The direct Responses API provider remains as a fallback when:

- the user explicitly selects direct API auth,
- no usable Codex binary is found,
- Codex is not logged in and no `CODEX_API_KEY` is available for `codex exec`,
- or the user runs in an environment where subprocess Codex is not allowed.

## Provider Model

Keep the public provider name `openai`, but split implementation by auth mode:

```yaml
swim:
  provider: openai
  openai:
    auth: auto # auto | codex | apiKey
    codexBinary: codex
    model: null
    reasoningEffort: null
```

`auth: auto` resolution:

1. If `CHUM_OPENAI_AUTH=apiKey` or config says `apiKey`, use the direct API-key
   provider.
2. If config says `codex`, require the Codex subprocess provider and fail with
   setup guidance if unavailable.
3. Otherwise prefer the Codex subprocess provider when a `codex` binary exists
   and either `codex login status` succeeds or `CODEX_API_KEY` is set.
4. Fall back to the direct Responses API provider when `CHUM_OPENAI_API_KEY`,
   `CODEX_OPENAI_API_KEY`, or `OPENAI_API_KEY` is set.
5. If no auth source works, print a concise setup message:
   `Run codex login, run codex login --device-auth, set CODEX_API_KEY for codex exec, or set OPENAI_API_KEY for direct API fallback.`

`CODEX_API_KEY` should be passed through only to `codex exec`, matching Codex's
documented automation behavior. Direct Responses API fallback should continue
using normal OpenAI API-key env vars, with `CHUM_OPENAI_API_KEY` preferred over
legacy `CODEX_OPENAI_API_KEY` and `OPENAI_API_KEY`.

## CodexExecProvider

Add `src/provider/codex.rs` with a `CodexExecProvider` that implements the
existing `ChumSwimProvider` trait.

For each provider request:

1. Build the same logical prompt currently sent to the direct OpenAI provider.
2. Write a temporary JSON Schema requiring a single string field:

   ```json
   {
     "type": "object",
     "properties": {
       "markdown": { "type": "string" }
     },
     "required": ["markdown"],
     "additionalProperties": false
   }
   ```

3. Invoke Codex with stdin input instead of putting source text in argv:

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

4. Pass `--model <model>` when config or `CHUM_CODEX_MODEL` provides one.
5. Pass reasoning config via `-c model_reasoning_effort="<value>"` only after
   confirming the current Codex config key during implementation.
6. Parse `<result.json>` and return `markdown`.
7. On failure, include exit status and a short stderr tail, but never echo
   environment values, auth cache paths with contents, or full source prompts.

The provider should not use `--full-auto` or any write-enabled sandbox mode.
`chum` writes the generated specs itself after validating and normalizing
backmatter. Codex only generates text.

## Environment Handling

By default, the Codex subprocess inherits the user's environment so existing
Codex settings continue to work. Two exceptions are worth implementing:

- If `auth: codex` is selected and `CHUM_CODEX_STRICT_CHATGPT=1`, remove
  `OPENAI_API_KEY`, `CODEX_OPENAI_API_KEY`, and `CHUM_OPENAI_API_KEY` from the
  child environment so a shell-level API key cannot accidentally change the
  effective auth path.
- Always redact env var names that imply secrets from error dumps. It is fine
  to say that an env var was detected, but never print its value.

## Direct ApiKeyProvider

Keep the current direct provider, but rename it internally to make the boundary
clear:

- `OpenAiProvider` becomes `OpenAiApiKeyProvider`.
- `from_environment()` checks `CHUM_OPENAI_API_KEY`,
  `CODEX_OPENAI_API_KEY`, then `OPENAI_API_KEY`.
- Error text should identify that this path is API-key-only and suggest
  `swim.openai.auth: codex` or `codex login` for ChatGPT-managed auth.

This provider continues to call the Responses API directly and is the right
fallback for users who do not install Codex.

## Why Not Parse Codex Credentials

Codex docs intentionally describe credential storage as an implementation
detail controlled by `cli_auth_credentials_store`. The same login can be stored
in a plaintext file or an OS credential store, and ChatGPT sessions are
refreshed automatically by Codex. Reading `auth.json` would only cover one
storage mode, would require handling token refresh, and would make `chum`
responsible for secrets that Codex already owns.

The integration contract should therefore be process-level, not token-level:
`chum` asks Codex to run a read-only non-interactive text generation task, and
Codex decides how to authenticate it.

## Future App-Server Path

The Codex app-server README exposes `account/read`, `account/login/start`,
`account/logout`, and auth update notifications. It also distinguishes API key
and ChatGPT-managed auth modes.

Do not use this in v1 because the CLI reference marks app-server as
experimental. Track it as a future improvement for richer status reporting or a
long-lived provider that avoids one `codex exec` process per spec.

## CLI UX

Add these diagnostics:

```bash
chum swim --auth-status
```

Output should be secret-free:

```text
provider: openai
auth mode: codex
codex: /path/to/codex
codex status: logged in using ChatGPT
direct api env: not used
```

Provider errors should be actionable:

- `codex executable not found`
- `codex login status failed`
- `codex exec failed with exit code <n>`
- `direct OpenAI API key not found`
- `structured Codex output did not include markdown`

## Implementation Phases

### Phase 1: Auth Resolver

- Add `OpenAiAuthMode` config and env override parsing.
- Add `AuthResolution` enum with `CodexExec`, `DirectApiKey`, and `Missing`.
- Add unit tests for resolution order and error messages.

### Phase 2: Codex Exec Provider

- Add `CodexExecProvider`.
- Add a small command-runner trait so tests can fake `codex exec`.
- Add schema/result parsing tests.
- Add failure redaction tests.

### Phase 3: Swim Integration

- Route `swim.provider = openai` through the resolver.
- Keep existing direct OpenAI behavior as explicit/fallback behavior.
- Add CLI tests with a fake `codex` binary on `PATH`.

### Phase 4: Docs And Config

- Update `README.md`, generated default config, and package design docs.
- Add a troubleshooting section for `codex login`, `codex login --device-auth`,
  `CODEX_API_KEY`, and direct `OPENAI_API_KEY`.

## Acceptance Criteria

- `chum swim --provider openai` uses Codex auth by default when a logged-in
  Codex CLI is available.
- `chum` never reads `CODEX_HOME/auth.json`.
- `chum` never prints API keys, access tokens, refresh tokens, or auth cache
  contents.
- `chum swim` still works with only `OPENAI_API_KEY`.
- `chum swim` still works outside Git repositories.
- Tests cover auth resolution, fake Codex exec success, fake Codex exec failure,
  direct API fallback, and missing-auth guidance.
