# Phase 4: Swim Integration And Auth Status

## Goal

Route `chum swim --provider openai` through the auth resolver and expose
secret-free auth diagnostics.

## Scope

In scope:

- provider selection in `src/commands/swim.rs`
- `chum swim --auth-status`
- human and JSON auth status output
- CLI-level tests with fake Codex
- preservation of `--stubs`

Out of scope:

- app-server usage
- release packaging changes
- new non-OpenAI providers

## Implementation Notes

### Provider Selection

Current code directly constructs `OpenAiProvider::from_environment()`.
Replace that with:

1. Load config.
2. Apply CLI/env overrides.
3. Resolve auth for provider `openai`.
4. Build either `CodexExecProvider` or `OpenAiApiKeyProvider`.
5. Pass the resulting boxed provider into the existing generation flow.

Suggested shape:

```rust
fn build_openai_provider(
    root: &Utf8Path,
    config: &Config,
    args: &SwimArgs,
) -> anyhow::Result<Box<dyn ChumSwimProvider>>;
```

If the trait object needs `Send + Sync` later, make that change in this phase
only if the compiler or concurrency work requires it.

### `--stubs`

`chum swim --stubs` should not resolve or require auth. It should continue to
work offline.

### `--auth-status`

Add a boolean to `SwimArgs`:

```rust
#[arg(long)]
pub auth_status: bool,
```

Behavior:

- only valid with `--provider openai` for now
- compute auth resolution and print the status
- exit without source discovery or spec generation
- support `--json`
- do not fail just because auth is missing
- fail only if config parsing itself fails

### Human Output

Example for Codex:

```text
provider: openai
requested auth: auto
resolved auth: codex
codex: /usr/local/bin/codex
codex status: logged in using ChatGPT
codex api key: not needed
direct api key: not used
```

Example for missing auth:

```text
provider: openai
requested auth: auto
resolved auth: missing
codex: not found
direct api key: not found
guidance: Run codex login, run codex login --device-auth, set CODEX_API_KEY for codex exec, or set OPENAI_API_KEY for direct API fallback.
```

### JSON Output

Use stable camelCase fields:

```json
{
  "provider": "openai",
  "requestedAuthMode": "auto",
  "resolvedAuthMode": "codex",
  "codexBinary": "/usr/local/bin/codex",
  "codexStatus": "logged in using ChatGPT",
  "codexApiKeyPresent": false,
  "directApiKeyPresent": false,
  "directApiKeyEnv": null,
  "guidance": null
}
```

### CLI Tests

Add tests that put a fake `codex` binary first on `PATH`.

Fake cases:

- `codex login status` exits zero and prints `Logged in using ChatGPT`
- `codex login status` exits non-zero but `CODEX_API_KEY` exists
- `codex exec` writes valid JSON to the requested result path
- `codex exec` exits non-zero with secret-looking stderr
- no fake Codex and direct API env exists

## Acceptance Criteria

- [x] `--stubs` still runs without auth resolution.
- [x] Auto mode selects Codex provider with logged-in fake Codex.
- [x] Auto mode selects Codex provider with only `CODEX_API_KEY`.
- [x] Auto mode falls back to direct API with no Codex and direct API env.
- [x] Forced Codex mode returns missing-auth when Codex is unavailable.
- [x] Forced API-key mode does not execute fake Codex.
- [x] `--auth-status` human output is secret-free.
- [x] `--auth-status --json` is stable JSON.
- [x] Codex exec failure does not print full prompt or secrets.

## Dependencies

- Phase 1 auth resolver.
- Phase 2 direct API provider cleanup.
- Phase 3 Codex exec provider.
