# Phase 5: Docs And Validation

## Goal

Update user-facing docs and finish validation for the Codex auth provider work.

## Scope

In scope:

- README provider/auth docs
- default config docs
- design doc cross-reference updates, if needed
- live specs for changed provider/config/CLI files
- validation checklist completion

Out of scope:

- Homebrew packaging
- app-server provider
- release publishing

## Implementation Notes

### README

Document:

- default `chum swim` auth behavior
- `codex login`
- `codex login --device-auth`
- `CODEX_API_KEY` for Codex exec automation
- direct API-key fallback with `CHUM_OPENAI_API_KEY` or `OPENAI_API_KEY`
- `chum swim --auth-status`
- how to force Codex or API-key mode in config

Keep examples short and avoid implying that users should inspect credential
files.

### Default Config

Ensure `Config::default_yaml()` shows the new `swim.openai` block. If the
default YAML becomes too noisy, keep defaults in code and document the optional
override block in README.

### Live Specs

Update or create specs for changed files:

- `src/config.rs.spec.md`
- `src/cli.rs.spec.md`
- `src/commands/swim.rs.spec.md`
- `src/provider/provider.spec.md`
- `src/provider/openai.rs.spec.md`
- any new provider module specs

Every touched source file should have current source hash backmatter and empty
todo, unknown, and verify lists.

### Validation

Run at minimum:

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
cargo run -- check --json
cargo run -- swim --stubs --dry-run
cargo run -- swim --auth-status
cargo run -- swim --auth-status --json
```

When possible, also run CLI tests with fake Codex and direct API-key env
fixtures.

## Acceptance Criteria

- [x] README explains Codex auth and direct API fallback.
- [x] Default config or docs show `swim.openai.auth`.
- [x] Live specs are current for every touched source file.
- [x] Validation checklist is updated.
- [x] No docs suggest reading Codex credential files.
- [x] All required local validation commands pass.

## Dependencies

- Phase 4 swim integration.
