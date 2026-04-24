# Validation Checklist: Codex Auth

## Local Rust Validation

- [ ] `cargo fmt --check`
- [ ] `cargo clippy --all-targets --all-features -- -D warnings`
- [ ] `cargo test`
- [ ] `cargo run -- check --json`
- [ ] `cargo run -- swim --stubs --dry-run`
- [ ] `cargo run -- swim --auth-status`
- [ ] `cargo run -- swim --auth-status --json`

## Auth Resolver Validation

- [ ] Default config resolves requested mode as `auto`
- [ ] YAML `swim.openai.auth: codex` is parsed
- [ ] YAML `swim.openai.auth: apiKey` is parsed
- [ ] Env `CHUM_OPENAI_AUTH=api-key` is parsed
- [ ] Invalid auth mode fails clearly
- [ ] Auto mode selects Codex with successful login status
- [ ] Auto mode selects Codex with `CODEX_API_KEY`
- [ ] Auto mode selects direct API with no Codex and direct key present
- [ ] Forced Codex mode does not fall back
- [ ] Forced API-key mode does not probe Codex for generation
- [ ] Missing auth report contains setup guidance
- [ ] Missing auth report contains no secret values

## Codex Exec Provider Validation

- [ ] Command uses stdin for prompt text
- [ ] Command does not put source text in argv
- [ ] Command includes `--ephemeral`
- [ ] Command includes `--skip-git-repo-check`
- [ ] Command includes `--sandbox read-only`
- [ ] Command includes `--ask-for-approval never`
- [ ] Command includes `--output-schema`
- [ ] Command includes `--output-last-message`
- [ ] Command includes `-C <repo-root>`
- [ ] Configured model is passed as `--model`
- [ ] Valid JSON result returns Markdown
- [ ] Invalid JSON result fails clearly
- [ ] Missing `markdown` field fails clearly
- [ ] Command stderr is redacted on failure
- [ ] Strict ChatGPT mode removes direct API-key env vars

## Direct API Provider Validation

- [ ] `CHUM_OPENAI_API_KEY` wins over `CODEX_OPENAI_API_KEY`
- [ ] `CODEX_OPENAI_API_KEY` wins over `OPENAI_API_KEY`
- [ ] `OPENAI_API_KEY` still works
- [ ] Missing direct API key error mentions direct mode
- [ ] API request failures do not include auth headers
- [ ] Response parsing still handles `output_text`
- [ ] Response parsing still handles nested `output[].content[].text`

## CLI Validation

- [ ] `chum swim --stubs --dry-run` does not require auth
- [ ] `chum swim --auth-status` exits zero with missing auth
- [ ] `chum swim --auth-status --json` emits valid JSON
- [ ] Fake logged-in Codex is selected in auto mode
- [ ] Fake Codex with `CODEX_API_KEY` is selected in auto mode
- [ ] Direct API fallback is selected when fake Codex is absent
- [ ] Forced Codex mode reports missing Codex when absent
- [ ] Forced API-key mode skips fake Codex
- [ ] Codex exec failure is reported without prompt text
- [ ] Non-Git fixture works through Codex exec command construction

## Documentation Validation

- [ ] README documents Codex login
- [ ] README documents device auth
- [ ] README documents `CODEX_API_KEY`
- [ ] README documents direct API fallback
- [ ] README documents `chum swim --auth-status`
- [ ] Docs do not instruct users to read `auth.json`
- [ ] Live specs are current for every touched source file
