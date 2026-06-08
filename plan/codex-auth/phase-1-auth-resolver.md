# Phase 1: Auth Resolver

## Goal

Add the config, environment parsing, and auth resolution layer that decides
whether OpenAI-backed `swim` should use Codex exec, direct API-key mode, or a
missing-auth diagnostic.

## Scope

In scope:

- nested `swim.openai` config
- `OpenAiAuthMode`
- env override parsing
- Codex binary lookup abstraction
- `codex login status` status abstraction
- direct API-key presence detection
- secret-free missing-auth report
- unit tests for resolution order

Out of scope:

- invoking `codex exec`
- changing the provider generation path
- mocked HTTP for direct API calls
- documentation updates outside the plan

## Implementation Notes

### Config Types

Add config structs equivalent to:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenAiSwimConfig {
    pub auth: OpenAiAuthMode,
    pub codex_binary: String,
    pub model: Option<String>,
    pub reasoning_effort: Option<String>,
}
```

Add `openai: OpenAiSwimConfig` to `SwimConfig`.

Default values:

- `auth = OpenAiAuthMode::Auto`
- `codex_binary = "codex"`
- `model = None`
- `reasoning_effort = None`

Partial config loading must merge nested `openai` fields without requiring the
whole block to be present.

### Auth Mode Parsing

Support:

- `auto`
- `codex`
- `apiKey`
- `api-key`

Use one canonical in-memory enum. Error messages should include invalid values.

### Resolver Inputs

Use a small injectable environment and command-status boundary:

```rust
pub trait EnvLookup {
    fn var(&self, name: &str) -> Option<String>;
}

pub trait CodexStatusProbe {
    fn find_binary(&self, configured: &str) -> Option<PathBuf>;
    fn login_status(&self, binary: &Path) -> CodexLoginStatus;
}
```

The concrete implementation can use `which`-style path lookup and
`codex login status`. Tests should use fakes.

### Resolution Output

The result should include a human-readable status object in addition to the
provider selection. `chum swim --auth-status` will use the same object later.

Fields to track:

- requested mode
- resolved mode
- Codex binary path, if found
- Codex login status summary
- whether `CODEX_API_KEY` is present
- whether a direct API-key env var is present
- direct API-key env var name, not value
- guidance

## Acceptance Criteria

- [x] `swim.openai` defaults are present in `Config::default()`.
- [x] Partial YAML can set only `swim.openai.auth`.
- [x] `CHUM_OPENAI_AUTH` overrides YAML auth mode.
- [x] `CHUM_CODEX_BINARY` overrides YAML Codex binary.
- [x] Auto mode chooses Codex when login status succeeds.
- [x] Auto mode chooses Codex when only `CODEX_API_KEY` exists.
- [x] Auto mode chooses direct API when Codex is unavailable and direct API key
      exists.
- [x] Forced Codex mode never selects direct API.
- [x] Forced API-key mode never probes Codex exec for generation.
- [x] Missing auth report includes guidance and no secret values.

## Dependencies

- Existing config loader.
- Existing provider module.
